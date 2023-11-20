use std::vec;

use crate::economy::processor::Processor;
use crate::economy::recipe::RecipeHandle;
use crate::economy::resource::ResourceHandle;
use crate::economy::stock::Stock;
use crate::market::offer::UnprocessedOffer;
use crate::market::order::UnprocessedOrder;
use crate::reinforcement_learning::action_space::ActionSpace;
use crate::reinforcement_learning::action_space::CompanyAction;
use crate::reinforcement_learning::deep_rl_agent::DeepRLAgent;
use crate::reinforcement_learning::state::CompanyState;
use crate::world_data::market_data::MarketData;
use crate::world_data::recipe_data::RecipeData;
use serde::{Deserialize, Serialize};

use super::processor;
pub type CompanyHandle = usize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Company {
    pub name: String,
    pub stock: Stock,
    pub currency: f64,
    pub processors: Vec<Processor>,
    pub orders: Vec<UnprocessedOrder>,
    pub offers: Vec<UnprocessedOffer>,
    pub company_value: f64,
    pub id: CompanyHandle,
    pub agent: DeepRLAgent,
    pub old_state: CompanyState,
    pub old_company_value: f64,
    pub productive_processor_ticks: usize,
}

impl Company {
    pub fn new(
        name: &str,
        company_handle: CompanyHandle,
        resource_count: usize,
        recipe_count: usize,
        state_dimensions: i32,
        action_dimensions: i32,
        discount: f64,
        experience_replay_batch_size: usize,
        experience_buffer_length: usize,
        q_update_ticks: usize,
    ) -> Self {
        Company {
            name: name.to_string(),
            stock: Stock::new(),
            currency: 0.0,
            processors: vec![],
            orders: vec![],
            offers: vec![],
            company_value: 0.0,
            id: company_handle,
            agent: DeepRLAgent::new(
                state_dimensions,
                action_dimensions,
                discount,
                experience_replay_batch_size,
                experience_buffer_length,
                q_update_ticks,
            ),
            old_state: CompanyState::new(resource_count, recipe_count),
            old_company_value: 0.0,
            productive_processor_ticks: 0,
        }
    }

    pub fn tick(
        &mut self,
        recipe_data: &RecipeData,
        market_data: &MarketData,
        processor_price: f64,
        actionspace: &ActionSpace,
        train: bool,
        exploration_factor: f64,
        ticks: usize,
    ) {
        let mut production_rates: Vec<usize> = vec![0; self.old_state.production_rates.len()];
        // Construct company state
        let mut processor_counts = self.old_state.processor_counts.clone();
        for recipe in 0..processor_counts.len() {
            processor_counts[recipe] = 0;
        }
        for processor in self.processors.iter_mut() {
            processor_counts[processor.recipe] += 1;
            processor.tick(&mut self.stock, recipe_data);
            if processor.produced_last_tick {
                self.productive_processor_ticks += 1;
                let recipe = recipe_data.get_recipe_by_handle(processor.recipe).unwrap();
                for (resource, amount) in recipe.products.iter() {
                    let total = (amount * processor.production_speed) as usize
                        + production_rates.get(*resource).unwrap();
                    production_rates[*resource] = total;
                }
            }
        }
        let company_state = CompanyState {
            stock: self.stock.resources.values().map(|x| *x as usize).collect(),
            currency: self.currency as usize,
            price_index: market_data
                .price_index
                .values()
                .map(|x: &Option<(usize, f64)>| {
                    if x.is_some() {
                        x.unwrap().1 as usize
                    } else {
                        0
                    }
                })
                .collect(),
            order_index: market_data
                .order_index
                .values()
                .map(|x: &Option<(usize, f64)>| {
                    if x.is_some() {
                        x.unwrap().1 as usize
                    } else {
                        0
                    }
                })
                .collect(),
            processor_counts: processor_counts,
            production_rates: production_rates,
        };

        self.old_company_value = self.company_value;
        self.company_value = self.calculate_company_value(market_data, processor_price);
        let reward = self.company_value - self.old_company_value - 1.0;
        if train {
            self.agent.train(
                self.old_state.as_f64_vec(),
                reward,
                company_state.as_f64_vec(),
                ticks,
            );
        }
        let action = self
            .agent
            .get_next_state_action(company_state.as_f64_vec(), exploration_factor);
        self.old_state = company_state;
        // Act according to agent decision
        match actionspace.actions[action] {
            CompanyAction::Nothing => {
                // do nothing
            }
            CompanyAction::BuyProcessor(recipe) => {
                if recipe_data.recipes.len() <= recipe {
                    return;
                }
                self.buy_processor(recipe, processor_price, &recipe_data);
            }
            CompanyAction::SellProcessor(recipe) => {
                // Search for processor with given recipe
                for (processor_handle, processor) in self.processors.iter().enumerate() {
                    if processor.recipe == recipe {
                        self.sell_processor(processor_handle, processor_price);
                        return;
                    }
                }
            }
            CompanyAction::BuyResource(resource, amount) => {
                // Buy resource to current best price
                if market_data.price_index[&resource].is_none() {
                    return;
                }
                self.place_order(
                    resource,
                    amount as f64,
                    market_data.price_index[&resource].unwrap().1,
                );
            }
            CompanyAction::SellResource(resource, amount) => {
                // Sell resource to current best price
                if market_data.order_index[&resource].is_none() {
                    return;
                }
                self.place_offer(
                    resource,
                    amount as f64,
                    market_data.order_index[&resource].unwrap().1,
                );
            }
        }
    }

    pub fn add_currency(&mut self, amount: f64) {
        self.currency += amount;
    }

    pub fn add_resource(&mut self, resource: ResourceHandle, amount: f64) {
        self.stock.add_to_stock(resource, amount);
    }

    // Methods to be used by an AI controller
    pub fn buy_processor(
        &mut self,
        recipe: RecipeHandle,
        processor_price: f64,
        recipe_data: &RecipeData,
    ) {
        if self.currency < processor_price {
            return;
        }
        self.currency -= processor_price;
        let processor_name = String::from("Proc")
            + &recipe_data
                .get_recipe_by_handle(recipe)
                .unwrap()
                .name
                .clone()
                .to_string();
        let proc = Processor {
            name: processor_name,
            production_speed: 1.0,
            recipe: recipe,
            productive: true,
            produced_last_tick: false,
        };
        self.processors.push(proc);
    }

    pub fn sell_processor(&mut self, processor: usize, processor_price: f64) {
        if self.processors.len() <= processor {
            return;
        }
        self.currency += processor_price;
        self.processors.remove(processor);
    }

    pub fn place_order(&mut self, resource: ResourceHandle, amount: f64, max_price_per_unit: f64) {
        self.orders.push(UnprocessedOrder {
            resource: resource,
            amount: amount,
            max_price_per_unit: max_price_per_unit,
            time_to_live: 100,
        });
    }

    pub fn place_offer(&mut self, resource: ResourceHandle, amount: f64, price_per_unit: f64) {
        self.offers.push(UnprocessedOffer {
            resource: resource,
            amount: amount,
            price_per_unit: price_per_unit,
            time_to_live: 100,
        });
    }

    pub fn calculate_company_value(&self, market_data: &MarketData, processor_value: f64) -> f64 {
        let mut new_company_value = self.currency;
        // Add value of all processors
        // new_company_value += self.processors.len() as f64 * processor_value;
        // // Add stockpile value
        // for (resource, amount) in self.stock.resources.iter() {
        //     if market_data.price_index.contains_key(resource) {
        //         match market_data.price_index[resource] {
        //             Some((_, price)) => {
        //                 new_company_value += *amount * price;
        //             }
        //             None => {
        //                 continue;
        //             }
        //         };
        //     }
        // }
        // // Add companies offers current value
        // for offer in market_data.offers.values() {
        //     match offer.company {
        //         Some(company) => {
        //             if company != self.id {
        //                 continue;
        //             }
        //         }
        //         None => {
        //             continue;
        //         }
        //     }
        //     match market_data.price_index[&offer.resource] {
        //         Some((_, price)) => {
        //             new_company_value += offer.amount * price;
        //         }
        //         None => {
        //             break;
        //         }
        //     };
        // }
        new_company_value
    }
}

use crate::economy::processor::Processor;
use crate::economy::recipe::RecipeHandle;
use crate::economy::resource::ResourceHandle;
use crate::economy::stock::Stock;
use crate::market::offer::UnprocessedOffer;
use crate::market::order::UnprocessedOrder;
use crate::reinforcement_learning::agent::CompanyAgent;
use crate::reinforcement_learning::company_trainer::CompanyTrainer;
use crate::reinforcement_learning::state::CompanyAction;
use crate::reinforcement_learning::state::CompanyState;
use crate::world_data::market_data::MarketData;
use crate::world_data::recipe_data::RecipeData;
use rurel::strategy::explore::RandomExploration;
use rurel::strategy::learn::QLearning;
use rurel::strategy::terminate::SinkStates;
use serde::{Deserialize, Serialize};

pub type CompanyHandle = usize;

#[derive(Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    pub stock: Stock,
    pub currency: f64,
    pub processors: Vec<Processor>,
    pub orders: Vec<UnprocessedOrder>,
    pub offers: Vec<UnprocessedOffer>,
    pub company_value: f64,
    pub id: CompanyHandle,
    trainer: CompanyTrainer,
    last_action: CompanyAction,
}

impl Company {
    pub fn new(name: &str, company_handle: CompanyHandle, resource_count: usize) -> Self {
        Company {
            name: name.to_string(),
            stock: Stock::new(),
            currency: 0.0,
            processors: vec![],
            orders: vec![],
            offers: vec![],
            company_value: 0.0,
            id: company_handle,
            trainer: CompanyTrainer::new(CompanyState::new(resource_count)),
            last_action: CompanyAction::Nothing,
        }
    }

    pub fn tick(
        &mut self,
        recipe_data: &RecipeData,
        agent: &mut CompanyAgent,
        exploration_strategy: &mut RandomExploration,
        market_data: &MarketData,
    ) {
        for processor in self.processors.iter() {
            processor.tick(&mut self.stock, recipe_data);
        }
        // Construct company state
        let mut company_state = CompanyState {
            stock: self.stock.resources.values().map(|x| *x as usize).collect(),
            currency: self.currency as usize,
            price_index: market_data
                .price_index
                .values()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap().1 as usize)
                .collect(),
            order_index: market_data
                .order_index
                .values()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap().1 as usize)
                .collect(),
        };
        // Select action by agent
        self.last_action = self
            .trainer
            .action_step(company_state, agent, exploration_strategy);

        // Act according to agent decision
        match self.last_action {
            CompanyAction::Nothing => {
                // do nothing
            }
            CompanyAction::BuyProcessor(recipe) => {
                self.buy_processor(recipe);
            }
            CompanyAction::SellProcessor(processor) => {
                // TODO: Only create order if resource exists
                self.buy_processor(processor);
            }
            CompanyAction::BuyResource(resource, amount, max_price) => {
                self.place_order(resource, amount as f64, max_price as f64);
            }
            CompanyAction::SellResource(resource, amount, price) => {
                self.place_offer(resource, amount as f64, price as f64)
            }
        }
    }

    pub fn train(
        &mut self,
        market_data: &MarketData,
        processor_value: f64,
        agent: &mut CompanyAgent,
        learning_strategy: &mut QLearning,
        termination_strategy: &mut SinkStates,
    ) {
        let reward = self.update_company_value(market_data, processor_value);
        self.trainer.training_step(
            reward,
            agent,
            learning_strategy,
            termination_strategy,
            self.last_action.clone(),
        );
    }

    pub fn add_currency(&mut self, amount: f64) {
        self.currency += amount;
    }

    pub fn add_resource(&mut self, resource: ResourceHandle, amount: f64) {
        self.stock.add_to_stock(resource, amount);
    }

    // Methods to be used by an AI controller
    pub fn buy_processor(&mut self, recipe: RecipeHandle) {}

    pub fn sell_processor(&mut self, processor: Processor) {}

    pub fn place_order(&mut self, resource: ResourceHandle, amount: f64, max_price_per_unit: f64) {
        self.orders.push(UnprocessedOrder {
            resource,
            amount,
            max_price_per_unit,
        });
    }

    pub fn place_offer(&mut self, resource: ResourceHandle, amount: f64, price_per_unit: f64) {
        self.offers.push(UnprocessedOffer {
            resource,
            amount,
            price_per_unit,
        });
    }

    pub fn update_company_value(&mut self, market_data: &MarketData, processor_value: f64) -> f64 {
        let mut new_company_value = self.currency;
        // Add value of all processors
        new_company_value += self.processors.len() as f64 * processor_value;
        // Add stockpile value
        for (resource, amount) in self.stock.resources.iter() {
            if market_data.price_index.contains_key(resource) {
                match market_data.price_index[resource] {
                    Some((_, price)) => {
                        new_company_value += *amount * price;
                    }
                    None => {
                        continue;
                    }
                };
            }
        }
        let old_company_value = self.company_value;
        // Add companies offers current value
        for order in market_data.offers.values() {
            if order.company != self.id {
                continue;
            }
            match market_data.price_index[&order.resource] {
                Some((_, price)) => {
                    new_company_value += order.amount * price;
                }
                None => {
                    break;
                }
            };
        }
        self.company_value = new_company_value;
        self.company_value - old_company_value
    }
}

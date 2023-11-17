use serde::{Deserialize, Serialize};
// Constants

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct CompanyState {
    // Stockpile
    pub stock: Vec<usize>,
    // Currentcy
    pub currency: usize,
    // Price and order index
    pub price_index: Vec<usize>,
    pub order_index: Vec<usize>,
    // Processor counts
    pub processor_counts: Vec<usize>,
    // Production rates
    pub production_rates: Vec<usize>,
}

impl CompanyState {}

impl CompanyState {
    pub fn new(resource_count: usize, recipe_count: usize) -> CompanyState {
        CompanyState {
            stock: vec![0; resource_count],
            currency: 0,
            price_index: vec![0; resource_count],
            order_index: vec![0; resource_count],
            processor_counts: vec![0; recipe_count],
            production_rates: vec![0; resource_count],
        }
    }

    pub fn as_f64_vec(&self) -> Vec<f64> {
        let mut return_value: Vec<f64> = vec![];
        let mut stock_vec: Vec<f64> = self.stock.iter().map(|x| *x as f64).collect();
        return_value.append(&mut stock_vec);
        return_value.push(self.currency as f64);
        let mut price_index_vec: Vec<f64> = self.price_index.iter().map(|x| *x as f64).collect();
        return_value.append(&mut price_index_vec);
        let mut order_index_vec: Vec<f64> = self.order_index.iter().map(|x| *x as f64).collect();
        return_value.append(&mut order_index_vec);
        let mut processor_counts_vec: Vec<f64> =
            self.processor_counts.iter().map(|x| *x as f64).collect();
        return_value.append(&mut processor_counts_vec);
        let mut production_rates_vec: Vec<f64> =
            self.production_rates.iter().map(|x| *x as f64).collect();
        return_value.append(&mut production_rates_vec);
        return_value
    }
}

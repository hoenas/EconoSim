use std::collections::HashMap;

use crate::economy::stock::Stock;

#[test]
fn add_to_stock_add_nothing() {
    let mut stock = Stock::new();
    // Add resources
    stock.resources.insert(0, 10.0);
    stock.resources.insert(1, 10.0);
    // Actual test
    stock.add_to_stock(0, 0.0);
    assert_eq!(*stock.resources.get(&0).unwrap(), 10.0);
    assert_eq!(*stock.resources.get(&0).unwrap(), 10.0);
}

#[test]
fn add_to_stock_add_something() {
    let mut stock = Stock::new();
    // Add resources
    stock.resources.insert(0, 10.0);
    stock.resources.insert(1, 10.0);
    // Actual test
    stock.add_to_stock(1, 10.0);
    assert_eq!(*stock.resources.get(&0).unwrap(), 10.0);
    assert_eq!(*stock.resources.get(&1).unwrap(), 20.0);
}

#[test]
#[should_panic]
fn add_to_stock_subtract_something() {
    let mut stock = Stock::new();
    // Add resources
    stock.resources.insert(0, 10.0);
    stock.resources.insert(1, 10.0);
    // Actual test
    stock.add_to_stock(1, -10.0);
}

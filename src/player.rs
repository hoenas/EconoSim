use crate::stock::Stock;

pub struct Player <'a> {
    name: String,
    stock: Stock<'a>,
}

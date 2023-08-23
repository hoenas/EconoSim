use rsrl::spaces::Space;

pub struct ActionSpace {}

pub enum ActionIdentifier {
    Nothing,
    BuyProcessor(usize),
    SellProcessor(usize),
    BuyResource(usize, i64, i64),
    SellResource(usize, i64, i64),
}

impl Space for ActionSpace {
    /// The data representation of the space.
    type Value: Clone;

    /// Return the dimensionality of the space.
    fn dim(&self) -> Dim {
        return Dim::Finite(4);
    }

    /// Return the number of elements in the set comprising the space.
    fn card(&self) -> Card {
        return Card::Finite(4);
    }
}

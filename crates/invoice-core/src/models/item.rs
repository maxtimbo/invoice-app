use crate::models::ids::ItemId;
use crate::models::currency::Currency;
use crate::models::quantity::Quantity;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub rate: Currency,
}

#[derive(Debug, Clone)]
pub struct ItemDetail {
    pub name: String,
    pub rate: Currency,
    pub quantity: Quantity,
    pub subtotal: Currency,
}

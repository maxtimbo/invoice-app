use crate::models::ids::ItemId;
use crate::models::currency::Currency;
use crate::models::quantity::Quantity;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub rate: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDetail {
    pub id: ItemId,
    pub name: String,
    pub rate: Currency,
    pub quantity: Quantity,
    pub subtotal: Currency,
}

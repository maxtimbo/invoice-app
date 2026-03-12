use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaidStatus {
    Waiting,
    PastDue,
    Paid { date: NaiveDate, check: Option<String> },
    Failed { date: NaiveDate },
    Refunded { date: NaiveDate },
}

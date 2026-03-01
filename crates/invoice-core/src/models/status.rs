use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub enum PaidStatus {
    Waiting,
    PastDue,
    Paid { date: NaiveDate, check: Option<String> },
    Failed { date: NaiveDate },
    Refunded { date: NaiveDate },
}

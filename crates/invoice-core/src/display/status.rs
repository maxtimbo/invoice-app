use std::fmt;

use crate::models::status::PaidStatus;

impl fmt::Display for PaidStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaidStatus::Waiting => write!(f, "Waiting for payment"),
            PaidStatus::PastDue => write!(f, "Payment is past due"),
            PaidStatus::Paid { date, check } => {
                match check {
                    Some(check_number) => {
                        write!(f, "Paid on {} (Check #{})", date, check_number)
                    }
                    None => {
                        write!(f, "Paid on {}", date)
                    }
                }
            },
            PaidStatus::Failed { date } => write!(f, "Failed\nDate:\t\t{}\n", date),
            PaidStatus::Refunded { date } => write!(f, "Refunded\nDate:\t\t{}\n", date),
        }
    }
}

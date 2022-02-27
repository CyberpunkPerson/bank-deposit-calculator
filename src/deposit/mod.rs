use std::collections::LinkedList;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::Context;

use bigdecimal::BigDecimal;
use chrono::Month;

pub(crate) mod fixed;
mod save;

pub(crate) struct PaymentMonth {
    pub month: Month,
    pub days_in_month: u32,
}

pub(crate) struct Payment {
    pub month: PaymentMonth,
    pub amount: BigDecimal,
    pub total: BigDecimal,
}


pub(crate) trait DepositPlan {
    fn get_payments(&self) -> &LinkedList<Payment>;
}

pub(crate) enum DepositType {
    FIXED,
    SAVE
}

type ParseError = &'static str;

impl FromStr for DepositType {
    type Err = ParseError;

    fn from_str(deposit_type: &str) -> Result<Self, Self::Err> {
        match deposit_type {
            "fixed" => Ok(DepositType::FIXED),
            "save" => Ok(DepositType::SAVE),
            _=> Err("Failed to parse deposit type"),
        }
    }
}

impl Display for DepositType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DepositType::FIXED => write!(f, "fixed"),
            DepositType::SAVE => write!(f, "save")
        }
    }
}

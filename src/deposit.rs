use std::collections::LinkedList;
use std::ops::{Add, Sub};

use bigdecimal::BigDecimal;
use chrono::{Datelike, Month, NaiveDate};

use crate::{Duration, FromPrimitive};

pub trait DepositPlan {
    fn get_payments(&self) -> LinkedList<Payment>;
}

pub struct PaymentMonth {
    pub month: Month,
    pub days_in_month: u32,
}

pub struct Payment {
    pub month: PaymentMonth,
    pub amount: BigDecimal,
    pub total: BigDecimal,
}

pub struct CapitalizationDepositPlan {
    current_amount: BigDecimal,
    open_date: NaiveDate,
    prolongation_date: NaiveDate,
    rate: BigDecimal,
    pub payments: LinkedList<Payment>,
    final_amount: BigDecimal,
}

impl CapitalizationDepositPlan {
    pub fn new(
        current_amount: BigDecimal,
        open_date: NaiveDate,
        prolongation_date: NaiveDate,
        rate: BigDecimal,
    ) -> Self {
        let mut state = Self {
            current_amount: current_amount.clone(),
            open_date,
            prolongation_date,
            rate,
            payments: Default::default(),
            final_amount: current_amount,
        };
        state.payments = Self::calculate_payments(&mut state);
        state
    }

    fn calculate_payments(&mut self) -> LinkedList<Payment> {
        let init_date = self.open_date.sub(Duration::days(1));
        let month_duration = self.prolongation_date.signed_duration_since(self.open_date);

        (1..month_duration.num_days())
            .map(|day| {
                let iter_date = init_date.add(Duration::days(day));
                (iter_date, iter_date.add(Duration::days(1)))
            })
            .filter(|&(iter_day, next_day)| iter_day.month() != next_day.month())
            .map(|(iter_day, _next_day)| Self::build_payment(self, iter_day))
            .collect()
    }

    fn build_payment(&mut self, iter_day: NaiveDate) -> Payment {
        let amount_per_month: BigDecimal =
            (&self.rate / 100 / 365) * &self.final_amount * BigDecimal::from(iter_day.day());
        self.final_amount = (&self.final_amount + &amount_per_month).with_scale(3);
        Payment {
            month: PaymentMonth {
                month: Month::from_u32(iter_day.month()).unwrap(),
                days_in_month: iter_day.day(),
            },
            amount: amount_per_month.with_scale(3),
            total: self.final_amount.clone(),
        }
    }
}

use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{Duration, NaiveDate};
use structopt::StructOpt;

use crate::deposit::CapitalizationDepositPlan;

mod deposit;

#[derive(StructOpt)]
struct DepositCli {
    #[structopt(long = "amount")]
    current_amount: BigDecimal,

    #[structopt(long = "open-date")]
    #[structopt(parse(try_from_str))]
    open_date: NaiveDate,

    #[structopt(long = "close-date")]
    #[structopt(parse(try_from_str))]
    prolongation_date: NaiveDate,

    #[structopt(short, long)]
    rate: BigDecimal,
}

fn main() -> Result<()> {
    let input = DepositCli::from_args();

    println!("Your input parameters are:");
    println!("Current amount is: {}", input.current_amount);
    println!("Open date is: {}", input.open_date);
    println!("Prolongation date is: {}", input.prolongation_date);
    println!("Rate is: {}", input.rate);

    let deposit_plan = CapitalizationDepositPlan::new(
        input.current_amount,
        input.open_date,
        input.prolongation_date,
        input.rate,
    );

    deposit_plan.payments.iter().for_each(|payment| {
        println!(
            "For {} amount is {}",
            payment.month.month.name(),
            payment.total
        );
    });

    Ok(())
}
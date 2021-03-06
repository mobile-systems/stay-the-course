#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate rust_decimal;

use self::chrono::NaiveDate;
use self::rust_decimal::Decimal;
use std::io;

mod allocation;
mod assets;
mod gnucash;
mod rebalance;
mod stats;

use gnucash::Book;

fn get_contribution() -> Decimal {
    let mut contribution = String::new();

    println!("How much to contribute or withdraw?");
    io::stdin()
        .read_line(&mut contribution)
        .expect("Failed to read line");

    contribution.trim().parse().expect("Please type a number!")
}

fn main() {
    let sqlite_file = "example.sqlite3";
    let book = Book::from_sqlite_file(sqlite_file);
    //let book = Book::from_xml_file("example.gnucash");

    // Identify our ideal allocations (percentages by asset class, summing to 100%)
    let birthday = NaiveDate::from_ymd(1960, 1, 1);
    let bond_allocation = allocation::bond_allocation(birthday, 120);
    let ideal_allocations = allocation::core_four(bond_allocation);

    let asset_classifications =
        assets::AssetClassifications::from_csv("data/classified.csv").unwrap();
    let portfolio = book.portfolio_status(asset_classifications, ideal_allocations);

    let sql_stats = stats::Stats::new(sqlite_file);
    println!(
        "After-tax income: ${:.0}",
        sql_stats.after_tax_income().unwrap()
    );
    println!("{:}\n", portfolio);
    let contribution = get_contribution();

    // From those ideal allocations, identify the best way to invest a lump sum
    let balanced_portfolio = rebalance::optimally_allocate(portfolio, contribution);
    balanced_portfolio.describe_future_contributions();
}

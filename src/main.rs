use exitfailure::ExitFailure;
use std::env;
use colored::*;
//use spinners::{Spinner, Spinners};

mod coin;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
  println!("\n{}\n", "Coind".blue().bold());

  let mut args = env::args();
  let coin_id = args.nth(1);
  let currency = args.nth(0);

  if coin_id.is_none() {
    let mut id = String::new();
    let mut money = String::new();

    println!("Enter coin id: ");
    std::io::stdin().read_line(&mut id).unwrap();
    println!("{}", id);
    println!("Enter currency: ");
    std::io::stdin().read_line(&mut money).unwrap();

    fetch_money(&id, &money).await;

  } else {
    let mut money: String = String::from("php");
    if currency.is_some() {
      money = currency.unwrap();
    } else {
      println!("There are no specified currency; default one \"PHP\" will be used instead.");
    }

    let id = coin_id.unwrap();
    fetch_money(&id, &money).await;
  }

  println!("\n");
  return Ok(());
}

async fn fetch_money(id: &String, currency: &String) {
  //let spinner = Spinner::new(&Spinners::Dots9, "Fetching".into());
  let coin = coin::Coin::fetch(&id).await.unwrap();
  //spinner.stop();

  println!("Name: {}", coin.name.cyan().bold());
  println!("Symbol: {}", coin.symbol.cyan().bold());
  println!("Current Price: {} {}", currency.to_uppercase().cyan(), coin.market_data.current_price[currency].to_string().cyan().bold());
}

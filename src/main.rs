use exitfailure::ExitFailure;
use std::env;

mod coin;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
  let mut args = env::args();
  let coin_id = args.nth(1).unwrap_or_else(|| "smooth-love-potion".to_string());
  let currency = args.nth(0).unwrap_or_else(|| "php".to_string());

  let coin = coin::Coin::fetch(&coin_id).await.unwrap();

  println!("Name: {}", coin.name);
  println!("Symbol: {}", coin.symbol);
  println!("Current Price: {}", coin.market_data.current_price[&currency]);

  return Ok(());
}


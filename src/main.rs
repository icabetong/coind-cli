use reqwest;
use exitfailure::ExitFailure;
use std::env;

mod domain {
  pub mod coin;
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
  let mut args = env::args();
  let coin_id = args.nth(1).unwrap_or_else(|| "smooth-love-potion".to_string());
  let currency = args.nth(0).unwrap_or_else(|| "php".to_string());

  let url = format!("https://api.coingecko.com/api/v3/coins/{id}", id = coin_id);
  let result = reqwest::get(url).await.unwrap().json::<domain::coin::Coin>().await.unwrap();

  println!("Current Price: {:?}", result.market_data.current_price[&currency]);
  return Ok(());
}


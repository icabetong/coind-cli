use reqwest;
use serde::{Deserialize, Serialize};
use exitfailure::ExitFailure;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
  let url = format!("https://api.coingecko.com/api/v3/coins/{id}", id = "smooth-love-potion");
  let result = reqwest::get(url).await.unwrap().json::<Coin>().await;

  println!("{:?}", result);
  return Ok(());
}

#[derive(Serialize, Deserialize, Debug)]
struct Coin {
  id: String,
  symbol: String,
  name: String,
  market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
  current_price: HashMap<String, f32>
}
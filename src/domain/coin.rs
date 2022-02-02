use serde::{Deserialize, Serialize};
use std::collections::{HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct Coin {
  pub id: String,
  pub symbol: String,
  pub name: String,
  pub description: HashMap<String, String>,
  pub categories: Vec<String>,
  pub links: Option<Links>,
  pub market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketData {
  pub current_price: HashMap<String, f32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
  homepage: Vec<String>,
  blockchain_site: Vec<String>,
  official_forum_url: Vec<String>,
  chat_url: Vec<String>,
  announcement_url: Vec<String>,
  twitter_screen_name: Option<String>,
  facebook_username: Option<String>,
  bitcointalk_thread_identifier: Option<i32>,
  telegram_channel_identifier: Option<String>,
  subreddit_url: Option<String>
}
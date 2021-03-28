use std::time::Instant;
use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
  //start timer
  let start = Instant::now();

  let caps = DesiredCapabilities::chrome();
  let driver = WebDriver::new("http://localhost:4444", &caps).await?;

  // Navigate to https://wikipedia.org.
  driver.get("https://play2048.co").await?;
  let html_body = driver.find_element(By::Tag("body")).await?;
  // println!("{}", html_body);

  // restart
  html_body.send_keys('r').await?;

  for _ in 0..100 {
    html_body.send_keys(Keys::Up).await?;
    html_body.send_keys(Keys::Right).await?;
    html_body.send_keys(Keys::Down).await?;
    html_body.send_keys(Keys::Left).await?;
  }

  //end timer
  let end = start.elapsed();
  println!(
    "Send keys :{}.{:03}[s]",
    end.as_secs(),
    end.subsec_nanos() / 1_000_000
  );

  //start timer
  let start = Instant::now();

  let mut tiles_vec = Vec::new();
  let grid = driver.find_element(By::ClassName("tile-container")).await?;
  for tile in grid.find_elements(By::ClassName("tile")).await? {
    // println!("{:?}", tile.class_name().await?);
    // println!("{}", tile.text().await?);
    // tiles_vec.push(tile.text().await?);
    let tmp_num: i32 = tile.text().await?.parse().unwrap();
    tiles_vec.push(tmp_num);
  }
  //end timer
  let end = start.elapsed();
  println!(
    "Get tile info :{}.{:03}[s]",
    end.as_secs(),
    end.subsec_nanos() / 1_000_000
  );

  // println!("{:?}", tiles_vec);
  tiles_vec.sort_unstable();
  println!("{:?}", tiles_vec);
  println!("max num of tile: {}", tiles_vec[tiles_vec.len() - 1]);

  let score = driver
    .find_element(By::ClassName("score-container"))
    .await?;
  println!("{}", score.text().await?);

  Ok(())
}

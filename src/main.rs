use std::error::Error;
use advent_tools::fetch_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2023/day/1/input";
    let cookie_value = "";

    let data = fetch_data(url, cookie_value).await?;

    println!("{:?}", data);

    Ok(())
}

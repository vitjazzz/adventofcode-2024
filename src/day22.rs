use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;

const PRUNE_AND_NUMBER: i64 = 0b111111111111111111111111;
const PRUNE_NUMBER: i64 = 0b1000000000000000000000000;
const DIVIDER_32: i64 = 0b100000;
const MULTIPLIER_64: i64 = 0b1000000;
const MULTIPLIER_2048: i64 = 0b100000000000;
pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/22/input";
    let data = fetch_data(url).await?;
    // let data = test_data();
    // let data = vec!["123".to_string()];

    let start = Instant::now();

    let starting_numbers = get_starting_numbers(&data);
    
    let mut res = 0;
    for number in starting_numbers {
        let mut secret_number = number;
        for _ in 0..2000 {
            secret_number = evolve(secret_number);
        }
        res += secret_number;
    }

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", res, duration);

    Ok(())
}



fn evolve(secret_number: i64) -> i64 {
    let number = secret_number << 6;
    let secret_number = secret_number ^ number;
    let secret_number = secret_number & PRUNE_AND_NUMBER;

    let number = secret_number >> 5;
    let secret_number = secret_number ^ number;
    let secret_number = secret_number & PRUNE_AND_NUMBER;

    let number = secret_number << 11;
    let secret_number = secret_number ^ number;
    let secret_number = secret_number & PRUNE_AND_NUMBER;

    secret_number
}

fn get_starting_numbers(data: &Vec<String>) -> Vec<i64> {
    data.iter().map(|s| s.parse().unwrap()).collect()
}

fn test_data() -> Vec<String> {
    r"  1
        10
        100
        2024"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}
use std::collections::HashMap;
use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;

const PRUNE_AND_NUMBER: i64 = 0b111111111111111111111111;
pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/22/input";
    let data = fetch_data(url).await?;
    // let data = test_data();
    // let data = vec!["123".to_string()];

    let start = Instant::now();

    let starting_numbers = get_starting_numbers(&data);
    let mut secret_numbers: HashMap<i64, Vec<i64>> = HashMap::new();
    
    for number in starting_numbers {
       secret_numbers.insert(number, evolve_n_times(number, 2000));
    }
    let res: i64 = secret_numbers.iter().map(|(k, v)| v[2000]).sum();

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", res, duration);

    Ok(())
}

fn evolve_n_times(secret_number: i64, times: i32) -> Vec<i64> {
    let mut res = vec![];
    let mut secret_number = secret_number;
    for _ in 0..=times {
        res.push(secret_number);
        secret_number = evolve(secret_number);
    }
    res
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
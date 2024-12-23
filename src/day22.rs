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
    let secret_numbers: HashMap<i64, Vec<i64>> = starting_numbers.iter().map(|&n| (n, evolve_n_times(n, 2000))).collect();
    let prices: HashMap<i64, Vec<i64>> = secret_numbers.iter().map(|(&key, numbers)| (key, get_prices(numbers))).collect();
    let price_changes: HashMap<i64, Vec<i64>> = prices.iter().map(|(&key, prices)| (key, get_price_changes(prices))).collect();

    let mut best_sequences: HashMap<Vec<i64>, Vec<i64>> = HashMap::new();
    for (key, price_change_vec) in price_changes {
        let current_best_sequences = calculate_best_sequences(prices.get(&key).unwrap(), &price_change_vec);
        best_sequences = merge_maps(best_sequences, current_best_sequences);
    }
    let mut best_price = 0;
    for best_prices in best_sequences.values() {
        let sum: i64 = best_prices.iter().sum();
        if sum > best_price {
            best_price = sum;
        }
    }

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", best_price, duration);

    Ok(())
}

fn calculate_best_sequences(prices: &Vec<i64>, price_changes: &Vec<i64>) -> HashMap<Vec<i64>, Vec<i64>> {
    let mut res = HashMap::new();
    for i in 3..price_changes.len() {
        let sequence = vec![price_changes[i - 3], price_changes[i - 2], price_changes[i - 1], price_changes[i]];
        if !res.contains_key(&sequence) {
            res.insert(sequence.clone(), vec![prices[i+1]]);
        }
    }
    res
}

fn merge_maps(mut map1: HashMap<Vec<i64>, Vec<i64>>, map2: HashMap<Vec<i64>, Vec<i64>>)
              -> HashMap<Vec<i64>, Vec<i64>> {
    for (key, value) in map2 {
        map1.entry(key)
            .and_modify(|existing_value| existing_value.extend(value.clone()))
            .or_insert(value);
    }
    map1
}

fn get_price_changes(prices: &Vec<i64>) -> Vec<i64> {
    let mut res = vec![];
    for i in 1..prices.len() {
        res.push(prices[i] - prices[i - 1]);
    }
    res
}

fn get_prices(secret_numbers: &Vec<i64>) -> Vec<i64> {
    secret_numbers.iter().map(|&n| n % 10).collect()
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
    // r"  1"
    r"  1
        2
        3
        2024"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}
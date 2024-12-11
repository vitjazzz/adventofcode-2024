use std::collections::HashMap;
use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;


pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/11/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let numbers: Vec<i64> = data[0].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();

    println!("{:?}", numbers);
    let mut stones_per_number: HashMap<i64, HashMap<i64, i64>> = HashMap::new();
    let mut score = 0;
    for n in numbers {
        score += blink_x_times(n, 75, &mut stones_per_number);
    }

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", score, duration);

    Ok(())
}

fn blink_x_times(number: i64, blinks: i64, stones_per_number: &mut HashMap<i64, HashMap<i64, i64>>) -> i64 {
    if blinks == 0 {
        return 1
    }
    let stones_per_blink = stones_per_number.entry(number).or_insert(HashMap::new());
    if let Some(stored_stones) = stones_per_blink.get(&blinks) {
        return *stored_stones;
    }
    let replaced_stones = transform_stone(number);
    let stones: i64 = replaced_stones.into_iter()
        .map(|new_number| blink_x_times(new_number, blinks - 1, stones_per_number))
        .sum();
    stones_per_number.entry(number).or_insert(HashMap::new()).insert(blinks, stones);
    stones
}

fn transform_stone(n: i64) -> Vec<i64> {
    if n == 0 {
        return vec![1]
    }
    let n_str = n.to_string();
    if n_str.len() % 2 == 0 {
        let (left, right) = n_str.split_at(n_str.len() / 2);
        return vec![left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap()]
    }
    return vec![n * 2024]
}


fn test_data() -> Vec<String> {
    r"125 17"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}
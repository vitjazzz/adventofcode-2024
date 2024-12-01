use std::error::Error;
use advent_tools::fetch_data;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/1/input";

    let data = fetch_data(url).await?;

    let first_numbers = get_sorted_numbers(&data, 0);
    let second_numbers = get_sorted_numbers(&data, 1);

    let diff_sum = calculate_diff_sum(&first_numbers, &second_numbers);
    println!("diff_sum: {:?}", diff_sum);

    let similarity_sum = calculate_similarity_sum(&first_numbers, &second_numbers);
    println!("similarity_sum: {:?}", similarity_sum);

    Ok(())

}

fn calculate_diff_sum(first_numbers: &Vec<i32>, second_numbers: &Vec<i32>) -> i32 {
    first_numbers.iter()
        .zip(second_numbers.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn calculate_similarity_sum(first_numbers: &Vec<i32>, second_numbers: &Vec<i32>) -> i32 {
    let mut similarity_sum = 0;
    let mut i = 0;
    let mut j = 0;
    let mut similarity_counter = 0;
    while i < first_numbers.len() && j < second_numbers.len() {
        let num_1 = first_numbers[i];
        let num_2 = second_numbers[j];
        if num_1 < num_2 {
            similarity_sum += num_1 * similarity_counter;
            i += 1;
            similarity_counter = 0;
        }
        if num_1 > num_2 {
            j += 1;
        }
        if num_1 == num_2 {
            similarity_counter += 1;
            j += 1;
        }
    }
    similarity_sum
}

fn get_sorted_numbers(data: &Vec<String>, n: usize) -> Vec<i32> {
    let mut numbers: Vec<i32> = data.iter()
        .filter_map(|s|
            s.split_whitespace()
                .nth(n)
                .and_then(|num_str| num_str.parse::<i32>().ok())
        )
        .collect();
    numbers.sort();
    numbers
}
use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/7/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let equations = get_equations(&data);

    let res: i64 = equations.into_iter()
        .filter(|equation| is_result_possible(equation.0, 1, '*', &equation.1, 0))
        .map(|equation| equation.0)
        .sum();

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", res, duration);

    Ok(())
}

fn is_result_possible(expected_result: i64, current_result: i64, operator: char, next_values: &Vec<i64>, next_value_index: usize) -> bool {
    if next_value_index == next_values.len() {
        return expected_result == current_result;
    }
    let next_value = next_values[next_value_index];
    let current_result = match operator {
        '+' => current_result + next_value,
        '*' => current_result * next_value,
        '|' => (current_result.to_string() + &next_value.to_string()).parse::<i64>().unwrap(),
        _ => -1111111111
    };
    if current_result > expected_result {
        return false
    }
    return is_result_possible(expected_result, current_result, '*', next_values, next_value_index + 1)
        || is_result_possible(expected_result, current_result, '+', next_values, next_value_index + 1)
        || is_result_possible(expected_result, current_result, '|', next_values, next_value_index + 1);
}

fn get_equations(data: &Vec<String>) -> Vec<(i64, Vec<i64>)> {
    data.iter()
        .map(|s| s.split(": "))
        .map(|mut splitted| (
            splitted.next().unwrap().parse::<i64>().unwrap(),
            splitted.next().unwrap().split_whitespace().map(|val| val.parse::<i64>().unwrap()).collect()
        ))
        .collect()
}

fn test_data() -> Vec<String> {
    r"  190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}
use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;

const MAX_TIMES: i64 = 100;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/13/input";
    let data = fetch_data(url).await?;
    let data = test_data();

    let start = Instant::now();
    let claw_machines = get_claw_machines(&data);

    let res: i64 = claw_machines.iter()
        .filter_map(|claw_machine| calculate_min_tokens(claw_machine))
        .sum();

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", res, duration);

    Ok(())
}

fn calculate_min_tokens(claw_machine: &ClawMachine) -> Option<i64> {
    let mut min_tokens = i64::MAX;
    for a in 0..MAX_TIMES {
        let new_location = calculate_location(claw_machine.button_a, a, claw_machine.button_b, 0);
        if new_location.0 > claw_machine.prize_location.0 || new_location.1 > claw_machine.prize_location.1 {
            break
        }
        for b in 0..MAX_TIMES {
            let new_location = calculate_location(claw_machine.button_a, a, claw_machine.button_b, b);
            if new_location.0 > claw_machine.prize_location.0 || new_location.1 > claw_machine.prize_location.1 {
                break
            }
            if new_location == claw_machine.prize_location && min_tokens > b + a * 3 {
                min_tokens = b + a * 3;
            }
        }
    }
    if min_tokens == i64::MAX { None } else { Some(min_tokens) }
}

fn calculate_location(button_a: (i64, i64), a_times: i64, button_b: (i64, i64), b_times: i64) -> (i64, i64) {
    let x = button_a.0 * a_times + button_b.0 * b_times;
    let y = button_a.1 * a_times + button_b.1 * b_times;
    (x, y)
}

fn get_claw_machines(data: &Vec<String>) -> Vec<ClawMachine> {
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    for i in 0..(data.len() / 4) + 1 {
        let starting_index = i * 4;
        let button_a = get_button(&data[starting_index]);
        let button_b = get_button(&data[starting_index + 1]);
        let prize_location = get_prize(&data[starting_index + 2]);
        claw_machines.push(ClawMachine{
            button_a,
            button_b,
            prize_location
        });
    }
    claw_machines
}

fn get_button(line: &String) -> (i64, i64) {
    let button_vec: Vec<i64> = line
        .split(": ")
        .nth(1).unwrap()
        .split(", ")
        .map(|s| s.split("+").nth(1).unwrap().parse::<i64>().unwrap())
        .collect();
    (button_vec[0], button_vec[1])
}

fn get_prize(line: &String) -> (i64, i64) {
    let button_vec: Vec<i64> = line
        .split(": ")
        .nth(1).unwrap()
        .split(", ")
        .map(|s| s.split("=").nth(1).unwrap().parse::<i64>().unwrap())
        .collect();
    (button_vec[0], button_vec[1])
}

fn test_data() -> Vec<String> {
    r"  Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize_location: (i64, i64),
}
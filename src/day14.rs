use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;
use regex::Regex;

// const SECONDS: i32 = 5;
const SECONDS: i32 = 100;
// const WIDTH: i32 = 11;
// const HEIGHT: i32 = 7;
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/14/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let robots = get_robots(&data);
    let robot_final_positions: Vec<(i32, i32)> = robots.iter()
        .map(|robot| calculate_position(robot, SECONDS))
        .collect();

    print_robots(&robot_final_positions);
    let score = calculate_score(&robot_final_positions);

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", score, duration);

    Ok(())
}

fn calculate_score(robot_positions: &Vec<(i32, i32)>) -> i32 {
    let mut left_top = 0;
    let mut right_top = 0;
    let mut left_bot = 0;
    let mut right_bot = 0;
    for (x, y) in robot_positions {
        if *x < WIDTH / 2 && *y < HEIGHT / 2 {
            left_top += 1;
        } else if *x > WIDTH / 2 && *y < HEIGHT / 2 {
            right_top += 1;
        } else if *x < WIDTH / 2 && *y > HEIGHT / 2 {
            left_bot += 1;
        } else if *x > WIDTH / 2 && *y > HEIGHT / 2 {
            right_bot += 1;
        }
    }
    left_top * right_top * left_bot * right_bot
}

fn calculate_position(robot: &Robot, seconds: i32) -> (i32, i32){
    let calculated_x = robot.position.0 + robot.velocity.0 * seconds;
    let calculated_y = robot.position.1 + robot.velocity.1 * seconds;
    let final_x = if calculated_x < 0 { WIDTH - calculated_x.abs() % WIDTH } else { calculated_x % WIDTH };
    let final_x = if final_x == WIDTH { 0 } else { final_x };
    let final_y = if calculated_y < 0 { HEIGHT - calculated_y.abs() % HEIGHT } else { calculated_y % HEIGHT };
    let final_y = if final_y == HEIGHT { 0 } else { final_y };
    (final_x, final_y)
}

fn get_robots(data: &Vec<String>) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    for line in data {
        let re = Regex::new(r"-?\d+").unwrap();
        let numbers: Vec<i32> = re
            .find_iter(line)
            .filter_map(|m| m.as_str().parse::<i32>().ok())
            .collect();
        robots.push(Robot{
            position: (numbers[0], numbers[1]),
            velocity: (numbers[2], numbers[3]),
        });
    }
    robots
}


fn test_data() -> Vec<String> {
    r"  p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3"
    // r"  p=2,4 v=2,-3"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

#[derive(Debug, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn print_robots(robot_positions: &Vec<(i32, i32)>) {
    println!();
    for i in 0..HEIGHT {
        println!();
        for j in 0..WIDTH {
            let robots = robot_positions.iter().filter(|&position| position == &(j, i)).count();
            if robots == 0 {
                print!(".");
            } else {
                print!("{robots}");
            }
        }
    }
    println!();
}
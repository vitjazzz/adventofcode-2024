use std::error::Error;
use std::fmt::format;
use advent_tools::fetch_data;
use regex::Regex;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/3/input";

    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();

    let data = fetch_data(url).await?;

    let mut enabled = true;
    let res: i32 = data
        .iter()
        .flat_map(|line| re.captures_iter(line))
        .map(|caps| {
            if caps[0].starts_with("do()") {
                enabled = true;
            } else if caps[0].starts_with("don't()") {
                enabled = false;
            } else if enabled {
                return caps[2].parse::<i32>().unwrap() * caps[3].parse::<i32>().unwrap()
            }
            return 0
        })
        .sum();
    println!("res - {:?}", res);

    Ok(())
}

fn is_safe(report: &Vec<i32>, allowed_unsafe: i32) -> bool {
    let mut dir = 0;
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        let new_dir = if diff == 0 { 0 } else { diff / diff.abs() };
        let is_unsafe = diff == 0 || diff.abs() > 3 || new_dir + dir == 0;
        if is_unsafe {
            if allowed_unsafe == 0 {
                return false;
            } else {
                let left_plus_index = if i == 1 { i } else { i - 2 };
                let report_left_plus = remove_index(report, left_plus_index);
                let report_left = remove_index(report, i - 1);
                let report_right = remove_index(report, i);
                let res = is_safe(&report_left_plus, allowed_unsafe - 1)
                    || is_safe(&report_left, allowed_unsafe - 1)
                    || is_safe(&report_right, allowed_unsafe - 1);
                return res;
            }
        }
        dir = new_dir;
    }
    return true;
}

fn remove_index(report: &Vec<i32>, i: usize) -> Vec<i32> {
    return report.iter()
        .enumerate()
        .filter(|&(index, _)| index != i)
        .map(|(_, &v)| v)
        .collect();
}
use std::error::Error;
use advent_tools::fetch_data;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/2/input";

    let data = fetch_data(url).await?;

    let reports = get_reports(&data);

    let safe_reports_count = reports.iter()
        .filter(|&report| is_safe(report, 1))
        .count();

    println!("safe_reports_count: {}", safe_reports_count);

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
                return false
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


fn get_reports(data: &Vec<String>) -> Vec<Vec<i32>> {
    return data.iter()
        .map(|s|
            s.split_whitespace()
                .filter_map(|level_str| level_str.parse::<i32>().ok())
                .collect()
        )
        .collect();
}
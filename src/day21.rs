use std::collections::{HashMap, VecDeque};
use std::error::Error;
use advent_tools::fetch_data;
use regex::Regex;
use tokio::time::Instant;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/21/input";
    let data = fetch_data(url).await?;
    let data = test_data();

    let start = Instant::now();
    let codes = get_codes(&data);
    let code_panel = code_panel();
    let robot_panel = robot_panel();
    let mut score = 0;

    for code in codes {
        let shortest_path = search_shortest_path(&code, &code_panel);
        let shortest_path = search_shortest_path(&shortest_path, &robot_panel);
        // let shortest_path = search_shortest_path(&shortest_path, &robot_panel);
        let numeric_part = get_number(&code);
        println!("{:?}: {}, numeric_part: {}, length: {}", code, shortest_path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""),
                 numeric_part, shortest_path.len());
        score += numeric_part * shortest_path.len() as i32;
    }

    let duration = start.elapsed();
    println!("\nResult: {}, Execution time: {:?}", score, duration);

    Ok(())
}

fn search_shortest_path(expected_code: &Vec<char>, panel: &Vec<Vec<char>>) -> Vec<char> {
    let mut shortest_path = Vec::new();
    for i in 0..expected_code.len() {
        let start_code = if i == 0 { 'A' } else { expected_code[i - 1] };
        let final_code = expected_code[i];
        let path = find_single_shortest_path(start_code, final_code, panel);
        shortest_path.extend(path);
        shortest_path.push('A');
    }

    shortest_path
}

fn find_single_shortest_path(start_code: char, final_code: char, panel: &Vec<Vec<char>>) -> Vec<char> {
    let start_position = get_position(&panel, start_code);
    let mut path_panel = build_path_panel(&panel);
    let mut tasks: VecDeque<(Vec<char>, (usize, usize))> = VecDeque::new();
    tasks.push_back((vec![], start_position));
    while let Some(task) = tasks.pop_front() {
        let (current_path, (i, j)) = task;
        let current_element = panel[i][j];
        if current_element == '#' {
            continue;
        }
        let stored_path = &path_panel[i][j];
        if !stored_path.is_empty() && stored_path.len() <= current_path.len() {
            continue;
        }
        path_panel[i][j] = current_path.clone();
        if current_element == final_code {
            return continue;
        }
        tasks.push_back((add_and_clone(&current_path, '>'), (i, j + 1)));
        tasks.push_back((add_and_clone(&current_path, 'v'), (i + 1, j)));
        tasks.push_back((add_and_clone(&current_path, '<'), (i, j - 1)));
        tasks.push_back((add_and_clone(&current_path, '^'), (i - 1, j)));
    }
    let final_position = get_position(&panel, final_code);
    path_panel[final_position.0][final_position.1].clone()
}

fn add_and_clone(path: &Vec<char>, c: char) -> Vec<char> {
    let mut new_path = path.clone();
    new_path.push(c);
    new_path
}

fn code_panel() -> Vec<Vec<char>> {
    vec![
        vec!['#', '#', '#', '#', '#'],
        vec!['#', '7', '8', '9', '#'],
        vec!['#', '4', '5', '6', '#'],
        vec!['#', '1', '2', '3', '#'],
        vec!['#', '#', '0', 'A', '#'],
        vec!['#', '#', '#', '#', '#'],
    ]
}

fn robot_panel() -> Vec<Vec<char>> {
    vec![
        vec!['#', '#', '#', '#', '#'],
        vec!['#', '#', '^', 'A', '#'],
        vec!['#', '<', 'v', '>', '#'],
        vec!['#', '#', '#', '#', '#'],
    ]
}

fn build_path_panel(map: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    map.iter()
        .map(|s| s.iter().map(|_| vec![]).collect::<Vec<Vec<char>>>())
        .collect()
}

fn get_position(map: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == c {
                return (i, j);
            }
        }
    }
    return (0, 0);
}

fn get_codes(data: &Vec<String>) -> Vec<Vec<char>> {
    data.iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect()
}

fn get_number(code: &Vec<char>) -> i32 {
    let re = Regex::new(r"^0*(\d+)").unwrap();
    let code = code.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("");
    if let Some(caps) = re.captures(&code) {
        *&caps[1].parse::<i32>().unwrap()
    } else {
        panic!("Couldn't parse")
    }
}

fn test_data() -> Vec<String> {
    r"  029A
        980A
        179A
        456A
        379A"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}
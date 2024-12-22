use std::collections::{HashMap, VecDeque};
use std::error::Error;
use advent_tools::fetch_data;
use regex::Regex;
use tokio::time::Instant;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/21/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let codes = get_codes(&data);
    let code_panel = code_panel();
    let robot_panel = robot_panel();
    let mut score = 0;

    let mut shortest_path_map: HashMap<(char, char, i32), Vec<char>> = HashMap::new();

    for code in codes {
        let code_paths = vec![code.clone()];
        let shortest_paths = search_shortest_paths(&code_paths, &code_panel, &mut shortest_path_map, 0);
        let numeric_part = get_number(&code);
        println!("{:?}: {}, numeric_part: {}, length: {}", code, shortest_paths[0].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""),
                 numeric_part, shortest_paths[0].len());
        score += numeric_part * shortest_paths[0].len() as i32;
    }

    let duration = start.elapsed();
    println!("\nResult: {}, Execution time: {:?}", score, duration);

    Ok(())
}

fn search_shortest_paths(expected_codes: &Vec<Vec<char>>,
                         panel: &Vec<Vec<char>>,
                         shortest_path_map: &mut HashMap<(char, char, i32), Vec<char>>,
                         recursion_level: i32) -> Vec<Vec<char>> {
    if recursion_level > 2 {
        return expected_codes.clone();
    }
    let robot_panel = robot_panel();

    let mut shortest_paths = Vec::new();
    for code in expected_codes {
        let mut shortest_path: Vec<char> = Vec::new();
        for i in 0..code.len() {
            let start_code = if i == 0 { 'A' } else { code[i - 1] };
            let final_code = code[i];
            if let Some(path) = shortest_path_map.get(&(start_code, final_code, recursion_level)) {
                shortest_path.extend(path.clone());
                continue;
            }
            let paths = find_multiple_shortest_paths(start_code, final_code, panel);
            // add 'A' to the end of all paths
            let paths = paths.iter().map(|x| {
                let mut path = x.clone();
                path.push('A');
                path
            }).collect::<Vec<_>>();
            let internal_shortest_paths = search_shortest_paths(&paths, &robot_panel, shortest_path_map, recursion_level + 1);
            shortest_path_map.insert((start_code, final_code, recursion_level), internal_shortest_paths[0].clone());
            shortest_path.extend(internal_shortest_paths[0].clone());
        }
        shortest_paths.push(shortest_path);
    }
    let shortest_path_len = shortest_paths.iter().map(|x| x.len()).min().unwrap();
    let shortest_paths = shortest_paths.iter().filter(|x| x.len() == shortest_path_len).map(|x| x.clone()).collect::<Vec<_>>();

    shortest_paths
}

fn find_multiple_shortest_paths(start_code: char, final_code: char, panel: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let start_position = get_position(&panel, start_code);
    let mut path_panel = build_path_panel(&panel);
    let mut tasks: VecDeque<(Vec<Vec<char>>, (usize, usize))> = VecDeque::new();
    tasks.push_back((vec![vec![]], start_position));
    while let Some(task) = tasks.pop_front() {
        let (current_paths, (i, j)) = task;
        let current_element = panel[i][j];
        if current_element == '#' {
            continue;
        }
        let stored_paths = &path_panel[i][j];
        if !stored_paths[0].is_empty() && stored_paths[0].len() < current_paths[0].len() {
            continue;
        }
        if !stored_paths[0].is_empty() && stored_paths[0].len() == current_paths[0].len() {
            // merge paths and store
            let mut new_paths = current_paths.clone();
            new_paths.extend(stored_paths.clone());
            new_paths.sort();
            new_paths.dedup();
            path_panel[i][j] = new_paths;
        } else {
            path_panel[i][j] = current_paths.clone();
        }
        if current_element == final_code {
            return continue;;
        }
        tasks.push_back((add_and_clone(&current_paths, '>'), (i, j + 1)));
        tasks.push_back((add_and_clone(&current_paths, 'v'), (i + 1, j)));
        tasks.push_back((add_and_clone(&current_paths, '<'), (i, j - 1)));
        tasks.push_back((add_and_clone(&current_paths, '^'), (i - 1, j)));
    }
    let final_position = get_position(&panel, final_code);
    path_panel[final_position.0][final_position.1].clone()
}

fn add_and_clone(paths: &Vec<Vec<char>>, c: char) -> Vec<Vec<char>> {
    paths.iter()
        .map(|path| {
            let mut new_path = path.clone();
            new_path.push(c);
            new_path
        })
        .collect()
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

fn build_path_panel(map: &Vec<Vec<char>>) -> Vec<Vec<Vec<Vec<char>>>> {
    map.iter()
        .map(|s| s.iter().map(|_| vec![vec![]]).collect::<Vec<Vec<Vec<char>>>>())
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
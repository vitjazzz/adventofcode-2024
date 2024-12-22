use std::collections::{HashMap, VecDeque};
use std::error::Error;
use advent_tools::fetch_data;
use regex::Regex;
use tokio::time::Instant;

const RECURSION_LEVEL: i32 = 25;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/21/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let codes = get_codes(&data);
    let code_panel = code_panel();
    let robot_panel = robot_panel();
    let mut score: i64 = 0;

    let internal_shortest_path_map = build_internal_shortest_path_map(&robot_panel);
    let full_internal_shortest_path_map = build_full_internal_shortest_path_map(&internal_shortest_path_map);

    let mut shortest_path_len_map: HashMap<(char, char, i32), i64> = HashMap::new();
    for code in codes {
        let code_paths = vec![code.clone()];
        let shortest_path_len = search_shortest_path_len(&code_paths, &code_panel, &mut shortest_path_len_map, &internal_shortest_path_map, &full_internal_shortest_path_map, 0);
        let numeric_part = get_number(&code);
        println!("{:?}: numeric_part: {}, length: {}", code, numeric_part, shortest_path_len);
        score += numeric_part * shortest_path_len;
    }

    let duration = start.elapsed();
    println!("\nResult: {}, Execution time: {:?}", score, duration);

    Ok(())
}

fn build_internal_shortest_path_map(robot_panel: &Vec<Vec<char>>) -> HashMap<(char, char), Vec<char>> {
    let mut internal_shortest_path_map: HashMap<(char, char), Vec<char>> = HashMap::new();
    for from in vec!['<', 'v', '>', '^', 'A'] {
        for to in vec!['<', 'v', '>', '^', 'A'] {
            let paths = find_multiple_shortest_paths(from, to, robot_panel);

            let mut shortest_path_len = i64::MAX;
            let mut real_shortest_path = paths[0].clone();
            for path in paths {
                let tmp = search_shortest_path_len(&vec![path.clone()], &robot_panel, &mut HashMap::new(), &HashMap::new(), &HashMap::new(), 1);
                if tmp < shortest_path_len {
                    real_shortest_path = path.clone();
                    shortest_path_len = tmp;
                }
            }
            internal_shortest_path_map.insert((from, to), real_shortest_path);
        }
    }
    internal_shortest_path_map
}

fn build_full_internal_shortest_path_map(internal_shortest_path_map: &HashMap<(char, char), Vec<char>>)
                                         -> HashMap<(char, char, i32), i64> {
    let mut full_internal_shortest_path_map: HashMap<(char, char, i32), i64> = HashMap::new();
    for recursion in (1..=RECURSION_LEVEL).rev() {
        for from in vec!['<', 'v', '>', '^', 'A'] {
            for to in vec!['<', 'v', '>', '^', 'A'] {
                let shortest_path = internal_shortest_path_map.get(&(from, to)).unwrap();
                let mut shortest_path_len = 0;
                for i in 0..shortest_path.len() {
                    let start_code = if i == 0 { 'A' } else { shortest_path[i - 1] };
                    let final_code = shortest_path[i];
                    shortest_path_len += full_internal_shortest_path_map
                        .get(&(start_code, final_code, recursion + 1))
                        .unwrap_or(&1);
                }
                full_internal_shortest_path_map.insert((from, to, recursion), shortest_path_len);
            }
        }
    }

    full_internal_shortest_path_map
}

fn search_shortest_path_len(expected_codes: &Vec<Vec<char>>,
                            panel: &Vec<Vec<char>>,
                            shortest_path_len_map: &mut HashMap<(char, char, i32), i64>,
                            internal_shortest_path_map: &HashMap<(char, char), Vec<char>>,
                            full_internal_shortest_path_map: &HashMap<(char, char, i32), i64>,
                            recursion_level: i32) -> i64 {
    if recursion_level > RECURSION_LEVEL {
        return expected_codes[0].len() as i64;
    }
    let robot_panel = robot_panel();

    let mut shortest_path_len = i64::MAX;
    for code in expected_codes {
        let mut current_shortest_path_len = 0;
        for i in 0..code.len() {
            let start_code = if i == 0 { 'A' } else { code[i - 1] };
            let final_code = code[i];
            if let Some(&path_len) = shortest_path_len_map.get(&(start_code, final_code, recursion_level)) {
                current_shortest_path_len += path_len;
            } else if let Some(&internal_shortest_path_len) = full_internal_shortest_path_map.get(&(start_code, final_code, recursion_level)) {
                shortest_path_len_map.insert((start_code, final_code, recursion_level), internal_shortest_path_len);
                current_shortest_path_len += internal_shortest_path_len;
            } else if let Some(path) = internal_shortest_path_map.get(&(start_code, final_code)) {
                let internal_shortest_path_len = search_shortest_path_len(&vec![path.clone()], &robot_panel, shortest_path_len_map,
                                                                          internal_shortest_path_map, full_internal_shortest_path_map, recursion_level + 1);
                shortest_path_len_map.insert((start_code, final_code, recursion_level), internal_shortest_path_len);
                current_shortest_path_len += internal_shortest_path_len;
            } else {
                let paths = find_multiple_shortest_paths(start_code, final_code, panel);
                let internal_shortest_path_len = search_shortest_path_len(&paths, &robot_panel, shortest_path_len_map,
                                                                          internal_shortest_path_map, full_internal_shortest_path_map, recursion_level + 1);
                shortest_path_len_map.insert((start_code, final_code, recursion_level), internal_shortest_path_len);
                current_shortest_path_len += internal_shortest_path_len;
            }
        }
        shortest_path_len = shortest_path_len.min(current_shortest_path_len);
    }

    shortest_path_len
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
            continue;
        }
        tasks.push_back((add_and_clone(&current_paths, '>'), (i, j + 1)));
        tasks.push_back((add_and_clone(&current_paths, 'v'), (i + 1, j)));
        tasks.push_back((add_and_clone(&current_paths, '<'), (i, j - 1)));
        tasks.push_back((add_and_clone(&current_paths, '^'), (i - 1, j)));
    }
    let final_position = get_position(&panel, final_code);
    let paths = path_panel[final_position.0][final_position.1].clone();
    let paths = paths.iter().map(|x| {
        let mut path = x.clone();
        path.push('A');
        path
    }).collect::<Vec<_>>();

    paths
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

fn get_number(code: &Vec<char>) -> i64 {
    let re = Regex::new(r"^0*(\d+)").unwrap();
    let code = code.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("");
    if let Some(caps) = re.captures(&code) {
        *&caps[1].parse::<i64>().unwrap()
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
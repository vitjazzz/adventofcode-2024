use std::collections::{HashMap, VecDeque};
use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/20/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let map = get_map(&data);

    let path_map = search_exit(&map);

    let big_cheats = find_cheats(&map, &path_map, 100);

    let duration = start.elapsed();
    println!("\nResult: {}, Execution time: {:?}", big_cheats, duration);

    Ok(())
}

fn find_cheats(map: &Vec<Vec<char>>, path_map: &Vec<Vec<i32>>, min_save: i32) -> i32 {
    let start_position = get_start_position(&map);
    let mut tasks: VecDeque<(i32, (usize, usize))> = VecDeque::new();
    tasks.push_back((0, start_position));
    let mut sum = 0;
    while let (Some(task)) = tasks.pop_front() {
        let ( expected_value, (i, j)) = task;
        let current_time = path_map[i][j];
        if current_time != expected_value {
            continue;
        }
        sum += find_good_cheated_locations(path_map, (i, j), min_save);
        tasks.push_front((expected_value + 1, (i, j + 1)));
        tasks.push_front((expected_value + 1, (i + 1, j)));
        tasks.push_front((expected_value + 1, (i, j - 1)));
        tasks.push_front((expected_value + 1, (i - 1, j)));
    }
    sum
}

fn find_good_cheated_locations(path_map: &Vec<Vec<i32>>, (i, j): (usize, usize), min_save: i32) -> i32 {
    let mut cheated_locations = vec![
        (i + 2, j), (i, j + 2),
        (i - 1, j - 1), (i - 1, j + 1), (i + 1, j - 1), (i + 1, j + 1),
    ];
    if i >= 2 {
        cheated_locations.push((i - 2, j));
    }
    if j >= 2 {
        cheated_locations.push((i, j - 2));
    }
    let mut sum = 0;
    let current_time = path_map[i][j];
    for (i, j) in cheated_locations.iter() {
        if *i >= path_map.len() || *j >= path_map[0].len() {
            continue;
        }
        let time = path_map[*i][*j];
        if time == i32::MAX {
            continue;
        }
        if time - current_time - 2 >= min_save {
            sum += 1;
        }
    }

    sum
}

fn search_exit(map: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let mut scores_map = build_scores_map(&map);
    let start_position = get_start_position(&map);
    let mut tasks: VecDeque<(i32, (usize, usize))> = VecDeque::new();
    tasks.push_back((0, start_position));
    while let Some(task) = tasks.pop_front() {
        let (current_score, (i, j)) = task;
        let current_element = map[i][j];
        if current_element == '#' {
            continue;
        }
        let stored_score = scores_map[i][j];
        if stored_score <= current_score {
            continue;
        }
        scores_map[i][j] = current_score;
        tasks.push_back((current_score + 1, (i, j + 1)));
        tasks.push_back((current_score + 1, (i + 1, j)));
        tasks.push_back((current_score + 1, (i, j - 1)));
        tasks.push_back((current_score + 1, (i - 1, j)));
    }
    let mut final_path_map = build_scores_map(&map);
    let exit_position = get_exit_position(&map);
    let mut tasks: VecDeque<(i32, (usize, usize))> = VecDeque::new();
    tasks.push_back((scores_map[exit_position.0][exit_position.1], exit_position));
    while let (Some(task)) = tasks.pop_front() {
        let (expected_score, (i, j)) = task;
        let actual_score = scores_map[i][j];
        if actual_score != expected_score {
            continue;
        }
        final_path_map[i][j] = actual_score;
        tasks.push_back((expected_score - 1, (i, j + 1)));
        tasks.push_back((expected_score - 1, (i + 1, j)));
        tasks.push_back((expected_score - 1, (i, j - 1)));
        tasks.push_back((expected_score - 1, (i - 1, j)));
    }
    final_path_map
}

fn build_scores_map(map: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    map.iter()
        .map(|s| s.iter().map(|_| i32::MAX).collect::<Vec<i32>>())
        .collect()
}

fn get_start_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    return (0, 0);
}

fn get_exit_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'E' {
                return (i, j);
            }
        }
    }
    return (0, 0);
}

fn get_map(data: &Vec<String>) -> Vec<Vec<char>> {
    data.iter()
        .filter(|s| s.contains('#'))
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect()
}

fn test_data() -> Vec<String> {
    r"  ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

fn print_map(map: &Vec<Vec<char>>, path_map: &Vec<Vec<i32>>) {
    for i in 0..map.len() {
        println!();
        for j in 0..map[0].len() {
            if map[i][j] == '#' || map[i][j] == 'S' || map[i][j] == 'E' {
                print!("{}", map[i][j])
            } else if path_map[i][j] != i32::MAX {
                print!("0");
            } else {
                print!(".");
            }
        }
    }
}
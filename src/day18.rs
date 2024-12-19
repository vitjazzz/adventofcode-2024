use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::iter::Map;
use advent_tools::fetch_data;
use tokio::time::Instant;

// const SIZE: usize = 6;
const SIZE: usize = 70;

// 2147483647 is too high
pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/18/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    for i in 0..data.len() {
        let mut map = get_map(&data);
        corrupt_map(&mut map, &data, i);
        let res = find_shortest_path(&map);
        if res == i32::MAX {
            println!("Result: {}, Execution time: {:?}", data[i-1], start.elapsed());
            break;
        }
    }
    Ok(())
}

fn find_shortest_path(map: &Vec<Vec<char>>) -> i32 {
    let mut scores_map: Vec<Vec<i32>> = vec![vec![i32::MAX; map.len()]; map[0].len()];
    let mut tasks: VecDeque<(i32, (usize, usize))> = VecDeque::new();
    tasks.push_back((0, (1, 1)));
    while let (Some(task)) = tasks.pop_front() {
        let (current_score, (i, j)) = task;
        let current_element = map[i][j];
        if current_element == '#' {
            continue;
        }
        let stored_score = &scores_map[i][j];
        if *stored_score <= current_score {
            continue;
        }
        scores_map[i][j] = current_score;
        tasks.push_back((current_score + 1, (i, j + 1)));
        tasks.push_back((current_score + 1, (i + 1, j)));
        tasks.push_back((current_score + 1, (i, j - 1)));
        tasks.push_back((current_score + 1, (i - 1, j)));
    }

    scores_map[SIZE+1][SIZE+1]
}

fn corrupt_map(map: &mut Vec<Vec<char>>, data: &Vec<String>, bytes: usize) {
    for i in 0..bytes {
        let mut iter = data[i].split(",");
        let x: usize = iter.next().unwrap().trim().parse().unwrap();
        let y: usize = iter.next().unwrap().trim().parse().unwrap();
        map[y + 1][x + 1] = '#';
    }
}

fn get_map(data: &Vec<String>) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![vec!['.'; SIZE + 1]; SIZE + 1];
    for i in 0..map.len() {
        map[i].push('#');
        map[i].insert(0, '#');
    }
    let row_length = map[0].len();
    map.push(vec!['#'; row_length]);
    map.insert(0, vec!['#'; row_length]);
    return map;
}
fn test_data() -> Vec<String> {
    r"  5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
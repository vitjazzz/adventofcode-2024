use std::collections::HashMap;
use std::error::Error;
use advent_tools::fetch_data;
use itertools::Itertools;

const UP: u32 = 0b1000;
const RIGHT: u32 = 0b0100;
const DOWN: u32 = 0b0010;
const LEFT: u32 = 0b0001;

// const DIRECTIONS: HashMap<u32, (i32, i32)> = build_directions();
const DIRECTIONS: &[(u32, (i32, i32))] = &[
    (UP, (-1, 0)),
    (RIGHT, (0, 1)),
    (DOWN, (1, 0)),
    (LEFT, (0, -1))
];

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/6/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let map = get_map(&data);
    let mut visit_map = build_visit_map(&map);

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                move_guard(&map, &mut visit_map, i, j, UP);
            }
        }
    }

    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if visit_map[i][j] > 0 {
                res += 1;
            }
        }
    }

    println!("res: {}", res + 1);

    Ok(())
}

fn move_guard(map: &Vec<Vec<char>>, visit_map: &mut Vec<Vec<u32>>, i: usize, j: usize, direction: u32) {
    let (i_dir, j_dir) = get_direction_change(&direction);
    let next_i = (i as i32 + i_dir) as usize;
    let next_j = (j as i32 + j_dir) as usize;

    let next_symbol = map[next_i][next_j];
    if next_symbol == 'E' {
        return;
    } else if next_symbol == '#' {
        visit_map[i][j] += 1;
        let new_direction = change_direction(direction);
        move_guard(map, visit_map, i, j, new_direction);
    } else {
        visit_map[i][j] += 1;
        move_guard(map, visit_map, next_i, next_j, direction);
    }
}

fn change_direction(direction: u32) -> u32 {
    let new_direction = direction >> 1;
    if new_direction == 0 { UP } else { new_direction }
}

fn get_map(data: &Vec<String>) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = data.iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(|mut row| {
            row.push('E');
            row.insert(0, 'E');
            return row;
        })
        .collect();
    let row_length = map[0].len();
    map.push(vec!['E'; row_length]);
    map.insert(0, vec!['E'; row_length]);
    return map;
}

fn build_visit_map(map: &Vec<Vec<char>>) -> Vec<Vec<u32>> {
    map.iter()
        .map(|row| row.iter().map(|_| 0).collect())
        .collect()
}

fn get_direction_change(direction: &u32) -> (i32, i32) {
    DIRECTIONS.iter()
        .filter(|&&entry| entry.0 == *direction)
        .map(|&entry|entry.1)
        .next()
        .unwrap()
}

fn test_data() -> Vec<String> {
    r"  ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#..."
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}
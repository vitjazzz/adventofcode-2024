use std::collections::HashMap;
use std::error::Error;
use advent_tools::fetch_data;
use itertools::Itertools;

const UP: u32 = 0b1000;
const RIGHT: u32 = 0b0100;
const DOWN: u32 = 0b0010;
const LEFT: u32 = 0b0001;

const DIRECTIONS: &[(u32, (i32, i32))] = &[
    (UP, (-1, 0)),
    (RIGHT, (0, 1)),
    (DOWN, (1, 0)),
    (LEFT, (0, -1))
];

// 1836 - too high
// 1690 - too high

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/6/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let map = get_map(&data);

    let mut startin_visit_map = build_visit_map(&map);
    let (i, j) = find_guard_starting_point(&map).unwrap();
    let res = move_guard_initial(map, &mut startin_visit_map, i, j, UP);
    println!("res: {}", res);


    Ok(())
}

fn move_guard_initial(map: Vec<Vec<char>>, starting_visit_map: &mut Vec<Vec<u32>>, i: usize, j: usize, direction: u32) -> i32 {
    let (next_i, next_j) = get_next_coordinates(i, j, &direction);
    let next_symbol = map[next_i][next_j];

    if next_symbol == 'E' {
        starting_visit_map[i][j] = starting_visit_map[i][j] | direction;
        return 0;
    } else if next_symbol == '#' {
        starting_visit_map[i][j] = starting_visit_map[i][j] | direction;
        let new_direction = change_direction(direction);
        return move_guard_initial(map, starting_visit_map, i, j, new_direction);
    } else if next_symbol == '^' {
        starting_visit_map[i][j] = starting_visit_map[i][j] | direction;
        return move_guard_initial(map, starting_visit_map, next_i, next_j, direction);
    } else {
        if starting_visit_map[next_i][next_j] > 0 {
            starting_visit_map[i][j] = starting_visit_map[i][j] | direction;
            return move_guard_initial(map, starting_visit_map, next_i, next_j, direction);
        } else {
            let mut map_with_obstruction = map.clone();
            map_with_obstruction[next_i][next_j] = '0';
            let mut visit_map = starting_visit_map.clone();
            let ends_with_loop = move_guard_with_obstruction(&map_with_obstruction, &mut visit_map, i, j, direction);
            let res = if ends_with_loop { 1 } else { 0 };
            starting_visit_map[i][j] = starting_visit_map[i][j] | direction;
            return res + move_guard_initial(map, starting_visit_map, next_i, next_j, direction);
        }
    }
}

fn move_guard_with_obstruction(map: &Vec<Vec<char>>, visit_map: &mut Vec<Vec<u32>>, i: usize, j: usize, direction: u32) -> bool {
    let current_bitmap = visit_map[i][j];
    if current_bitmap & direction > 0 {
        return true;
    }
    visit_map[i][j] = current_bitmap | direction;

    let (next_i, next_j) = get_next_coordinates(i, j, &direction);
    let next_symbol = map[next_i][next_j];

    if next_symbol == 'E' {
        return false;
    } else if next_symbol == '#' || next_symbol == '0' {
        let new_direction = change_direction(direction);
        return  move_guard_with_obstruction(map, visit_map, i, j, new_direction);
    } else {
        return move_guard_with_obstruction(map, visit_map, next_i, next_j, direction);
    }
}

fn get_next_coordinates(i: usize, j: usize, direction: &u32) -> (usize, usize) {
    let (i_dir, j_dir) = get_direction_change(&direction);
    let next_i = (i as i32 + i_dir) as usize;
    let next_j = (j as i32 + j_dir) as usize;
    (next_i, next_j)
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

fn find_guard_starting_point(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                return Some((i, j))
            }
        }
    }
    None
}

fn get_direction_change(direction: &u32) -> (i32, i32) {
    DIRECTIONS.iter()
        .filter(|&&entry| entry.0 == *direction)
        .map(|&entry|entry.1)
        .next()
        .unwrap()
}

fn test_data() -> Vec<String> {
    r"....#.....
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
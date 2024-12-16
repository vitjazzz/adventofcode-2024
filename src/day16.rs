use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;

// 583 is too high
pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/16/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let map = get_map(&data);
    let mut scores_map = build_scores_map(&map);

    search_exit(&map, &mut scores_map);

    let exit_position = get_exit_position(&map);
    let min_score = scores_map[exit_position.0][exit_position.1].values().min().unwrap();

    let duration = start.elapsed();
    println!("\nResult: {}, Execution time: {:?}", min_score, duration);

    Ok(())
}

fn search_exit(map: &Vec<Vec<char>>,
               scores_map: &mut Vec<Vec<HashMap<char, i32>>>) {
    let reindeer_position = get_reindeer_position(&map);
    let mut tasks: VecDeque<(i32, char, (usize, usize))> = VecDeque::new();
    tasks.push_back((0, '>', reindeer_position));
    while let (Some(task)) = tasks.pop_front() {
        let (current_score, direction, (i, j)) = task;
        let current_element = map[i][j];
        if current_element == '#' {
            continue;
        }
        let mut stored_scores = &mut scores_map[i][j];
        if let Some(score) = stored_scores.get(&direction) {
            if *score <= current_score {
                continue;
            }
        }
        stored_scores.insert(direction, current_score);
        if current_element == 'E' {
            continue;
        }
        let move_position = match direction {
            '>' => (i, j + 1),
            'v' => (i + 1, j),
            '<' => (i, j - 1),
            '^' => (i - 1, j),
            _ => (i, j)
        };
        let left_direction = match direction {
            '>' => '^',
            'v' => '>',
            '<' => 'v',
            '^' => '<',
            _ => direction
        };
        let right_direction = match direction {
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            '^' => '>',
            _ => direction
        };
        tasks.push_back((current_score + 1, direction, move_position));
        tasks.push_back((current_score + 1000, left_direction, (i, j)));
        tasks.push_back((current_score + 1000, right_direction, (i, j)));
    }
}

fn build_scores_map(map: &Vec<Vec<char>>) -> Vec<Vec<HashMap<char, i32>>> {
    map.iter()
        .map(|s| s.iter().map(|_| HashMap::new()).collect::<Vec<HashMap<char, i32>>>())
        .collect()
}

fn get_reindeer_position(map: &Vec<Vec<char>>) -> (usize, usize) {
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
    // r"  ###############
    //     #.......#....E#
    //     #.#.###.#.###.#
    //     #.....#.#...#.#
    //     #.###.#####.#.#
    //     #.#.#.......#.#
    //     #.#.#####.###.#
    //     #...........#.#
    //     ###.#.#####.#.#
    //     #...#.....#.#.#
    //     #.#.#.###.#.#.#
    //     #.....#...#.#.#
    //     #.###.#.#.#.#.#
    //     #S..#.....#...#
    //     ###############"
    r"  #################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

fn print_map(map: &Vec<Vec<char>>, scores_map: &Vec<Vec<HashMap<char, i32>>>) {
    for i in 0..map.len() {
        println!();
        for j in 0..map[0].len() {
            if map[i][j] == '#' || map[i][j] == 'S' || map[i][j] == 'E' {
                print!("{}", map[i][j])
            } else {
                let mut best_direction = '.';
                let mut best_score = i32::MAX;
                for (direction, score) in &scores_map[i][j] {
                    if *score < best_score {
                        best_direction = *direction;
                        best_score = *score;
                    }
                }
                print!("{}", best_direction);
            }
        }
    }
}

fn print_map_part_2(map: &Vec<Vec<char>>, shortest_path_to_end_map: &Vec<Vec<HashMap<char, i32>>>, min_score: i32) {
    for i in 0..map.len() {
        println!();
        for j in 0..map[0].len() {
            if map[i][j] == '#' || map[i][j] == 'S' || map[i][j] == 'E' {
                print!("{}", map[i][j])
            } else {
                let c = shortest_path_to_end_map[i][j].values()
                    .filter(|&score| *score == min_score)
                    .map(|_| 'O')
                    .next()
                    .unwrap_or('.');
                print!("{}", c);
            }
        }
    }
}
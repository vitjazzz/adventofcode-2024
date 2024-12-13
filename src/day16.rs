use std::cmp::min;
use std::collections::HashMap;
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
    let mut shortest_path_to_end_map = build_shortest_path_to_end_map(&map);
    let reindeer_position = get_reindeer_position(&map);

    search_exit(0, reindeer_position, '>', &map, &mut scores_map, &mut shortest_path_to_end_map);

    let exit_position = get_exit_position(&map);
    let min_score = scores_map[exit_position.0][exit_position.1].values().min().unwrap();
    let score = shortest_path_to_end_map.iter()
        .flat_map(|v| v)
        .filter(|&o| o.values().filter(|&val| val == min_score).count() > 0)
        .count();
    // print_map_part_2(&map, &shortest_path_to_end_map, *min_score);

    let duration = start.elapsed();
    println!("\nResult: {}, Execution time: {:?}", score, duration);

    Ok(())
}

fn search_exit(current_score: i32, position: (usize, usize), direction: char, map: &Vec<Vec<char>>,
               scores_map: &mut Vec<Vec<HashMap<char, i32>>>,
               shortest_path_to_end_map: &mut Vec<Vec<HashMap<char, i32>>>) -> Option<i32> {
    let current_element = map[position.0][position.1];
    if current_element == '#' {
        return None;
    }
    let mut stored_scores = &mut scores_map[position.0][position.1];
    if let Some(score) = stored_scores.get(&direction) {
       if *score < current_score {
           return None;
       } else if *score == current_score && shortest_path_to_end_map[position.0][position.1].contains_key(&direction) {
           return Some(*shortest_path_to_end_map[position.0][position.1].get(&direction).unwrap());
       }
    }
    stored_scores.insert(direction, current_score);
    if current_element == 'E' {
        if stored_scores.values().filter(|&score|* score < current_score).count() > 0 {
            return None;
        } else {
            shortest_path_to_end_map[position.0][position.1].insert(direction, current_score);
            return Some(current_score);
        }
    }

    match direction {
        '>' => {
            let score1 = search_exit(current_score + 1, (position.0, position.1 + 1), direction, map, scores_map, shortest_path_to_end_map);
            let score2 =search_exit(current_score + 1000, position, '^', map, scores_map, shortest_path_to_end_map);
            let score3 =search_exit(current_score + 1000, position, 'v', map, scores_map, shortest_path_to_end_map);
            let min_score = min(score1.unwrap_or(i32::MAX), min(score2.unwrap_or(i32::MAX), score3.unwrap_or(i32::MAX)));
            if min_score == i32::MAX {
                None
            } else {
                shortest_path_to_end_map[position.0][position.1].insert(direction, min_score);
                Some(min_score)
            }
        }
        'v' => {
            let score1 = search_exit(current_score + 1, (position.0 + 1, position.1), direction, map, scores_map, shortest_path_to_end_map);
            let score2 = search_exit(current_score + 1000, position, '>', map, scores_map, shortest_path_to_end_map);
            let score3 = search_exit(current_score + 1000, position, '<', map, scores_map, shortest_path_to_end_map);
            let min_score = min(score1.unwrap_or(i32::MAX), min(score2.unwrap_or(i32::MAX), score3.unwrap_or(i32::MAX)));
            if min_score == i32::MAX {
                None
            } else {
                shortest_path_to_end_map[position.0][position.1].insert(direction, min_score);
                Some(min_score)
            }
        }
        '<' => {
            let score1 = search_exit(current_score + 1, (position.0, position.1 - 1), direction, map, scores_map, shortest_path_to_end_map);
            let score2 = search_exit(current_score + 1000, position, 'v', map, scores_map, shortest_path_to_end_map);
            let score3 = search_exit(current_score + 1000, position, '^', map, scores_map, shortest_path_to_end_map);
            let min_score = min(score1.unwrap_or(i32::MAX), min(score2.unwrap_or(i32::MAX), score3.unwrap_or(i32::MAX)));
            if min_score == i32::MAX {
                None
            } else {
                shortest_path_to_end_map[position.0][position.1].insert(direction, min_score);
                Some(min_score)
            }
        }
        '^' => {
            let score1 = search_exit(current_score + 1, (position.0 - 1, position.1), direction, map, scores_map, shortest_path_to_end_map);
            let score2 = search_exit(current_score + 1000, position, '<', map, scores_map, shortest_path_to_end_map);
            let score3 = search_exit(current_score + 1000, position, '>', map, scores_map, shortest_path_to_end_map);
            let min_score = min(score1.unwrap_or(i32::MAX), min(score2.unwrap_or(i32::MAX), score3.unwrap_or(i32::MAX)));
            if min_score == i32::MAX {
                None
            } else {
                shortest_path_to_end_map[position.0][position.1].insert(direction, min_score);
                Some(min_score)
            }
        }
        _ => {
            return None;
        }
    }

    // move
    // turn left
    // turn right
}

fn build_scores_map(map: &Vec<Vec<char>>) -> Vec<Vec<HashMap<char, i32>>> {
    map.iter()
        .map(|s| s.iter().map(|_| HashMap::new()).collect::<Vec<HashMap<char, i32>>>())
        .collect()
}


fn build_shortest_path_to_end_map(map: &Vec<Vec<char>>) -> Vec<Vec<HashMap<char, i32>>> {
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
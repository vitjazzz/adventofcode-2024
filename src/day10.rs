use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;


pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/10/input";
    let data = fetch_data(url).await?;
    let data = test_data();

    let start = Instant::now();
    let map = get_map(&data);
    let mut score_map = build_score_map(&map);

    let mut score = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                score += calculate_score(-1, i, j, &map, &mut score_map);
            }
        }
    }

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", score, duration);

    Ok(())
}

fn calculate_score(prev_value: i32, i: usize, j: usize, map: &Vec<Vec<i32>>, score_map: &mut Vec<Vec<i32>>) -> i32 {
    let current_val = map[i][j];
    if current_val - prev_value != 1 {
        return 0;
    }
    if current_val == 9 {
        return 1;
    }
    if score_map[i][j] != -1 {
        return score_map[i][j];
    }
    let score = calculate_score(current_val, i - 1, j, map, score_map)
        + calculate_score(current_val, i, j + 1, map, score_map)
        + calculate_score(current_val, i + 1, j, map, score_map)
        + calculate_score(current_val, i, j - 1, map, score_map);
    score_map[i][j] = score;
    return score;
}

fn build_score_map(map: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    map.iter()
        .map(|s| s.iter().map(|_| -1).collect::<Vec<i32>>())
        .collect()
}

fn get_map(data: &Vec<String>) -> Vec<Vec<i32>> {
    let mut map: Vec<Vec<i32>> = data.iter()
        .map(|s| s.chars()
            .map(|c| {
                if c != '.' { c as i32 - '0' as i32 } else { -1 }
            })
            .collect::<Vec<i32>>())
        .map(|mut row| {
            row.push(-1);
            row.insert(0, -1);
            return row;
        })
        .collect();
    let row_length = map[0].len();
    map.push(vec![-1; row_length]);
    map.insert(0, vec![-1; row_length]);
    return map;
}

fn test_data() -> Vec<String> {
    // r"  89010123
    //     78121874
    //     87430965
    //     96549874
    //     45678903
    //     32019012
    //     01329801
    //     10456732"
    r" ..90..9
...1.98
...2..7
6543456
765.987
876....
987...."
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}
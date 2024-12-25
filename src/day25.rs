use std::error::Error;
use advent_tools::fetch_data;
use itertools::Itertools;
use tokio::time::Instant;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/25/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let keys = get_keys(&data);
    let locks = get_locks(&data);
    let pairs = find_pairs(&keys, &locks);

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", pairs.len(), duration);

    Ok(())
}

fn find_pairs<'a>(keys: &'a Vec<Vec<i32>>, locks: &'a Vec<Vec<i32>>) -> Vec<(&'a Vec<i32>, &'a Vec<i32>)> {
    let mut pairs = Vec::new();
    for key in keys.iter() {
        for lock in locks.iter() {
            if matches(key, lock) {
                pairs.push((lock, key));
            }
        }
    }

    pairs
}

fn matches(key: &Vec<i32>, lock: &Vec<i32>) -> bool {
    for i in 0..key.len() {
        if key[i] + lock[i] > 5 {
            return false
        }
    }
    return true
}

fn get_keys(data: &Vec<String>) -> Vec<Vec<i32>> {
    group_by_separator(data).iter()
        .filter_map(|block| {
            if block[0].contains('#') {
                return None; // Skip locks
            }

            let mut heights = vec![0; block[0].len()];

            for (i, line) in block.iter().rev().enumerate() { // Start from the bottom
                for (j, &ch) in line.as_bytes().iter().enumerate() {
                    if ch == b'#' {
                        heights[j] = i as i32 ; // Record height (0-based index)
                    }
                }
            }

            Some(heights)
        })
        .collect()
}

fn get_locks(data: &Vec<String>) -> Vec<Vec<i32>> {
    group_by_separator(data).iter()
        .filter_map(|block| {
            if block[0].contains('.') {
                return None; // Skip keys
            }

            let mut heights = vec![0; block[0].len()];

            for (i, line) in block.iter().enumerate() { // Start from the bottom
                for (j, &ch) in line.as_bytes().iter().enumerate() {
                    if ch == b'#' {
                        heights[j] = i as i32 ; // Record height (0-based index)
                    }
                }
            }

            Some(heights)
        })
        .collect()
}

fn test_data() -> Vec<String> {
    r"  #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

fn group_by_separator(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut groups = Vec::new();
    let mut current_group = Vec::new();

    for line in lines {
        if line.is_empty() {
            if !current_group.is_empty() {
                groups.push(current_group);
                current_group = Vec::new();
            }
        } else {
            current_group.push(line.clone());
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}
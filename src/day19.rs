use std::collections::{HashMap, VecDeque};
use std::error::Error;
use advent_tools::fetch_data;
use itertools::Itertools;
use tokio::time::Instant;
use trie_rs::{Trie};

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/19/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();

    let patterns = get_available_patterns(&data);
    let patterns_trie = build_trie(&patterns);

    let desired_designs = get_designs(&data);
    let max_pattern_len = patterns.iter().map(|s| s.len()).max().unwrap() as usize;

    let res: i64 = desired_designs.iter()
        .map(|design| calculate_possible(&patterns_trie, design))
        .sum();

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", res, duration);

    Ok(())
}

fn calculate_possible(patterns: &Trie<u8>, design: &str) -> i64 {
    let mut result_map: HashMap<&str, i64> = HashMap::new();
    let mut tasks: VecDeque<(usize)> = VecDeque::new();
    tasks.push_back((0));
    while let (Some(task)) = tasks.pop_front() {
        let (position) = task;
        if position > design.len() {
            break;
        }
        let current_design = &design[design.len() - position..];
        if current_design.is_empty() {
            result_map.insert(current_design, 1);
        } else {
            let matching_prefixes: Vec<String> = patterns.common_prefix_search(current_design).collect();
            let mut possible_patterns = 0;
            for matching_prefix in matching_prefixes {
                let remaining_design = &design[design.len() - (position - matching_prefix.len())..];
                if let Some(count) = result_map.get(remaining_design) {
                    possible_patterns += count;
                }
            }
            result_map.insert(current_design, possible_patterns);
        }
        tasks.push_back((position + 1));
    }
    *result_map.get(design).unwrap_or(&0)
}

fn build_trie(patterns: &Vec<String>) -> Trie<u8> {
    Trie::from_iter(patterns.iter().map(|s| s.as_str()))
}

fn get_designs(data: &Vec<String>) -> Vec<String> {
    data.iter()
        .filter(|s| !s.contains(',') && !s.is_empty())
        .map(String::from)
        .collect()
}

fn get_available_patterns(data: &Vec<String>) -> Vec<String> {
    data.iter()
        .filter(|s| s.contains(','))
        .flat_map(|s| s.split(", "))
        .map(String::from)
        .collect()
}

fn test_data() -> Vec<String> {
    r"  r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}
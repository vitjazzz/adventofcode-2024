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

    let res = desired_designs.iter()
        .filter(|design| {
            println!("Checking design: {}", design);
            is_possible(&patterns_trie, design, max_pattern_len)
        })
        .count();

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", res, duration);

    Ok(())
}

fn build_trie(patterns: &Vec<String>) -> Trie<u8> {
    Trie::from_iter(patterns.iter().map(|s| s.as_str()))
}

fn is_possible(patterns: &Trie<u8>, design: &str, max_pattern_len: usize) -> bool {
    if design.is_empty() {
        return true;
    }
    // let matching_prefixes: Vec<String> = patterns.common_prefix_search(design).collect();
    for i in 1..max_pattern_len + 1 {
        if i > design.len() {
            break;
        }
        let possible_pattern = &design[design.len() - i..];
        let pattern_match = patterns.exact_match(possible_pattern);
        if pattern_match && is_possible(patterns, &design[..design.len()-i], max_pattern_len) {
            return true;
        }
    }
    false
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
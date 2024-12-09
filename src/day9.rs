use std::cmp::{max, min};
use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;


pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/9/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let disk_fragments = get_disk_fragments(&data[0]);
    // println!("{:?}", disk_fragments);

    let checksum = move_and_calculate_checksum(disk_fragments);

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", checksum, duration);

    Ok(())
}

fn move_and_calculate_checksum(mut disk_fragments: Vec<DiskFragment>) -> i64 {
    let mut right_disk_fragments = disk_fragments.clone();
    let mut i = 0;
    let mut j = disk_fragments.len() - 1;
    let mut checksum = 0;
    let mut index = 0;
    while i <= j {
        let mut left = &mut disk_fragments[i];
        let used_space = right_disk_fragments[i].total_space - right_disk_fragments[i].free_space;
        checksum += calculate_current_checksum(index, left.id, used_space);
        index += used_space;
        while left.free_space != 0 && i < j  {
            let mut right = &mut right_disk_fragments[j];
            let used_space_to_move = min(right.total_space - right.free_space, left.free_space);
            right.free_space += used_space_to_move;
            left.free_space -= used_space_to_move;
            checksum += calculate_current_checksum(index, right.id, used_space_to_move);
            index += used_space_to_move;
            if right.free_space == right.total_space {
                j -= 1;
                continue
            }
        }
        i += 1;
    }
    checksum
}

fn calculate_current_checksum(index: i64, id: i64, used_space: i64) -> i64 {
    id * used_space * (2 * index + used_space - 1) / 2
}

fn get_disk_fragments(s: &String) -> Vec<DiskFragment> {
    let mut res = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    for i in 0..(chars.len() / 2) + 1 {
        let index = i * 2;
        let free_space = if index + 1 < chars.len() { (chars[index + 1] as i64) - ('0' as i64) } else { 0 } ;
        let total_space = free_space + (chars[index] as i64) - ('0' as i64);
        res.push(DiskFragment {
            id: i as i64,
            total_space,
            free_space
        })
    }
    res
}

fn test_data() -> Vec<String> {
    r"2333133121414131402"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

#[derive(Debug, Clone)]
struct DiskFragment {
    id: i64,
    total_space: i64,
    free_space: i64
}
use std::error::Error;
use std::fmt::format;
use advent_tools::fetch_data;
use regex::Regex;

const XMAS: &str = "XMAS";

pub async fn execute() -> Result<(), Box<dyn Error>> {
    // 2596 - too high
    let url = "https://adventofcode.com/2024/day/4/input";
    let data = fetch_data(url).await?;
    // let data: Vec<String> = r"MMMSXXMASM
    //                     MSAMXMSMSA
    //                     AMXSXMAAMM
    //                     MSAMASMSMX
    //                     XMASAMXAMM
    //                     XXAMMXXAMA
    //                     SMSMSASXSS
    //                     SAXAMASAAA
    //                     MAMMMXMMMM
    //                     MXMXAXMASX"
    //     .lines()
    //     .map(|s| s.trim().to_string())
    //     .collect();
    // let data: Vec<String> = r"....XXMAS.
    //                         .SAMXMS...
    //                         ...S..A...
    //                         ..A.A.MS.X
    //                         XMASAMX.MM
    //                         X.....XA.A
    //                         S.S.S.S.SS
    //                         .A.A.A.A.A
    //                         ..M.M.M.MM
    //                         .X.X.XMASX"
    //     .lines()
    //     .map(|s| s.trim().to_string())
    //     .collect();

    let mut data_arr: Vec<Vec<char>> = data.into_iter()
        .map(|s| {
            let mut chars: Vec<char> = s.chars().collect();
            chars.push('*');
            chars.insert(0, '*');
            return chars;
        })
        .collect();

    let row_length = data_arr[0].len();
    data_arr.push(vec!['*'; row_length]);
    data_arr.insert(0, vec!['*'; row_length]);

    let mut xmas_count = 0;
    for i in 0..data_arr.len() {
        for j in 0..data_arr[0].len() {
            xmas_count += count_xmas(i, j, &data_arr, 0, -1, -1);
            xmas_count += count_xmas(i, j, &data_arr, 0, -1, 0);
            xmas_count += count_xmas(i, j, &data_arr, 0, -1, 1);
            xmas_count += count_xmas(i, j, &data_arr, 0, 0, -1);
            xmas_count += count_xmas(i, j, &data_arr, 0, 0, 1);
            xmas_count += count_xmas(i, j, &data_arr, 0, 1, -1);
            xmas_count += count_xmas(i, j, &data_arr, 0, 1, 0);
            xmas_count += count_xmas(i, j, &data_arr, 0, 1, 1);
        }
    }
    print(&data_arr);
    println!("xmas_count: {}", xmas_count);

    Ok(())
}

fn count_xmas(i: usize, j: usize, data_arr: &Vec<Vec<char>>, xmas_char_index: usize, i_dir: i32, j_dir: i32) -> i32 {
    if i < 0 || j < 0 || i >= data_arr.len() || j >= data_arr[i].len() {
        return 0;
    }
    let current_char = data_arr[i][j];
    let expected_char = XMAS.chars().nth(xmas_char_index).unwrap();
    if current_char != expected_char {
        return 0;
    }

    let next_xmas_char_index = xmas_char_index + 1;
    if next_xmas_char_index == XMAS.len() {
        return 1;
    }
    let next_i = (i as i32 + i_dir) as usize;
    let next_j = (j as i32 + j_dir) as usize;
    let res = count_xmas(next_i, next_j, data_arr, next_xmas_char_index, i_dir, j_dir);

    res
}

fn print(arr: &Vec<Vec<char>>) {
    for row in arr {
        let row: String = row.iter().collect();
        println!("{}", row)
    }
}
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

    let mut data_heat_map: Vec<Vec<char>> = data_arr.iter()
        .map(|row| row.iter().map(|_| '0').collect())
        .collect();

    let mut xmas_count = 0;
    for i in 0..data_arr.len() {
        for j in 0..data_arr[0].len() {
            xmas_count += count_xmas(i, j, &data_arr, &mut data_heat_map, 0, 0b11111111);
        }
    }
    print(&data_heat_map);
    print(&data_arr);
    println!("xmas_count: {}", xmas_count);

    Ok(())
}

fn count_xmas(i: usize, j: usize, data_arr: &Vec<Vec<char>>, data_heat_map: &mut Vec<Vec<char>>, xmas_char_index: usize, direction_bitmap: u32) -> i32 {
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
        let new_heat_value = char::from_u32((data_heat_map[i][j] as u32) + 1).unwrap();
        data_heat_map[i][j] = new_heat_value;
        return 1;
    }
    let res = if any_bit_set(direction_bitmap & 0b10000000) { count_xmas(i - 1, j - 1, data_arr, data_heat_map, next_xmas_char_index, 0b10000000) } else { 0 }
        + if any_bit_set(direction_bitmap & 0b01000000) { count_xmas(i - 1, j, data_arr, data_heat_map, next_xmas_char_index, 0b01000000) } else { 0 }
        + if any_bit_set(direction_bitmap & 0b00100000) { count_xmas(i - 1, j + 1, data_arr, data_heat_map, next_xmas_char_index, 0b00100000) } else { 0 }
        + if any_bit_set(direction_bitmap & 0b00010000) { count_xmas(i, j + 1, data_arr, data_heat_map, next_xmas_char_index, 0b00010000) } else { 0 }
        + if any_bit_set(direction_bitmap & 0b00001000) { count_xmas(i + 1, j + 1, data_arr, data_heat_map, next_xmas_char_index, 0b00001000) } else { 0 }
        + if any_bit_set(direction_bitmap & 0b00000100) { count_xmas(i + 1, j, data_arr, data_heat_map, next_xmas_char_index, 0b00000100) } else { 0 }
        + if any_bit_set(direction_bitmap & 0b00000010) { count_xmas(i + 1, j + 1, data_arr, data_heat_map, next_xmas_char_index, 0b00000010) } else { 0 }
        + if any_bit_set(direction_bitmap & 0b00000001) { count_xmas(i, j - 1, data_arr, data_heat_map, next_xmas_char_index, 0b00000001) } else { 0 };

    if res > 0 {
        let new_heat_value = char::from_u32((data_heat_map[i][j] as u32) + 1).unwrap();
        data_heat_map[i][j] = new_heat_value;
    }

    res
}

fn any_bit_set(value: u32) -> bool {
    value != 0
}

fn print(arr: &Vec<Vec<char>>) {
    for row in arr {
        let row: String = row.iter().collect();
        println!("{}", row)
    }
}
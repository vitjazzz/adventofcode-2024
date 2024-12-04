use std::error::Error;
use advent_tools::fetch_data;

const MAS: &str = "MAS";

pub async fn execute() -> Result<(), Box<dyn Error>> {
    // 2596 - too high
    let url = "https://adventofcode.com/2024/day/4/input";
    let data = fetch_data(url).await?;

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

    for i in 0..data_arr.len() {
        for j in 0..data_arr[0].len() {
            find_x_mas(i, j, &data_arr, &mut data_heat_map, 0, -1, -1);
            find_x_mas(i, j, &data_arr, &mut data_heat_map, 0, -1, 1);
            find_x_mas(i, j, &data_arr, &mut data_heat_map, 0, 1, -1);
            find_x_mas(i, j, &data_arr, &mut data_heat_map, 0, 1, 1);
        }
    }
    let mut xmas_count = 0;
    for i in 0..data_arr.len() {
        for j in 0..data_arr[0].len() {
            if data_arr[i][j] == 'A' && data_heat_map[i][j] == '2' {
                xmas_count += 1;
            }
        }
    }
    print(&data_heat_map);
    print(&data_arr);
    println!("x_mas_count: {}", xmas_count);

    Ok(())
}

fn find_x_mas(i: usize, j: usize, data_arr: &Vec<Vec<char>>, data_heat_map: &mut Vec<Vec<char>>, xmas_char_index: usize, i_dir: i32, j_dir: i32) -> i32 {
    if i < 0 || j < 0 || i >= data_arr.len() || j >= data_arr[i].len() {
        return 0;
    }
    let current_char = data_arr[i][j];
    let expected_char = MAS.chars().nth(xmas_char_index).unwrap();
    if current_char != expected_char {
        return 0;
    }

    let next_xmas_char_index = xmas_char_index + 1;
    if next_xmas_char_index == MAS.len() {
        data_heat_map[i][j] = char::from_u32((data_heat_map[i][j] as u32) + 1).unwrap();
        return 1;
    }
    let next_i = (i as i32 + i_dir) as usize;
    let next_j = (j as i32 + j_dir) as usize;
    let res = find_x_mas(next_i, next_j, data_arr, data_heat_map, next_xmas_char_index, i_dir, j_dir);

    if res > 0 {
        data_heat_map[i][j] = char::from_u32((data_heat_map[i][j] as u32) + 1).unwrap();
    }

    res
}

fn print(arr: &Vec<Vec<char>>) {
    for row in arr {
        let row: String = row.iter().collect();
        println!("{}", row)
    }
}
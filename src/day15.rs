use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;


pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/15/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let mut map = get_map(&data);
    let movements = get_movements(&data);
    let mut robot_position = get_robot_position(&map);

    for movement in movements {
        (robot_position, map) = try_move(robot_position, movement, map);
    }

    print_map(&map);
    let score = calculate_score(&map);

    let duration = start.elapsed();
    println!("\nResult: {}, Execution time: {:?}", score, duration);

    Ok(())
}

fn try_move(position: (usize, usize), movement: char, map: Vec<Vec<char>>) -> ((usize, usize), Vec<Vec<char>>) {
    let current_element = map[position.0][position.1];
    if current_element == '#' || current_element == '.' {
        return (position, map);
    }
    let expected_position = match movement {
        '^' => (position.0 - 1, position.1),
        '>' => (position.0, position.1 + 1),
        'v' => (position.0 + 1, position.1),
        '<' => (position.0, position.1 - 1),
        _ => (position.0, position.1)
    };
    let new_map = map.clone();
    let (_, mut new_map) = try_move(expected_position, movement, new_map);
    if new_map[expected_position.0][expected_position.1] == '.' {
        new_map[expected_position.0][expected_position.1] = current_element;
        new_map[position.0][position.1] = '.';
        if (movement == '^' || movement == 'v') && current_element == '[' {
            let right_position = (position.0, position.1 + 1);
            let (_, new_map) = try_move(right_position, movement, new_map);
            if new_map[right_position.0][right_position.1] == '.' {
                return (expected_position, new_map)
            }
        } else if (movement == '^' || movement == 'v') && current_element == ']' {
            let left_position = (position.0, position.1 - 1);
            let (_, new_map) = try_move(left_position, movement, new_map);
            if new_map[left_position.0][left_position.1] == '.' {
                return (expected_position, new_map)
            }
        } else {
            return (expected_position, new_map);
        }
    }
    return (position, map);
}

fn calculate_score(map: &Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '[' {
                res += (i * 100 + j) as i32;
            }
        }
    }
    res
}

fn get_robot_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '@' {
                return (i, j);
            }
        }
    }
    return (0, 0);
}

fn get_map(data: &Vec<String>) -> Vec<Vec<char>> {
    let original_map: Vec<Vec<char>> = data.iter()
        .filter(|s| s.contains('#'))
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let mut res: Vec<Vec<char>> = Vec::new();
    for i in 0..original_map.len() {
        res.push(Vec::new());
        for j in 0..original_map[0].len() {
            if original_map[i][j] == '@' {
                res[i].push('@');
                res[i].push('.');
            } else if original_map[i][j] == 'O' {
                res[i].push('[');
                res[i].push(']');
            } else {
                res[i].push(original_map[i][j]);
                res[i].push(original_map[i][j]);
            }
        }
    }
    res
}

fn get_movements(data: &Vec<String>) -> Vec<char> {
    data.iter()
        .filter(|s| s.contains('>') || s.contains('<') || s.contains('^') || s.contains('v'))
        .flat_map(|s| s.chars().collect::<Vec<char>>())
        .collect()
}

fn test_data() -> Vec<String> {
    // r"  #######
    //     #...#.#
    //     #.....#
    //     #..OO@#
    //     #..O..#
    //     #.....#
    //     #######
    //
    //     <vv<<^^<<^^"
        r"  ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}


fn print_map(map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        println!();
        for j in 0..map[0].len() {
            print!("{}", map[i][j])
        }
    }
}
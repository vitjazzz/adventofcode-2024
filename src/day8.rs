use std::collections::{HashMap, HashSet};
use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;


pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/8/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let map = get_map(&data);
    let locations = get_locations(&map);

    let antinodes: HashSet<(i32, i32)> = locations.into_iter()
        .map(|locations_entry|locations_entry.1)
        .flat_map(|locations_points| find_antinodes(&locations_points, map.len() as i32, map[0].len() as i32))
        .collect();

    // print_map(&map, &antinodes);
    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", antinodes.len(), duration);

    Ok(())
}

fn find_antinodes(locations: &Vec<(i32, i32)>, map_limit_i: i32, map_limit_j: i32) -> Vec<(i32, i32)> {
    let mut res: Vec<(i32, i32)> = Vec::new();
    for i in 0..locations.len() {
        for j in i+1..locations.len() {
            let point1 = locations[i];
            let point2 = locations[j];

            res.extend(find_antinodes_for_2_points(point1, point2, map_limit_i, map_limit_j));
        }
    }
    res
}

fn find_antinodes_for_2_points(point1: (i32, i32), point2: (i32, i32), map_limit_i: i32, map_limit_j: i32) -> Vec<(i32, i32)> {
    let mut res: Vec<(i32, i32)> = Vec::new();

    let x_delta = point2.1 - point1.1;
    let y_delta = point2.0 - point1.0;

    let mut antinode1 = point1;
    while inside_map(antinode1, map_limit_i, map_limit_j) {
        res.push(antinode1);
        antinode1 = (antinode1.0 - y_delta, antinode1.1 - x_delta);
    }

    let mut antinode2 = point2;
    while inside_map(antinode2, map_limit_i, map_limit_j) {
        res.push(antinode2);
        antinode2 = (antinode2.0 + y_delta, antinode2.1 + x_delta);
    }

    res
}

fn inside_map(point: (i32, i32), map_limit_i: i32, map_limit_j: i32) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < map_limit_i && point.1 < map_limit_j
}

fn get_map(data: &Vec<String>) -> Vec<Vec<char>> {
    data.iter()
        .map(|s| s.chars().collect())
        .collect()
}

fn get_locations(map: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut res: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let c = map[i][j];
            if c != '.' {
                res.entry(c)
                    .or_insert_with(Vec::new)
                    .push((i as i32, j as i32));
            }
        }
    }
    res
}

fn test_data() -> Vec<String> {
    r"  ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

fn print_map(map: &Vec<Vec<char>>, antinodes: &HashSet<(i32, i32)>) {
    println!();
    for i in 0..map.len() {
        println!();
        for j in 0..map[0].len() {
            let mut x = 0;
            if antinodes.contains(&(i as i32, j as i32)) && map[i][j] != '.' {
                x += 1;
            }
            if map[i][j] != '.' {
                print!("{}", map[i][j]);
            } else if antinodes.contains(&(i as i32, j as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
    println!();
}
use std::collections::{HashMap, HashSet};
use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;


pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/12/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let map = get_map(&data);
    let mut region_map: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' {
                continue;
            }
            fill_region_map((i, j), (i, j), &map, &mut region_map);
        }
    }
    let region_area_and_perimeter = calculate_area_and_perimeter(&region_map);

    let score: i32 = region_area_and_perimeter.values().into_iter()
        .map(|(area, perimeter)| area * perimeter)
        .sum();

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", score, duration);

    Ok(())
}

fn fill_region_map(current_region_point: (usize, usize), point: (usize, usize), map: &Vec<Vec<char>>, region_map: &mut HashMap<(usize, usize), (usize, usize)>) {
    let current_val = map[point.0][point.1];
    if current_val == '.' {
        return;
    }
    if let Some(_) = region_map.get(&point) {
        return;
    }
    let current_region_val = map[current_region_point.0][current_region_point.1];
    if current_val != current_region_val {
        return;
    }
    region_map.insert(point, current_region_point);

    fill_region_map(current_region_point, (point.0 - 1, point.1), map, region_map);
    fill_region_map(current_region_point, (point.0 + 1, point.1), map, region_map);
    fill_region_map(current_region_point, (point.0, point.1 - 1), map, region_map);
    fill_region_map(current_region_point, (point.0, point.1 + 1), map, region_map);
}

fn calculate_area_and_perimeter(region_map: &HashMap<(usize, usize), (usize, usize)>) -> HashMap<(usize, usize), (i32, i32)> {
    let mut res: HashMap<(usize, usize), (i32, i32)> = HashMap::new();
    let inverted_region_map = invert_map(region_map);

    for (region, points) in inverted_region_map {
        let mut perimeter = 0;
        let area = points.len() as i32;
        let points_clone = points.clone();
        for point in points_clone {
            if !points.contains(&(point.0 - 1, point.1))
                && (!points.contains(&(point.0, point.1 - 1)) || points.contains(&(point.0 - 1, point.1 - 1))) {
                perimeter += 1;
            }
            if !points.contains(&(point.0 + 1, point.1))
                && (!points.contains(&(point.0, point.1 + 1)) || points.contains(&(point.0 + 1, point.1 + 1))) {
                perimeter += 1;
            }
            if !points.contains(&(point.0, point.1 - 1))
                && (!points.contains(&(point.0 + 1, point.1)) || points.contains(&(point.0 + 1, point.1 - 1))) {
                perimeter += 1;
            }
            if !points.contains(&(point.0, point.1 + 1))
                && (!points.contains(&(point.0 - 1, point.1)) || points.contains(&(point.0 - 1, point.1 + 1))) {
                perimeter += 1;
            }
        }
        res.insert(region, (area, perimeter));
    }

    res
}

fn invert_map(region_map: &HashMap<(usize, usize), (usize, usize)>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut res: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    for (point, region) in region_map {
        res.entry(*region).or_insert(Vec::new()).push(*point);
    }
    res
}

fn get_map(data: &Vec<String>) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = data.iter()
        .map(|s| s.chars()
            .collect::<Vec<char>>())
        .map(|mut row| {
            row.push('.');
            row.insert(0, '.');
            return row;
        })
        .collect();
    let row_length = map[0].len();
    map.push(vec!['.'; row_length]);
    map.insert(0, vec!['.'; row_length]);
    return map;
}

fn test_data() -> Vec<String> {
    r"  AAAA
        BBCD
        BBCC
        EEEC"
        // r"  OOOOO
        //     OXOXO
        //     OOOOO
        //     OXOXO
        //     OOOOO"
        // r"  RRRRIICCFF
        //     RRRRIICCCF
        //     VVRRRCCFFF
        //     VVRCCCJFFF
        //     VVVVCJJCFE
        //     VVIVCCJJEE
        //     VVIIICJJEE
        //     MIIIIIJJEE
        //     MIIISIJEEE
        //     MMMISSJEEE"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}
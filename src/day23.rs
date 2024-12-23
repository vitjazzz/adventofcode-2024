use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/23/input";
    let data = fetch_data(url).await?;
    let data = test_data();

    let start = Instant::now();

    let pairs = get_pairs(&data);
    let links = get_links(&pairs);

    let res = find_max_group(links).join(",");

    let duration = start.elapsed();
    println!("");
    println!("Result: {:?}, Execution time: {:?}", res, duration);

    Ok(())
}

fn find_max_group(links: HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut max_group = vec![];
    for (from_node, from_neighbours) in links.iter() {
        let group = find_max_group_recursive(&links, vec![from_node.clone()], from_neighbours);
        if max_group.len() < group.len() {
            max_group = group;
        }
    }
    max_group.sort();
    max_group
}

fn find_max_group_recursive(links: &HashMap<String, HashSet<String>>, nodes: Vec<String>, current_intersection: &HashSet<String>) -> Vec<String> {
    let mut max_group = vec![];
    for to_node in current_intersection {
        let to_neighbours = links.get(to_node).unwrap();
        let intersection: HashSet<String> = current_intersection.intersection(to_neighbours).cloned().collect();
        let mut nodes = nodes.clone();
        nodes.push(to_node.clone());
        let group = if !intersection.is_empty() {
            find_max_group_recursive(links, nodes, &intersection)
        } else {
            nodes
        };
        if max_group.len() < group.len() {
            max_group = group;
        }
    }
    max_group
}

fn get_links(pairs: &Vec<(String, String)>) -> HashMap<String, HashSet<String>> {
    let mut res: HashMap<String, HashSet<String>> = HashMap::new();
    for (a, b) in pairs {
        res.entry(a.clone())
            .or_insert_with(HashSet::new)
            .insert(b.clone());
        res.entry(b.clone())
            .or_insert_with(HashSet::new)
            .insert(a.clone());
    }
    res
}

fn get_pairs(data: &Vec<String>) -> Vec<(String, String)> {
    data.iter()
        .map(|line| {
            let parts: Vec<&str> = line.trim().split('-').collect();
            (
                parts[0].to_string(),
                parts[1].to_string(),
            )
        })
        .collect()
}

fn test_data() -> Vec<String> {
    r"  kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}
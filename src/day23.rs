use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/23/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();

    let pairs = get_pairs(&data);
    let links = get_links(&pairs);

    let res = find_sets(links);
    let res: usize = res.iter()
        .filter(|&set| set[0].starts_with('t') || set[1].starts_with('t') || set[2].starts_with('t'))
        .count();

    let duration = start.elapsed();
    println!("");
    println!("Result: {:?}, Execution time: {:?}", res, duration);

    Ok(())
}

fn find_sets(links: HashMap<String, Vec<String>>) -> HashSet<Vec<String>> {
    let mut sets: HashSet<Vec<String>> = HashSet::new();
    let mut tasks: VecDeque<(Vec<&String>, &String)> = VecDeque::new();
    for node in links.keys() {
        tasks.push_back((vec![], node));
    }
    while let Some((path, node)) = tasks.pop_front() {
        if path.len() == 3 {
            if path[path.len() - 3] == node {
                let mut set = vec![];
                for i in path.len() - 3..path.len() {
                    set.push(path[i].clone());
                }
                set.sort();
                sets.insert(set);
            }
            continue
        }
        if path.contains(&node) {
            continue;
        }

        let node_links = links.get(node).unwrap();
        for neighbour in node_links {
            let mut new_path = path.clone();
            new_path.push(node);
            tasks.push_back((new_path, neighbour));
        }
    }

    sets
}

fn get_links(pairs: &Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut res: HashMap<String, Vec<String>> = HashMap::new();
    for (a, b) in pairs {
        res.entry(a.clone())
            .and_modify(|v| v.push(b.clone()))
            .or_insert(vec![b.clone()]);
        res.entry(b.clone())
            .and_modify(|v| v.push(a.clone()))
            .or_insert(vec![a.clone()]);
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
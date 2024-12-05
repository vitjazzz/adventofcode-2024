use std::collections::HashMap;
use std::error::Error;
use advent_tools::fetch_data;
use itertools::Itertools;

// 6380 is too high
// 6369 is too high
pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/5/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let rules = get_rules(&data);
    let reverse_rules_map = build_reverse_rules_map(rules.clone());
    let rules_map = build_rules_map(rules.clone());
    let pages = get_pages(&data);

    let res: i32 = pages.into_iter()
        .filter(|page| !is_correct(page, &reverse_rules_map))
        .inspect(|page| println!("uncorrected: {:?}", page))
        .map(|page| correct(page, &reverse_rules_map, &rules_map))
        .inspect(|page| println!("corrected: {:?}", page))
        .map(|page| get_middle_value(&page))
        .sum();

    println!("res: {}", res);

    Ok(())
}

fn build_reverse_rules_map(rules: Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    rules.into_iter()
        .map(|rule| (rule.1, rule.0))
        .into_group_map()
}

fn build_rules_map(rules: Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    rules.into_iter()
        .into_group_map()
}

fn is_correct(page: &Vec<i32>, reverted_rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut not_allowed_pages: Vec<i32> = Vec::new();
    for page_val in page {
        if not_allowed_pages.contains(page_val) {
            return false;
        }
        if let Some(reverted_rule) = reverted_rules.get(page_val) {
            not_allowed_pages.extend(reverted_rule);
        }
    }
    return true;
}

fn correct(mut page: Vec<i32>, reverted_rules: &HashMap<i32, Vec<i32>>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut not_allowed_pages: Vec<i32> = Vec::new();
    let mut page_clone = page.clone();
    for (i, page_val) in page.iter().enumerate() {
        if not_allowed_pages.contains(page_val) {
            let rule = rules.get(page_val).unwrap();
            for j in 0..i {
                let incorrect_page = page.get(j).unwrap();
                if rule.contains(incorrect_page) {
                    page_clone.remove(i);
                    page_clone.insert(i, *incorrect_page);
                    page_clone.remove(j);
                    page_clone.insert(j, *page_val);
                    return correct(page_clone, reverted_rules, rules);
                }
            }
        }
        if let Some(reverted_rule) = reverted_rules.get(page_val) {
            not_allowed_pages.extend(reverted_rule);
        }
    }
    return page;
}

fn get_middle_value(page: &Vec<i32>) -> i32 {
    page[page.len() / 2]
}

fn get_rules(data: &Vec<String>) -> Vec<(i32, i32)> {
    data.iter()
        .filter(|s| s.contains('|'))
        .map(|s| s.split('|').collect::<Vec<&str>>())
        .map(|values| (values.get(0).unwrap().parse::<i32>().unwrap(), values.get(1).unwrap().parse::<i32>().unwrap()))
        .collect()
}

fn get_pages(data: &Vec<String>) -> Vec<Vec<i32>> {
    data.iter()
        .filter(|s| s.contains(','))
        .map(|s| s.split(',').map(|val| val.parse::<i32>().unwrap()).collect())
        .collect()
}

fn test_data() -> Vec<String> {
    r"  47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        97,13,75,29,47"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}
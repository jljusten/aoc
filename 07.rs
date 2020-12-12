/* Advent of Code 2020, https://adventofcode.com/2020/day/7 */

/* Jordan Justen : this file is public domain */

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input("07.test");
    assert_eq!(count_bags(&input, &"shiny gold".to_string()), 4);
    assert_eq!(count_bags_part2(&input, &"shiny gold".to_string()), 32);

    let input = read_input("07.test2");
    assert_eq!(count_bags_part2(&input, &"shiny gold".to_string()), 126);

    let input = read_input("07.input");
    println!("Part 1: {}", count_bags(&input, &"shiny gold".to_string()));
    println!(
        "Part 2: {}",
        count_bags_part2(&input, &"shiny gold".to_string())
    );
}

fn count_bags(input: &Input, bag_name: &String) -> usize {
    let mut reachable = HashSet::<String>::new();
    let mut needed = input
        .bags
        .keys()
        .filter(|k| k.as_str() != bag_name)
        .map(|k| k.clone())
        .collect::<HashSet<String>>();
    loop {
        let mut done = HashSet::new();
        for need in needed.iter() {
            let rules = &input.bags[need].contents;
            if rules.is_empty() {
                done.insert(need.clone());
            } else {
                let mut rules_done = true;
                for (key, _value) in rules {
                    if key == bag_name || (!needed.contains(key) && reachable.contains(key)) {
                        reachable.insert(need.clone());
                        done.insert(need.clone());
                    } else if needed.contains(key) {
                        rules_done = false;
                    }
                }
                if rules_done {
                    done.insert(need.clone());
                }
            }
        }
        assert!(!done.is_empty());
        for b in done.iter() {
            needed.remove(b);
        }
        if needed.len() == 0 {
            break;
        }
    }
    reachable.len()
}

#[derive(Debug)]
enum BagsRequired {
    Macro { contents: HashMap<String, usize> },
    Value { bags: usize },
}

fn count_bags_part2(input: &Input, bag_name: &String) -> usize {
    let mut req = input
        .bags
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                BagsRequired::Macro {
                    contents: v.contents.clone(),
                },
            )
        })
        .collect::<HashMap<String, BagsRequired>>();
    loop {
        let mut updates = HashMap::<String, usize>::new();
        if let BagsRequired::Value { bags } = req[bag_name] {
            return bags;
        }
        for k in req.keys() {
            if let BagsRequired::Macro { contents } = &req[&k.clone()] {
                let mut acc = Some(0usize);
                for k in contents.keys() {
                    match req[k] {
                        BagsRequired::Value { bags } => {
                            acc = Some(acc.unwrap() + (contents[k] * (bags + 1)));
                        }
                        _ => {
                            acc = None;
                            break;
                        }
                    }
                }
                if let Some(count) = acc {
                    updates.insert(k.clone(), count);
                }
            }
        }
        assert!(!updates.is_empty());
        for (k, v) in updates.iter() {
            req.insert(k.clone(), BagsRequired::Value { bags: *v });
        }
    }
}

#[derive(Debug)]
struct BagRules {
    contents: HashMap<String, usize>,
}

#[derive(Debug)]
struct Input {
    bags: HashMap<String, BagRules>,
}

fn read_input(fname: &str) -> Input {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    let bags: HashMap<String, BagRules> = input_file
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = &line[..line.len() - 1];
            let mut bag_name_split = line.split(" bags contain ");
            let bag_name = bag_name_split.next().unwrap().to_string();
            let bag_rules = bag_name_split.next().unwrap();
            let rules: HashMap<String, usize> = if bag_rules == "no other bags" {
                HashMap::new()
            } else {
                bag_rules
                    .split(", ")
                    .map(|rule| {
                        let rule = if rule.ends_with("bags") {
                            &rule[..rule.len() - 5]
                        } else {
                            &rule[..rule.len() - 4]
                        };
                        let mut rule_split = rule.splitn(2, ' ');
                        let count = usize::from_str(rule_split.next().unwrap()).unwrap();
                        let rule_bag = rule_split.next().unwrap();
                        (rule_bag.to_string(), count)
                    })
                    .collect::<HashMap<String, usize>>()
            };
            (bag_name, BagRules { contents: rules })
        })
        .collect();
    Input { bags: bags }
}

/* Advent of Code 2020, Day 4 */

/* Jordan Justen : this file is public domain */

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn main() {
    let input = read_input("04.test");
    assert_eq!(input.into_iter().filter(is_valid).fold(0, |a, _| a + 1), 2);

    let input = read_input("04.input");
    let num_valid = input.into_iter().filter(is_valid).fold(0, |a, _| a + 1);
    println!("Part 1: {}", num_valid);
}

static REQUIRED_FIELDS: [&str; 7] = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

fn is_valid(passport: &Input) -> bool {
    for f in &REQUIRED_FIELDS {
        if !passport.fields.contains_key(*f) {
            return false;
        }
    }
    true
}

#[derive(Debug)]
struct Input {
    fields: HashMap<String, String>,
}

fn read_input(fname: &str) -> Vec<Input> {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    let mut res = Vec::<Input>::new();
    let mut h = HashMap::<String, String>::new();
    for line in input_file.lines() {
        let line = line.unwrap();
        if line.len() > 0 {
            for kv in line.split_whitespace() {
                let mut kvi = kv.split(":");
                let (k, v) = (kvi.next().unwrap(), kvi.next().unwrap());
                h.insert(k.to_string(), v.to_string());
            }
        } else {
            res.push(Input { fields: h });
            h = HashMap::new();
        }
    }
    if h.len() > 0 {
        res.push(Input { fields: h });
    }
    res
}

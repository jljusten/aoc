/* Advent of Code 2020, https://adventofcode.com/2020/day/4 */

/* Jordan Justen : this file is public domain */

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input("04.test");
    assert_eq!(
        input.iter().filter(|x| is_valid(x)).fold(0, |a, _| a + 1),
        2
    );
    let num_valid = input
        .iter()
        .filter(|x| is_valid_part2(x))
        .fold(0, |a, _| a + 1);
    assert_eq!(num_valid, 2);

    let input = read_input("04.input");
    let num_valid = input.iter().filter(|x| is_valid(x)).fold(0, |a, _| a + 1);
    println!("Part 1: {}", num_valid);

    let num_valid = input
        .iter()
        .filter(|x| is_valid_part2(x))
        .fold(0, |a, _| a + 1);
    println!("Part 2: {}", num_valid);
}

static REQUIRED_FIELDS: [&str; 7] = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

static EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn is_valid(passport: &Input) -> bool {
    for f in &REQUIRED_FIELDS {
        if !passport.fields.contains_key(*f) {
            return false;
        }
    }
    true
}

fn is_valid_part2(passport: &Input) -> bool {
    if !is_valid(passport) {
        return false;
    }
    let fields = &passport.fields;
    if let Some(y) = fields.get("byr") {
        if let Ok(y) = usize::from_str(y) {
            if y < 1920 || y > 2002 {
                return false;
            }
        }
    }
    if let Some(y) = fields.get("iyr") {
        if let Ok(y) = usize::from_str(y) {
            if y < 2010 || y > 2020 {
                return false;
            }
        }
    }
    if let Some(y) = fields.get("eyr") {
        if let Ok(y) = usize::from_str(y) {
            if y < 2020 || y > 2030 {
                return false;
            }
        }
    }
    if let Some(h) = fields.get("hgt") {
        if h.ends_with("cm") {
            if let Ok(h) = usize::from_str(&h[..(h.len() - 2)]) {
                if h < 150 || h > 193 {
                    return false;
                }
            }
        } else if h.ends_with("in") {
            if let Ok(h) = usize::from_str(&h[..(h.len() - 2)]) {
                if h < 59 || h > 76 {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    if let Some(h) = fields.get("hcl") {
        if h.len() != 7 {
            return false;
        }
        for c in h[1..].chars() {
            if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
                return false;
            }
        }
    }
    if let Some(pc) = fields.get("ecl") {
        if !EYE_COLORS.iter().any(|c| c == pc) {
            return false;
        }
    }
    if let Some(i) = fields.get("pid") {
        if i.len() != 9 {
            return false;
        }
        for c in i.chars() {
            if !(c >= '0' && c <= '9') {
                return false;
            }
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

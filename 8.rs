/* Advent of Code 2017, Day 8 */

/* Jordan Justen : this file is public domain */

use std::cmp::max;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let input = read_input();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Vec<String>>) {
    let mut regs: HashMap<&str, i32> = HashMap::new();
    for vs in input {
        let v4 = *regs.entry(vs[4].as_ref()).or_insert(0);
        let v = Vec::from_iter(vs.iter().map(|s| s.as_ref()));
        let b = {
            let v6 = i32::from_str(v[6]).unwrap();
            eval_cond(v4, &v[5], v6)
        };
        if b {
            let v0 = regs.entry(vs[0].as_ref()).or_insert(0);
            let v2 = i32::from_str(v[2]).unwrap();
            match v[1] {
                "inc" => ( *v0 += v2 ),
                "dec" => ( *v0 -= v2 ),
                &_ => (),
            }
        }
    }
    let mut it = regs.values();
    let mut max_val = it.nth(0).unwrap();
    for val in it {
        max_val = max(val, max_val);
    }
    println!("part1: {}", max_val);
}

fn eval_cond(v1: i32, cond: &str, v2: i32) -> bool {
    match cond.as_ref() {
        "<" => v1 < v2,
        "<=" => v1 <= v2,
        ">" => v1 > v2,
        ">=" => v1 >= v2,
        "==" => v1 == v2,
        "!=" => v1 != v2,
        &_ => panic!("'{}' unknown", cond),
    }
}

fn part2(input: &Vec<Vec<String>>) {
    let mut max_val = 0i32;
    let mut write_count = 0u32;
    let mut regs: HashMap<&str, i32> = HashMap::new();
    for vs in input {
        let v = Vec::from_iter(vs.iter().map(|s| s.as_ref()));
        let v4 = *regs.entry(v[4]).or_insert(0);
        let v6 = i32::from_str(v[6]).unwrap();
        if eval_cond(v4, &v[5], v6) {
            let v2 = i32::from_str(v[2]).unwrap();
            let entry = regs.entry(v[0]).or_insert(0);
            match v[1] {
                "inc" => ( *entry += v2 ),
                "dec" => ( *entry -= v2 ),
                &_ => (),
            }
            if write_count == 0 {
                max_val = *entry;
            } else {
                max_val = max(*entry, max_val);
            }
            write_count += 1;
        }
    }
    println!("part2: {}", max_val);
}

fn read_input() -> Vec<Vec<String>> {
    let fname = "8.input";
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    let mut s: Vec<Vec<String>> = Vec::new();
    for (_, rln) in input_file.lines().enumerate() {
        let ln = rln.unwrap();
        let vln = Vec::from_iter(ln.split_whitespace().map(|s| String::from(s)));
        s.push(vln);
    }
    s
}

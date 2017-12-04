/* Advent of Code 2017, Day 4 */

/* Jordan Justen : this file is public domain */

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::iter::FromIterator;
use std::collections::HashSet;

fn main() {
    let s = read_input();
    part1(&s);
    part2(&s);
}

fn part1(s: &Vec<Vec<String>>) {
    let mut num_valid = 0;

    for pp in s {
        if passphase_is_valid(pp) {
            num_valid += 1;
        }
    }

    println!("part1: {}", num_valid);
}

fn passphase_is_valid(pp: &Vec<String>) -> bool {
    let mut cws: HashSet<&String> = HashSet::new();
    for cw in pp {
        if cws.contains(cw) {
            return false;
        } else {
            cws.insert(cw);
        }
    }
    return true;
}

fn part2(s: &Vec<Vec<String>>) {
    let mut num_valid = 0;

    for pp in s {
        let spp = sort_codewords(pp);
        if passphase_is_valid(&spp) {
            num_valid += 1;
        }
    }

    println!("part2: {}", num_valid);
}

fn sort_codewords(pp: &Vec<String>) -> Vec<String> {
    let mut res = Vec::<String>::new();
    for cw in pp {
        let mut scw: Vec<char> = cw.chars().collect();
        scw.sort();
        res.push(String::from_iter(scw));
    }
    return res;
}

fn read_input() -> Vec<Vec<String>> {
    let fname = "4.input";
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

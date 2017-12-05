/* Advent of Code 2017, Day 5 */

/* Jordan Justen : this file is public domain */

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

fn part1(input: &Vec<i32>) {
    let mut jt = input.clone();
    let mut ip = 0usize;
    let mut steps = 0;
    loop {
        if ip >= input.len() {
            break;
        }
        steps += 1;
        let next = ip as i32 + jt[ip];
        assert!(next >= 0);
        jt[ip] += 1;
        ip = next as usize;
    }
    println!("part1: {}", steps);
}

fn part2(input: &Vec<i32>) {
    let mut jt = input.clone();
    let mut ip = 0usize;
    let mut steps = 0;
    loop {
        if ip >= input.len() {
            break;
        }
        steps += 1;
        let next = ip as i32 + jt[ip];
        assert!(next >= 0);
        if jt[ip] < 3 {
            jt[ip] += 1;
        } else {
            jt[ip] -= 1;
        }
        ip = next as usize;
    }
    println!("part2: {}", steps);
}

fn read_input() -> Vec<i32> {
    let fname = "5.input";
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    Vec::from_iter(input_file.lines().
                   enumerate().
                   map(|(_, s)| i32::from_str(s.unwrap().as_str()).unwrap()))
}

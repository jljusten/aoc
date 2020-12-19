/* Advent of Code 2020, https://adventofcode.com/2020/day/13 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input("13.test");
    assert_eq!(part1(&input), 295);

    let input = read_input("13.input");
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &Input) -> usize {
    input
        .nums
        .iter()
        .scan((std::usize::MAX, 0usize), |s, &x| {
            let x = x.0;
            let wait = x - (input.depart % x);
            if wait < s.0 {
                s.0 = wait;
                s.1 = wait * x;
            }
            Some(s.1)
        })
        .last()
        .unwrap()
}

#[derive(Debug)]
struct Input {
    depart: usize,
    nums: Vec<(usize, usize)>,
}

fn read_input(fname: &str) -> Input {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    let mut lines = input_file.lines();
    let depart = lines.next().unwrap().unwrap();
    let depart = usize::from_str(&depart).unwrap();
    let nums: Vec<(usize, usize)> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|n| n.1 != "x")
        .map(|n| (usize::from_str(n.1).unwrap(), n.0))
        .collect();
    Input { depart, nums }
}

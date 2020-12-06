/* Advent of Code 2020, Day 3 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn main() {
    let input = read_input("03.test");
    assert_eq!(part1(&input), 7);
    let input = read_input("03.input");
    println!("Part 1:");
    println!("count: {}", part1(&input));
    assert!(part1(&input) < 229);
}

fn part1(input: &Input) -> usize {
    let (mut x, mut y) = (3, 1);
    let mut count = 0;
    while y < input.map.len() {
        if input.lookup(x, y) {
            count += 1;
        }
        x += 3;
        y += 1;
    }
    count
}

impl Input {
    fn lookup(&self, x: usize, y: usize) -> bool {
        let row = &self.map[y];
        row[x % row.len()]
    }
}

#[derive(Debug)]
struct Input {
    map: Vec<Vec<bool>>,
}

fn line_to_input(line: &String) -> Vec<bool> {
    line.chars().map(|c| {
        match c {
            '#' => true,
            '.' => false,
            _ => panic!("Unsupported char ({}) in input", c),
        }
    }).collect()
}

fn read_input(fname: &str) -> Input {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    Input {
        map: input_file
            .lines()
            .enumerate()
            .map(|line| line_to_input(&line.1.unwrap()))
            .collect()
    }
}

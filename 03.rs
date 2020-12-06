/* Advent of Code 2020, Day 3 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static PART_2_PATHS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

static PART_2_TEST_RESULTS: [usize; 5] = [2, 7, 3, 4, 2];

pub fn main() {
    let input = read_input("03.test");
    for i in 0..PART_2_PATHS.len() {
        let (x, y) = PART_2_PATHS[i];
        assert_eq!(count_path(&input, x, y), PART_2_TEST_RESULTS[i]);
    }
    let input = read_input("03.input");
    println!("Part 1:");
    println!("count: {}", part1(&input));
    assert!(part1(&input) < 229);
}

fn part1(input: &Input) -> usize {
    count_path(input, 3, 1)
}

fn count_path(input: &Input, dx: usize, dy: usize) -> usize {
    let (mut x, mut y) = (dx, dy);
    let mut count = 0;
    while y < input.map.len() {
        if input.lookup(x, y) {
            count += 1;
        }
        x += dx;
        y += dy;
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

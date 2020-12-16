/* Advent of Code 2020, https://adventofcode.com/2020/day/11 */

/* Jordan Justen : this file is public domain */

use std::cmp::{max, min};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn main() {
    let input = read_input("11.test");
    assert_eq!(part1(&input), 37);

    let input = read_input("11.input");
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &Input) -> usize {
    let mut input = Box::<Input>::new((*input).clone());
    let last = loop {
        let (next, changed) = next_state(&input);
        if !changed {
            break next;
        }
        input = next;
    };
    count_occupied(&last)
}

fn next_state(input: &Input) -> (Box<Input>, bool) {
    let counts = count_surrounding_occupied(input);

    let mut changed = false;
    let d = &input.data;
    let (h, w) = (d.len(), d[0].len());
    let mut next = Box::new(input.clone());
    for y in 0..h {
        for x in 0..w {
            match d[y][x] {
                SquareState::Floor => (),
                SquareState::Empty => {
                    if counts[y][x] == 0 {
                        next.data[y][x] = SquareState::Occupied;
                        changed = true;
                    }
                }
                SquareState::Occupied => {
                    if counts[y][x] >= 4 {
                        next.data[y][x] = SquareState::Empty;
                        changed = true;
                    }
                }
            }
        }
    }
    (next, changed)
}

fn count_surrounding_occupied(input: &Input) -> Vec<Vec<u8>> {
    let d = &input.data;
    let (h, w) = (d.len(), d[0].len());
    let mut counts = vec![vec!(0u8; w); h];
    for y in 0..h {
        for x in 0..w {
            let mut occupied_count = 0;
            for ay in (max(y, 1) - 1)..(min(y, h - 2) + 2) {
                for ax in (max(x, 1) - 1)..(min(x, w - 2) + 2) {
                    if (ax != x || ay != y) && d[ay][ax] == SquareState::Occupied {
                        occupied_count += 1;
                    }
                }
            }
            counts[y][x] = occupied_count;
        }
    }
    counts
}

fn count_occupied(input: &Input) -> usize {
    let mut occupied_count = 0;
    let d = &input.data;
    let (h, w) = (d.len(), d[0].len());
    for y in 0..h {
        for x in 0..w {
            if d[y][x] == SquareState::Occupied {
                occupied_count += 1;
            }
        }
    }
    occupied_count
}

#[derive(Clone, Debug, PartialEq)]
enum SquareState {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, Debug)]
struct Input {
    data: Vec<Vec<SquareState>>,
}

fn read_input(fname: &str) -> Input {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    Input {
        data: input_file
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| match c {
                        '.' => SquareState::Floor,
                        'L' => SquareState::Empty,
                        '#' => SquareState::Occupied,
                        _ => panic!("Unsupported char: {}", c),
                    })
                    .collect()
            })
            .collect(),
    }
}

/* Advent of Code 2020, https://adventofcode.com/2020/day/11 */

/* Jordan Justen : this file is public domain */

use std::cmp::{max, min};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn main() {
    let input = read_input("11.test");
    assert_eq!(part1(&input), 37);
    assert_eq!(part2(&input), 26);

    let input = read_input("11.input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut state = (*input).clone();
    loop {
        let changed = next_state(&mut state);
        if !changed {
            break;
        }
    }
    count_occupied(&state)
}

fn next_state(input: &mut Input) -> bool {
    let counts = count_surrounding_occupied(input);
    apply_next_state(input, &counts, 4)
}

fn apply_next_state(input: &mut Input, counts: &Vec<Vec<u8>>, max_occupied: u8) -> bool {
    let mut changed = false;
    let (h, w) = (input.data.len(), input.data[0].len());
    let next = input;
    for y in 0..h {
        for x in 0..w {
            match next.data[y][x] {
                SquareState::Floor => (),
                SquareState::Empty => {
                    if counts[y][x] == 0 {
                        next.data[y][x] = SquareState::Occupied;
                        changed = true;
                    }
                }
                SquareState::Occupied => {
                    if counts[y][x] >= max_occupied {
                        next.data[y][x] = SquareState::Empty;
                        changed = true;
                    }
                }
            }
        }
    }
    changed
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

fn part2(input: &Input) -> usize {
    let mut state = (*input).clone();
    loop {
        let changed = next_state_part2(&mut state);
        if !changed {
            break;
        }
    }
    count_occupied(&state)
}

fn next_state_part2(input: &mut Input) -> bool {
    let counts = count_surrounding_occupied_part2(input);
    apply_next_state(input, &counts, 5)
}

static DIRS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn count_surrounding_occupied_part2(input: &Input) -> Vec<Vec<u8>> {
    let d = &input.data;
    let (h, w) = (d.len(), d[0].len());
    let mut counts = vec![vec!(0u8; w); h];
    for y in 0..h {
        for x in 0..w {
            let mut occupied_count = 0;
            for dir in DIRS.iter() {
                let (mut x, mut y) = (x as isize, y as isize);
                loop {
                    x += dir.0;
                    y += dir.1;
                    if x < 0 || x >= w as isize || y < 0 || y >= h as isize {
                        break;
                    }
                    match d[y as usize][x as usize] {
                        SquareState::Occupied => {
                            occupied_count += 1;
                            break;
                        },
                        SquareState::Empty => break,
                        _ => (),
                    }
                }
            }
            counts[y][x] = occupied_count;
        }
    }
    counts
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

/* Advent of Code 2020, https://adventofcode.com/2020/day/10 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input("10.test");
    assert_eq!(part1(&input), 35);

    let input = read_input("10.test2");
    assert_eq!(part1(&input), 220);

    let input = read_input("10.input");
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &Input) -> u64 {
    let last = input
        .data
        .iter()
        .chain([input.data[input.data.len() - 1] + 3].iter())
        .scan((0, 0, 0, 0), |state, &x| {
            let d = x - state.0;
            match d {
                1 => (*state).1 += 1,
                2 => (*state).2 += 1,
                3 => (*state).3 += 1,
                _ => panic!("Unsupported delta in jolts of {}", d),
            }
            (*state).0 = x;
            Some(*state)
        })
        .last()
        .unwrap();
    last.1 * last.3
}

#[derive(Debug)]
struct Input {
    data: Vec<u8>,
}

fn read_input(fname: &str) -> Input {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    let mut jolts: Vec<u8> = input_file
        .lines()
        .map(|line| u8::from_str(line.unwrap().as_str()).unwrap())
        .collect();
    jolts.sort();
    Input { data: jolts }
}

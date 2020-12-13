/* Advent of Code 2020, https://adventofcode.com/2020/day/9 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input("09.test");
    assert_eq!(part1(&input, 5), 127);

    let input = read_input("09.input");
    println!("Part 1: {}", part1(&input, 25));
}

fn part1(input: &Input, size: usize) -> u64 {
    let mut pos = size;
    loop {
        let target = input.data[pos];
        let mut found = false;
        'outer: for i in (pos - size)..(pos - 1) {
            for j in (i + 1)..pos {
                let (fst, snd) = (input.data[i], input.data[j]);
                if fst != snd && fst + snd == target {
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            return target;
        }
        pos += 1;
        assert!(pos < input.data.len());
    }
}

#[derive(Debug)]
struct Input {
    data: Vec<u64>,
}

fn read_input(fname: &str) -> Input {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    Input {
        data: input_file
            .lines()
            .map(|line| u64::from_str(line.unwrap().as_str()).unwrap())
            .collect(),
    }
}

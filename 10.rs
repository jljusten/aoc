/* Advent of Code 2020, https://adventofcode.com/2020/day/10 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input("10.test");
    assert_eq!(part1(&input), 35);
    assert_eq!(part2(&input), 8);

    let input = read_input("10.test2");
    assert_eq!(part1(&input), 220);
    assert_eq!(part2(&input), 19208);

    let input = read_input("10.input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

fn part2(input: &Input) -> u64 {
    let dev_jolts = input.data.last().unwrap() + 3;
    let mut jolts = input.data.clone();
    jolts.insert(0, 0);
    jolts.push(dev_jolts);
    let jolts = jolts;
    let mut deps = vec![[0; 3]; jolts.len()];
    for i in 0..jolts.len() {
        for j in (i + 1)..jolts.len() {
            if jolts[j] > jolts[i] + 3 {
                break;
            } else {
                deps[i][j - i - 1] = j;
            }
        }
    }

    let mut counts = vec![0; jolts.len()];
    counts[jolts.len() - 1] = 1;
    while deps[0] != [0, 0, 0] {
        let mut progress = false;
        for i in 0..jolts.len() {
            if deps[i] == [0, 0, 0] {
                continue;
            }
            for j in 0..3 {
                let dep = deps[i][j];
                if dep > 0 && deps[dep] == [0, 0, 0] {
                    counts[i] += counts[dep];
                    deps[i][j] = 0;
                    progress = true;
                }
            }
        }
        assert!(progress);
    }

    counts[0]
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

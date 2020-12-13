/* Advent of Code 2020, https://adventofcode.com/2020/day/9 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input("09.test");
    assert_eq!(part1(&input, 5), 127);
    assert_eq!(part2(&input, 5), 62);

    let input = read_input("09.input");
    println!("Part 1: {}", part1(&input, 25));
    println!("Part 2: {}", part2(&input, 25));
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

fn part2(input: &Input, size: usize) -> u64 {
    let target = part1(input, size);
    let (mut start, mut end) = (0, 0);

    'outer: for i in 0..(input.data.len() - 1) {
        let mut sum = input.data[i];
        for j in (i + 1)..(input.data.len() - i) {
            sum += input.data[j];
            if sum == target {
                start = i;
                end = j + 1;
                break 'outer;
            } else if sum > target {
                break;
            }
        }
    }

    assert!(end <= input.data.len() && end - start >= 2);
    let mut nums: Vec<u64> = input.data[start..end].to_vec();
    nums.sort();
    nums[0] + nums[nums.len() - 1]
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

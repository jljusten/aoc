/* Advent of Code 2020, https://adventofcode.com/2020/day/13 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::str::FromStr;

pub fn main() {
    let input = read_input("13.test");
    assert_eq!(part1(&input), 295);
    assert_eq!(part2(&input), 1068781);

    static MORE_PART2_EXAMPLES: [(&str, u64); 5] = [
        ("17,x,13,19", 3417),
        ("67,7,59,61", 754018),
        ("67,x,7,59,61", 779210),
        ("67,7,x,59,61", 1261476),
        ("1789,37,47,1889", 1202161486),
    ];
    for e in MORE_PART2_EXAMPLES.iter() {
        let input = input_from_str(["0\n", e.0, "\n"].concat().as_str());
        assert_eq!(part2(&input), e.1);
    }

    let input = read_input("13.input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

fn part2(input: &Input) -> u64 {
    input
        .nums
        .iter()
        .scan(vec![(1, 0)], |terms, &x| {
            let mod_target = (x.0 - (x.1 % x.0)) % x.0;
            let mut res = 0u64;
            for i in 0.. {
                let eval = terms.iter().rev().fold(i, |a, x| a * x.0 + x.1);
                if (eval % x.0) == mod_target {
                    res = eval as u64;
                    terms.push((x.0, i));
                    break;
                }
            }
            Some(res)
        })
        .last()
        .unwrap()
}

#[derive(Debug)]
struct Input {
    depart: usize,
    nums: Vec<(usize, usize)>,
}

fn read_br<R: Read>(br: &mut BufReader<R>) -> Input {
    let mut lines = br.lines();
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

fn input_from_str(s: &str) -> Input {
    let mut input_file = BufReader::new(s.as_bytes());
    read_br(&mut input_file)
}

fn read_input(fname: &str) -> Input {
    let input_f = File::open(fname).unwrap();
    let mut input_file = BufReader::new(&input_f);
    read_br(&mut input_file)
}

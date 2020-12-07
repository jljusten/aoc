/* Advent of Code 2020, Day 6 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn main() {
    let input = read_input("06.test");
    let counts = input.iter().map(|i| count_group(i)).fold(0, |a, c| a + c);
    assert_eq!(counts, 11);
    let counts = input.iter().map(|i| count_group_part2(i)).fold(0, |a, c| a + c);
    assert_eq!(counts, 6);

    let input = read_input("06.input");
    let counts = input.iter().map(|i| count_group(i)).fold(0, |a, c| a + c);
    println!("Part 1: {}", counts);

    let counts = input.iter().map(|i| count_group_part2(i)).fold(0, |a, c| a + c);
    println!("Part 2: {}", counts);
}

fn count_group(group: &Input) -> u32 {
    let mut answered = 0u32;
    for answers in group.answers.iter() {
        for c in answers.chars() {
            match c {
                'a'..='z' => {
                    answered |= 1 << (c as u32 - 'a' as u32);
                },
                _ => panic!("Unknown answer, {}", c),
            }
        }
    }
    answered.count_ones()
}

fn count_group_part2(group: &Input) -> u32 {
    let mut answered = 0u32;
    let mut not_answered = 0u32;
    for answers in group.answers.iter() {
        let mut person_answered = 0u32;
        for c in answers.chars() {
            match c {
                'a'..='z' => {
                    person_answered |= 1 << (c as u32 - 'a' as u32);
                },
                _ => panic!("Unknown answer, {}", c),
            }
        }
        not_answered |= 0x3ffffff & !person_answered;
        answered |= person_answered;
    }
    /* All answered */
    (answered & !not_answered).count_ones()
}

#[derive(Debug)]
struct Input {
    answers: Vec<String>,
}

fn read_input(fname: &str) -> Vec<Input> {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    let mut res = Vec::<Input>::new();
    let mut g = Vec::<String>::new();
    for line in input_file.lines() {
        let line = line.unwrap();
        if line.len() > 0 {
            g.push(line);
        } else {
            res.push(Input { answers: g });
            g = Vec::new();
        }
    }
    if g.len() > 0 {
        res.push(Input { answers: g });
    }
    res
}

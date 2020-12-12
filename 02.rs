/* Advent of Code 2020, https://adventofcode.com/2020/day/2 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input();
    let num_valid = input.iter().filter(|i| is_valid(i)).count();
    println!("Part 1:");
    println!("num_valid: {}", num_valid);
    let num_valid = input.iter().filter(|i| is_valid_part2(i)).count();
    println!("Part 2:");
    println!("num_valid: {}", num_valid);
}

fn is_valid(pw: &Input) -> bool {
    let count = pw.pw.chars().filter(|p| *p == pw.ch).count();
    count >= pw.min && count <= pw.max
}

fn is_valid_part2(pw: &Input) -> bool {
    let locs: [usize; 2] = [pw.min, pw.max];
    let chars: Vec<char> = pw.pw.chars().collect();
    let count = locs.iter().filter(|p| chars[**p-1] == pw.ch).count();
    count == 1
}

#[derive(Debug)]
struct Input {
    min: usize,
    max: usize,
    ch: char,
    pw: String,
}

fn line_to_input(line: &String) -> Input {
    let mut split = line.split_whitespace();
    let range = split.next().unwrap();
    let char_str = split.next().unwrap();
    let char_str = &char_str[..char_str.len()-1];
    let pw = split.next().unwrap();
    let mut range_split = range.split('-');
    Input {
        min: usize::from_str(range_split.next().unwrap()).unwrap(),
        max: usize::from_str(range_split.next().unwrap()).unwrap(),
        ch: char::from_str(char_str).unwrap(),
        pw: pw.clone().to_string(),
    }
}

fn read_input() -> Vec<Input> {
    let fname = "02.input";
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    input_file
        .lines()
        .enumerate()
        .map(|line| line_to_input(&line.1.unwrap()))
        .collect()
}

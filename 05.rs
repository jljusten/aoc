/* Advent of Code 2020, https://adventofcode.com/2020/day/5 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn main() {
    let input = read_input("05.input");
    println!("Part 1: {}", input.iter().fold(0, |a, x| usize::max(a, seat_id(&x.seat))));

    println!("Part 2: {}", find_seat(&input));
}

fn seat_id(seat: &String) -> usize {
    let as_bin: String = seat.chars().map(|c| {
        match c {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => panic!("Bad char {}", c),
        }
    }).collect();
    usize::from_str_radix(as_bin.as_str(), 2).unwrap()
}

fn find_seat(input: &Vec<Input>) -> usize {
    let mut seats: Vec<usize> = input.iter().map(|i| seat_id(&i.seat)).collect();
    seats.sort();
    for i in 1..(seats.len()-1) {
        if seats[i] + 2 == seats[i+1] {
            return seats[i] + 1
        }
    }
    panic!("Didn't find seat!")
}

#[derive(Debug)]
struct Input {
    seat: String,
}

fn read_input(fname: &str) -> Vec<Input> {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);

    input_file
        .lines()
        .enumerate()
        .map(|line| Input {
            seat: line.1.unwrap(),
        })
        .collect()
}

/* Advent of Code 2017, Day 9 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = read_input();
    part1(&input);
    part2(&input);
}

enum State {
    Looking,
    Group,
    Garbage,
    Escape,
}

fn part1(input: &String) {
    let mut state = State::Looking;
    let mut level = 0u32;
    let mut score = 0u32;
    let mut garbage = 0u32;
    for c in input.chars() {
        match state {
            State::Looking => {
                match c {
                    '{' => { level += 1; state = State::Group },
                    '}' => { score += level; level -= 1 },
                    '<' => { state = State::Garbage; },
                    ',' => (),
                    '\n' => assert!(level == 0),
                    _ => panic!("char {}", c),
                }
            },
            State::Group => {
                match c {
                    '{' => { level += 1; state = State::Group },
                    '}' => {
                        score += level;
                        level -= 1;
                        state =
                            if level > 0 { State::Group }
                            else { State::Looking }
                    },
                    '<' => { state = State::Garbage; },
                    _ => (),
                }
            }
            State::Garbage => {
                match c {
                    '!' => state = State::Escape,
                    '>' => state = State::Looking,
                    _ => ( garbage += 1),
                }
            },
            State::Escape => state = State::Garbage,
        }
    }

    println!("part1: {}", score);
    println!("part2: {}", garbage);
}

fn part2(input: &String) {
    println!("part2: {}", '?');
}

fn read_input() -> String {
    let fname = "9.input";
    let mut input_file = File::open(fname).unwrap();
    let mut s = String::new();
    input_file.read_to_string(&mut s);
    s
    //String::from("{{<a!>},{<a!>},{<a!>},{<ab>}}")
    //String::from("{{<ab>},{<ab>},{<ab>},{<ab>}}")
    //String::from("{{{},{},{{}}}}\n")
}

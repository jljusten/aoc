/* Advent of Code 2017, Day 19 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum State {
    Down { x: usize, y: usize },
    Up { x: usize, y: usize },
    Left { x: usize, y: usize },
    Right { x: usize, y: usize },
}

fn main() {
    let input = read_input1();
    let (string, steps) = trace_path(&input);
    println!("part1: {}", string);
    println!("part2: {}", steps);
}

fn trace_path(input: &Vec<Vec<char>>) -> (String, usize) {
    let mut x = 0;
    for (i, c) in input[0].iter().enumerate() {
        if *c == '|' {
            x = i;
            break;
        }
    }
    assert!(input[0][x] == '|');
    let mut steps = 1;
    let mut state = State::Down { x: x, y: 0 };
    let mut result = String::new();
    for _ in 0 .. {
        state = step(input, &state);
        let (x, y) = get_pos(&state);
        if input[y][x].is_alphabetic() {
            result.push(input[y][x]);
        } else if input[y][x].is_whitespace() {
            break;
        }
        steps += 1;
    }
    (result, steps)
}

fn step(input: &Vec<Vec<char>>, state: &State) -> State {
    let moved = match state {
        &State::Down { x, y } => State::Down { x: x, y: y + 1 },
        &State::Up { x, y } => State::Up { x: x, y: y - 1 },
        &State::Left { x, y } => State::Left { x: x - 1, y: y },
        &State::Right { x, y } => State::Right { x: x + 1, y: y },
    };
    let (x, y) = get_pos(&moved);
    if input[y][x] == '+' {
        let turned = match moved {
            State::Down { x, y } => {
                if input[y][x - 1] != ' ' { State::Left { x: x, y: y } }
                else { State::Right { x: x, y: y } }
            },
            State::Up { x, y } => {
                if input[y][x - 1] != ' ' { State::Left { x: x, y: y } }
                else { State::Right { x: x, y: y } }
            },
            State::Left { x, y } => {
                if input[y - 1][x] != ' ' { State::Up { x: x, y: y } }
                else { State::Down { x: x, y: y } }
            },
            State::Right { x, y } => {
                if input[y - 1][x] != ' ' { State::Up { x: x, y: y } }
                else { State::Down { x: x, y: y } }
            },
        };
        return turned;
    }
    moved
}

fn get_pos(state: &State) -> (usize, usize) {
    match state {
        &State::Down { x, y } => ( x, y ),
        &State::Up { x, y } => ( x, y ),
        &State::Left { x, y } => ( x, y ),
        &State::Right { x, y } => ( x, y ),
    }
}

#[allow(dead_code)]
fn read_input0() -> Vec<Vec<char>> {
    let sample_lines: Vec<String> ="
             |          
             |  +--+    
             A  |  C    
         F---|----E|--+ 
             |  |  |  D 
             +B-+  +--+ 
                        
        ".split('\n').map(|s| String::from(s)).
        skip(1).collect();
    lines_to_vecs(&sample_lines)
}

fn lines_to_vecs(ls: &Vec<String>) -> Vec<Vec<char>> {
    ls.iter().map(|s| s.chars().collect::<Vec<_>>()).collect()
}

#[allow(dead_code)]
fn read_input1() -> Vec<Vec<char>> {
    let fname = "19.input";
    let mut input_file = File::open(fname).unwrap();
    let mut s = String::new();
    let r = input_file.read_to_string(&mut s);
    assert!(r.is_ok());
    let input =s.split('\n').map(|s| String::from(s)).collect();
    lines_to_vecs(&input)
}

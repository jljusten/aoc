/* Advent of Code 2017, Day 16 */

/* Jordan Justen : this file is public domain */

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
//use std::iter::FromIterator;
use std::str::FromStr;

struct State {
    v: Vec<char>,
    m: HashMap<char, usize>,
    r: usize,
}

enum Move {
    Spin { o: usize },
    Exchange { o1: usize, o2: usize },
    Partner { p1: char, p2: char },
}

fn main() {
    let (programs, input) = read_input1();
    let mut state = State {
        v: programs.chars().collect(),
        m: HashMap::new(),
        r: 0,
    };
    for (i, c) in state.v.iter().enumerate() {
        state.m.insert(*c, i);
    }
    //let mut state: Vec<char> = programs.chars().collect();
    let mut next_state = dance_routine(&mut state, &input);
    let part1: String = next_state.v.iter().collect();
    println!("part1: {}", part1);

    for i in 0 .. 10000-1 {
        let cur_state = next_state;
        next_state = dance_routine(cur_state, &input);
        if i % 1000000 == 0 {
            println!("{}", i);
        }
    }
    let part2: String = next_state.v.iter().collect();
    println!("part2: {}", part2);
}

fn dance_routine<'a>(state: &'a mut State, input: &Vec<Move>) -> &'a mut State {
    let len = state.v.len();
    for mv in input {
        match *mv {
            Move::Spin {o} => {
                let mut rest = state.v.split_off(len - o);
                while rest.len() > 0 {
                    state.v.insert(0, rest.pop().unwrap());
                }
            }
            Move::Exchange {o1, o2} => {
                state.v.swap(o1, o2);
            },
            Move::Partner {p1, p2} => {
                let mut o1: i32 = -1;
                let mut o2: i32 = -1;
                for (i, c) in state.v.iter().enumerate() {
                    if *c == p1 {
                        o1 = i as i32;
                    } else if *c == p2 {
                        o2 = i as i32;
                    }
                    if o1 >= 0 && o2 >= 0 {
                        break;
                    }
                }
                assert!(o1 >= 0 && o2 >= 0);
                state.v.swap(o1 as usize, o2 as usize);
            },
        }
    }
    state
}

#[allow(dead_code)]
fn read_input0() -> (String, Vec<Move>) {
    let sample = "s1,x3/4,pe/b";
    (String::from("abcde"), str_to_moves(String::from(sample)))
}

fn str_to_moves(s: String) -> Vec<Move> {
    s.trim_right().split(',').map(|s| {
        let mut cmd = String::from(s).clone();
        let rest = cmd.split_off(1);
        //println!("{} {}", cmd, rest);
        let params: Vec<&str> = rest.split("/").collect();
        match cmd.as_str() {
            "s" => {
                Move::Spin { o: usize::from_str(params[0]).unwrap() }
            },
            "x" => {
                let nums: Vec<usize> =
                    params.into_iter().map(|s| usize::from_str(s).unwrap()).
                    collect();
                Move::Exchange { o1: nums[0], o2: nums[1] }
            },
            "p" => {
                Move::Partner { p1: params[0].chars().next().unwrap(),
                                p2: params[1].chars().next().unwrap() }
            },
            _ => panic!("unknown move {}", s)
        }
    }).collect()
}

#[allow(dead_code)]
fn read_input1() -> (String, Vec<Move>) {
    let fname = "16.input";
    let mut input_file = File::open(fname).unwrap();
    let mut s = String::new();
    let r = input_file.read_to_string(&mut s);
    assert!(r.is_ok());
    (String::from("abcdefghijklmnop"), str_to_moves(s))
}

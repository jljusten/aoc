/* Advent of Code 2017, Day 16 */

/* Jordan Justen : this file is public domain */

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

struct State {
    p: [u8; 16],
    v: [u8; 16],
    r: usize,
    len: usize,
}

#[derive(Clone)]
enum Move {
    Spin { o: usize },
    Exchange { o1: usize, o2: usize },
    Partner { p1: char, p2: char },
}

fn main() {
    let (programs, input_dance) = read_input1();
    let input = optimize_dance(&input_dance, programs.len());
    let mut state = State {
        v: [0u8; 16],
        p: [0u8; 16],
        r: 0,
        len: programs.len(),
    };
    for i in 0..16 {
        state.p[i] = i as u8;
        state.v[i] = i as u8;
    }
    let program_chars: Vec<char> = programs.chars().collect();
    let mut next_state = dance_routine(&mut state, &input);
    let part1 =
        (0..programs.len()).map(|i| {
            let pos = (i + next_state.r) % programs.len();
            program_chars[next_state.p[pos] as usize]
        }).collect::<String>();
    println!("part1: {}", part1);

    let mut repeat = false;
    let mut repeat_start = 0;
    let mut repeat_end = 0;
    let routine_count = 1000000000;

    {
        let mut seen: HashMap<u64, usize> = HashMap::new();
        seen.insert(state_to_u64(next_state), 0);
        for i in 1 .. routine_count {
            let cur_state = next_state;
            next_state = dance_routine(cur_state, &input);
            let u = state_to_u64(next_state);
            if seen.contains_key(&u) {
                repeat_start = *seen.get(&u).unwrap();
                repeat_end = i;
                repeat = true;
                break;
            } else {
                seen.insert(u, i);
            }
        }
    }

    if repeat {
        let remaining = routine_count - repeat_end - 1;
        let ending_routines = remaining % (repeat_end - repeat_start);
        for _ in 0 .. ending_routines {
            let cur_state = next_state;
            next_state = dance_routine(cur_state, &input);
        }
    }
    let part2 =
        (0..programs.len()).map(|i| {
            let pos = (i + next_state.r) % programs.len();
            program_chars[next_state.p[pos] as usize]
        }).collect::<String>();
    println!("part2: {}", part2);
}

fn state_to_u64(state: &State) -> u64 {
    assert!(state.len <= 16);
    let mut r = 0u64;
    for i in 0 .. state.len {
        let idx = (i + state.r) % state.len;
        r = (r << 4) + state.v[idx] as u64;
    }
    r
}

fn optimize_dance(input: &Vec<Move>, len: usize) -> Vec<Move> {
    let mut result = Vec::<Move>::new();
    let mut source: Vec<Move> = input.clone();
    let mut i = 0;
    let mut progress = false;

    loop {
        while i < source.len() {
            if let Move::Spin { o } = source[i] {
                if o == 0 || o == len {
                    i += 1;
                    progress = true;
                    continue;
                }
            }
            if i + 1 < source.len() {
                match (&source[i], &source[i + 1]) {
                    (&Move::Spin { o: o1 }, &Move::Spin { o: o2 }) => {
                        result.push(Move::Spin { o: (o1 + o2) % len });
                        i += 2;
                        progress = true;
                    },
                    (&Move::Spin { o }, &Move::Exchange { o1, o2 }) => {
                        let no1 = (o1 + len - o) % len;
                        let no2 = (o2 + len - o) % len;
                        result.push(Move::Exchange { o1: no1, o2: no2 });
                        result.push(Move::Spin { o: o });
                        i += 2;
                        progress = true;
                    },
                    (&Move::Exchange { o1, o2 } , &Move::Exchange { o1: s1, o2: s2 }) => {
                        if o1.min(o2) == s1.min(s2) && o1.max(o2) == s1.max(s2) {
                            progress = true;
                            i += 2;
                        } else {
                            result.push(source[i].clone());
                            i += 1;
                        }
                    },
                    (&Move::Partner { p1, p2 } , &Move::Partner { p1: s1, p2: s2 }) => {
                        if p1.min(p2) == s1.min(s2) && p1.max(p2) == s1.max(s2) {
                            progress = true;
                            i += 2;
                        } else {
                            result.push(source[i].clone());
                            i += 1;
                        }
                    },
                    _ => {
                        result.push(source[i].clone());
                        i += 1;
                    },
                }
            } else {
                result.push(source[i].clone());
                i += 1;
            }
        }
        if !progress {
            break;
        } else {
            source = result.clone();
            result = Vec::<Move>::new();
            i = 0;
            progress = false;
        }
    }
    result
}

fn dance_routine<'a>(state: &'a mut State, input: &Vec<Move>) -> &'a mut State {
    let len = state.len;
    for mv in input {
        match *mv {
            Move::Spin {o} => {
                state.r = (state.r + (len - o)) % len;
            }
            Move::Exchange {o1, o2} => {
                let ro1 = (state.r + o1) % len;
                let ro2 = (state.r + o2) % len;
                let byval_swap = state.v[state.p[ro1] as usize];
                state.v[state.p[ro1] as usize] = state.v[state.p[ro2] as usize];
                state.v[state.p[ro2] as usize] = byval_swap;
                let swap = state.p[ro1];
                state.p[ro1] = state.p[ro2];
                state.p[ro2] = swap;
            },
            Move::Partner {p1, p2} => {
                let o1 = state.v[p1 as usize - 'a' as usize] as usize;
                let o2 = state.v[p2 as usize - 'a' as usize] as usize;
                let byval_swap = state.v[state.p[o1] as usize];
                state.v[state.p[o1] as usize] = state.v[state.p[o2] as usize];
                state.v[state.p[o2] as usize] = byval_swap;
                let swap = state.p[o1];
                state.p[o1] = state.p[o2];
                state.p[o2] = swap;
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

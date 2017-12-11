/* Advent of Code 2017, Day 10 */

/* Jordan Justen : this file is public domain */

use std::iter::FromIterator;

struct State {
    pos: usize,
    skip: usize,
    seq: Vec<u8>,
    input: Vec<usize>,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let (seq, input) = read_input();
    let mut state = State {
        pos: 0,
        skip: 0,
        input: input.clone(),
        seq: Vec::from_iter(seq.iter().map(|b| *b as u8)),
    };
    knot(&mut state);

    println!("part1: {}", state.seq[0] as usize * state.seq[1] as usize);
}

fn knot(s: &mut State) {
    let len = s.seq.len();
    for u_8 in s.input.iter() {
        let i = *u_8 as usize;
        assert!(i <= len);
        let start: usize = s.pos;
        let end: usize = s.pos + i;
        for si in 0 .. i / 2 {
            let p1 = (start + si) % len;
            let p2 = (end - 1 - si) % len;
            let swap = s.seq[p1];
            s.seq[p1] = s.seq[p2];
            s.seq[p2] = swap;
        }
        s.pos += i + s.skip;
        s.skip += 1;
    }
}

fn part2() {
    let (seq, input) = read_input2();
    let mut state = State {
        pos: 0,
        skip: 0,
        input: input.clone(),
        seq: Vec::from_iter(seq.iter().map(|b| *b as u8)),
    };
    let mut suffix = vec!(17, 31, 73, 47, 23);
    state.input.append(&mut suffix);

    for _ in 0 .. 64 {
        knot(&mut state);
    }

    let mut dense = vec!(0u8; 16);
    for i in 0usize .. 16 {
        let base: usize = 16 * i;
        dense[i] = state.seq[base];
        for j in 1usize .. 16 {
            dense[i] ^= state.seq[base + j];
        }
    }
    let mut hash = String::new();
    for u in dense {
        hash += format!("{:02x}", u).as_str();
    }

    println!("part2: {}", hash);
}

#[allow(dead_code)]
fn read_input0() -> (Vec<usize>, Vec<usize>) {
    let seq: Vec<usize> = (0usize .. 5usize).collect();
    let input = vec!(3, 4, 1, 5);
    (seq, input)
}

#[allow(dead_code)]
fn read_input() -> (Vec<usize>, Vec<usize>) {
    let seq: Vec<usize> = (0usize .. 256usize).collect();
    let input = vec!(212,254,178,237,2,0,1,54,167,92,117,125,255,61,159,164);
    (seq, input)
}

#[allow(dead_code)]
fn read_input2() -> (Vec<usize>, Vec<usize>) {
    let seq: Vec<usize> = (0usize .. 256usize).collect();
    let input_str = "212,254,178,237,2,0,1,54,167,92,117,125,255,61,159,164";
    //let input_str = "1,2,3";
    let input = Vec::from_iter(input_str.chars().map(|c| c as usize));
    (seq, input)
}

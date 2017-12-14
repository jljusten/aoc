/* Advent of Code 2017, Day 14 */

/* Jordan Justen : this file is public domain */

use std::iter::FromIterator;
//use std::str::FromStr;

struct State {
    pos: usize,
    skip: usize,
    seq: Vec<u8>,
    input: Vec<usize>,
}

fn main() {
    let input = read_input1();
    let num_used =
        input.iter().
        map(|vb| vb.iter().
            map(|b| if *b { 1u32 } else { 0u32 }).
            sum::<u32>()).
        sum::<u32>();
    println!("part1: {}", num_used);

    let mut num_regions = 0;
    let mut regions = vec!(vec!(0u32; 128); 128);
    let mut growing = Vec::<(usize, usize)>::new();
    for row in 0 .. regions.len() {
        for col in 0 .. regions[0].len() {
            if regions[row][col] == 0 && input[row][col] {
                regions[row][col] = num_regions + 1;
                growing.push((row, col));
                num_regions += 1;
            }
            while growing.len() > 0 {
                let (row, col) = growing.pop().unwrap();
                if row > 0 && regions[row-1][col] == 0 && input[row-1][col] {
                    growing.push((row-1, col));
                    regions[row-1][col] = num_regions;
                }
                if row < (regions.len() - 1) && regions[row+1][col] == 0 &&
                    input[row+1][col] {
                    growing.push((row+1, col));
                    regions[row+1][col] = num_regions;
                }
                if col > 0 && regions[row][col-1] == 0 && input[row][col-1] {
                    growing.push((row, col-1));
                    regions[row][col-1] = num_regions;
                }
                if col < (regions[0].len() - 1) && regions[row][col+1] == 0 &&
                    input[row][col+1] {
                    growing.push((row, col+1));
                    regions[row][col+1] = num_regions;
                }
            }
        }
    }

    println!("part2: {}", num_regions);
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

#[allow(dead_code)]
fn read_input0() -> Vec<Vec<bool>> {
    /* Sample input */
    str_to_input("flqrgnkx")
}

fn str_to_hash(s: &str) -> Vec<bool> {
    let seq: Vec<usize> = (0usize .. 256usize).collect();
    let input = Vec::from_iter(s.chars().map(|c| c as usize));
    let mut state = State {
        pos: 0,
        skip: 0,
        input: input.clone(),
        seq: Vec::from_iter(seq.into_iter().map(|b| b as u8)),
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

    let mut result = Vec::new();

    for d in dense {
        let mut byte = d;
        for _ in 0 .. 8 {
            result.push(byte & 0x80 != 0);
            byte <<= 1;
        }
    }

    result
}

fn str_to_input(s: &str) -> Vec<Vec<bool>> {
    let strs: Vec<String> =
        (0 .. 128).collect::<Vec<_>>().iter().map(|i| format!("{}-{}", s, i)).collect();
    strs.iter().map(|s| str_to_hash(s)).collect()
}

#[allow(dead_code)]
fn read_input1() -> Vec<Vec<bool>> {
    /* My puzzle input */
    str_to_input("amgozmfv")
}

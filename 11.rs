/* Advent of Code 2017, Day 11 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

#[allow(dead_code)]
#[derive(Debug,Clone)]
enum Dir { N, NE, SE, S, SW, NW }

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input1();

    let pos = walk_path(&input);

    let dist = pos_dist(pos);

    println!("part1: step={} pos={:?} dist={}", input.len(), pos, dist);
}

fn walk_path(input: &Vec<Dir>) -> (i32, i32) {
    walk_from((0, 0), input)
}

fn walk_from(pos: (i32, i32), input: &[Dir]) -> (i32, i32) {
    let mut x = pos.0;
    let mut y = pos.1;

    for dir in input {
        match *dir {
            Dir::N => y += 2,
            Dir::NE => { x += 1; y += 1 },
            Dir::SE => { x += 1; y -= 1 },
            Dir::S => y -= 2,
            Dir::SW => { x -= 1; y -= 1 },
            Dir::NW => { x -= 1; y += 1 },
        }
    }

    (x, y)
}

fn pos_dist(pos: (i32, i32)) -> u32 {
    let hdist = pos.0.abs() as u32;
    let vdist = pos.1.abs() as u32;
    let vdist2 = if vdist > hdist { (vdist - hdist) / 2 } else { 0 };
    hdist + vdist2
}

fn part2() {
    let input = read_input1();
    let mut max_pos = (0i32, 0i32);
    let mut cur_pos = (0i32, 0i32);
    let mut max_dist = 0u32;
    let mut max_step_num = 0usize;

    for i in 0 .. input.len() {
        let new_pos = walk_from(cur_pos, &input[i..i+1]);
        let dist = pos_dist(new_pos);
        if dist > max_dist {
            max_dist = dist;
            max_pos = new_pos;
            max_step_num = i + 1;
        }
        cur_pos = new_pos;
    }

    println!("part2: step={} pos={:?} dist={}", max_step_num, max_pos, max_dist);
}

#[allow(dead_code)]
fn read_input0() -> Vec<Dir> {
    //vec!(Dir::NE; 3)
    //vec!(Dir::NE, Dir::NE, Dir::SW, Dir::SW)
    //vec!(Dir::NE, Dir::NE, Dir::S, Dir::S)
    vec!(Dir::SE, Dir::SW, Dir::SE, Dir::SW, Dir::SW)
}

#[allow(dead_code)]
fn read_input1() -> Vec<Dir> {
    let fname = "11.input";
    let mut input_file = File::open(fname).unwrap();
    let mut s = String::new();
    let r = input_file.read_to_string(&mut s);
    assert!(r.is_ok());
    s.trim_right().split(',').map(|s| match s {
        "n" => Dir::N,
        "ne" => Dir::NE,
        "se" => Dir::SE,
        "s" => Dir::S,
        "sw" => Dir::SW,
        "nw" => Dir::NW,
        _ => panic!("unknown dir {}", s)
    }).collect()
}

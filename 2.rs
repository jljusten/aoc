/* Advent of Code 2017, Day 2 */

/* Jordan Justen : this file is public domain */

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let s = read_input();
    part1(&s);
    part2(&s);
}

fn part1(s: &Vec<Vec<i32>>) {
    let mut csum = 0;
    for v in s {
        let (min, max) = min_max(v);
        let diff = max - min;
        csum += diff;
    }
    println!("part1 checksum: {}", csum);
}

fn min_max(v: &Vec<i32>) -> (i32, i32) {
    let mut it = v.iter();
    let mut min = it.next().unwrap();
    let mut max = min;
    for i in it {
        min = min.min(i);
        max = max.max(i);
    }
    (*min, *max)
}

fn part2(s: &Vec<Vec<i32>>) {
    let mut csum = 0;
    for v in s {
        csum += div_val(v);
    }
    println!("part2 checksum: {}", csum);
}

fn div_val(v: &Vec<i32>) -> i32 {
    for i in 0..v.len() {
        for j in 0..v.len() {
            if i == j {
                continue;
            }
            let (nc, dc) = (v[i], v[j]);
            if dc != 0 && (nc % dc) == 0 {
                return nc / dc;
            }
        }
    }
    0
}

fn read_input() -> Vec<Vec<i32>> {
    let fname = "2.input";
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    let mut s: Vec<Vec<i32>> = Vec::new();
    for (_, rln) in input_file.lines().enumerate() {
        let ln = rln.unwrap();
        let vln = Vec::from_iter(ln.split_whitespace().map(|s| i32::from_str(s).unwrap()));
        s.push(vln);
    }
    s
}

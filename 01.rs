/* Advent of Code 2020, Day 1 */

/* Jordan Justen : this file is public domain */

use std::collections::LinkedList;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input();
    let res = find_vec(2, 2020, &input);
    if let Some(v) = res {
        print(&v);
    }
}

fn find_vec(len: usize, target: usize, input: &Vec<usize>) -> Option<Vec<usize>> {
    let res = find_vec_inner(0, len, target, &input[..]);
    if let Some(l) = res {
        return Some(l.into_iter().collect())
    } else {
        None
    }
}

fn find_vec_inner(
    acc: usize,
    more: usize,
    target: usize,
    input: &[usize],
) -> Option<LinkedList<usize>> {
    if more == 0 {
        return if acc == target {
            Some(LinkedList::new())
        } else {
            None
        };
    }
    for i in 0..input.len() {
        if acc + input[i] <= target {
            let res = find_vec_inner(acc + input[i], more - 1, target, &input[i + 1..]);
            if let Some(r) = res {
                let mut r = r.clone();
                r.push_front(input[i]);
                return Some(r);
            }
        }
    }
    None
}

fn print(v: &Vec<usize>) {
    if v.len() > 0 {
        print!("{}", v[0]);
        for i in 1..v.len() {
            print!(" + {}", v[i]);
        }
        println!(" = {}", v.iter().fold(0, |acc, v| acc + v));
        print!("{}", v[0]);
        for i in 1..v.len() {
            print!(" * {}", v[i]);
        }
        println!(" = {}", v.iter().fold(1, |acc, v| acc * v));
    }
}

fn read_input() -> Vec<usize> {
    let fname = "01.input";
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    input_file
        .lines()
        .enumerate()
        .map(|s| usize::from_str(&s.1.unwrap()).unwrap())
        .collect()
}

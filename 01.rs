/* Advent of Code 2020, Day 1 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input();
    let mut res: Option<Vec<usize>> = None;
    'outer: for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            if input[i] + input[j] == 2020 {
                res = Some(vec![i, j]);
                break 'outer;
            }
        }
    }
    if let Some(v) = res {
        let v: Vec<usize> = v.into_iter().map(|i| input[i]).collect();
        print(&v);
    }
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

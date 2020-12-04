/* Advent of Code 2020, Day 1 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input();
    let mut res: Option<(usize, usize)> = None;
    'outer: for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            if input[i] + input[j] == 2020 {
                res = Some((i, j));
                break 'outer;
            }
        }
    }
    if let Some((i, j)) = res {
        let (v1, v2) = (input[i], input[j]);
        println!("{} + {} = {}", v1, v2, v1 + v2);
        println!("{} * {} = {}", v1, v2, v1 * v2);
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

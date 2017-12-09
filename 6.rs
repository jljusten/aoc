/* Advent of Code 2017, Day 6 */

/* Jordan Justen : this file is public domain */

use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let input = get_input();
    let mut states: HashMap<Vec<u32>,u32> = HashMap::new();
    let mut state = input.clone();
    states.insert(input.clone(), 0u32);
    assert!(states.contains_key(&input));
    assert!(states.contains_key(&state));
    loop {
        let mut redist_pos = 0;
        for i in 1 .. state.len() {
            if state[i] > state[redist_pos] {
                redist_pos = i;
            }
        }
        let redist_count = state[redist_pos];
        state[redist_pos] = 0;
        for _ in 0 .. redist_count {
            redist_pos = (redist_pos + 1) % state.len();
            state[redist_pos] += 1;
        }
        if states.contains_key(&state) {
            break;
        } else {
            let num_states = states.len() as u32;
            states.insert(state.clone(), num_states);
        }
    }
    println!("part1: {}", states.len());
    println!("part2: {}", states.len() as u32 - states[&state]);
}

fn get_input() -> Vec<u32> {
    //let input = "0 2 7 0";
    let input = "0 5 10 0 11 14 13 4 11 8 8 7 1 4 12 11";
    Vec::from_iter(input.split_whitespace().
                   map(|s| u32::from_str(s).unwrap()))
}

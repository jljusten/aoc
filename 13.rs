/* Advent of Code 2017, Day 13 */

/* Jordan Justen : this file is public domain */

use std::str::FromStr;

fn main() {
    let input = read_input1();
    let severity = calc_severity(0, &input);
    println!("part1: {}", severity);

    let mut delay = 0;
    for i in 0u32 .. {
        if !is_caught(i, &input) {
            delay = i;
            break;
        }
    }
    println!("part2: {}", delay);
}

fn calc_severity(delay: u32, depths: &Vec<u32>) -> u32 {
    let mut severity = 0;
    for time_offset in 0 .. depths.len() as u32 {
        let time = delay + time_offset;
        let layer = time_offset;
        severity += match depths[layer as usize] {
            0 => 0,
            1 => layer,
            depth => {
                if pos_at(time, depth) == 0 { layer * depth } else { 0 }
            }
        }
    }
    severity
}

fn is_caught(delay: u32, depths: &Vec<u32>) -> bool {
    for time_offset in 0 .. depths.len() as u32 {
        let time = delay + time_offset;
        let layer = time_offset;
        match depths[layer as usize] {
            0 => (),
            1 => { return true; },
            depth => {
                if pos_at(time, depth) == 0 {
                    return true;
                }
            }
        }
    }
    false
}

fn pos_at(time: u32, depth: u32) -> u32 {
    assert!(depth > 1);

    let loop_size = 2 * (depth - 1);
    let loop_pos = time % loop_size;

    if loop_pos < depth { loop_pos } else { 2 * depth - loop_pos - 1 }
}

#[allow(dead_code)]
fn read_input0() -> Vec<u32> {
    str_to_input("0: 3
                  1: 2
                  4: 4
                  6: 4")
}

fn str_to_input(s: &str) -> Vec<u32> {
    let data: Vec<Vec<u32>> =
        s.split('\n').map(|s| s.trim().split(": ").
                          map(|s| u32::from_str(s).unwrap()).
                          collect::<Vec<_>>()).
        collect();
    let mut input = Vec::<u32>::new();
    for d in data {
        assert!(d.len() == 0 || d.len() == 2);
        if d.len() < 2 {
            continue;
        }
        let layer = d[0];
        let depth = d[1];
        assert!(input.len() <= layer as usize);
        while input.len() < layer as usize {
            input.push(0u32)
        }
        input.push(depth);
    }
    input
}

#[allow(dead_code)]
fn read_input1() -> Vec<u32> {
    str_to_input("\
0: 3
1: 2
2: 6
4: 4
6: 4
8: 10
10: 6
12: 8
14: 5
16: 6
18: 8
20: 8
22: 12
24: 6
26: 9
28: 8
30: 8
32: 10
34: 12
36: 12
38: 8
40: 12
42: 12
44: 14
46: 12
48: 12
50: 12
52: 12
54: 14
56: 14
58: 14
60: 12
62: 14
64: 14
66: 17
68: 14
72: 18
74: 14
76: 20
78: 14
82: 18
86: 14
90: 18
92: 14")
}

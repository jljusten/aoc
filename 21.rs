/* Advent of Code 2017, Day 21 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Rule {
    Pat2 { i: [u16;8], o: u16 },
    Pat3 { i: [u16;8], o: u16 },
}

fn main() {
    let input = read_input1();
    println!("part1: {}", pixels_set_after_n_iterations(&input, 5));
    println!("part2: {}", pixels_set_after_n_iterations(&input, 18));
}

fn pixels_set_after_n_iterations(input: &Vec<Rule>, n: usize) -> u32 {
    let mut state =
        vec!(vec!('.', '#', '.'), vec!('.', '.', '#'), vec!('#', '#', '#'));
    for _ in 0 .. n {
        let (chunk_sz, mut as_u16s) = text_to_u16s(&state);
        let hsz = state.len() / chunk_sz;
        enhance(input, &mut as_u16s, chunk_sz);
        state = u16s_to_text(&as_u16s, hsz, chunk_sz + 1);
    }
    let (_, final_u16s) = text_to_u16s(&state);
    final_u16s.iter().map(|v| v.count_ones()).sum::<u32>()
}

#[allow(dead_code)]
fn print_state(t: &Vec<Vec<char>>) {
    for l in t {
        println!("{}", l.iter().collect::<String>());
    }
}

fn enhance(rules: &Vec<Rule>, i: &mut Vec<u16>, sz: usize) {
    for ch in i {
        for rule in rules {
            match *rule {
                Rule::Pat2 { i, o } => {
                    if sz == 2 && i.iter().any(|v| *ch == *v) {
                        *ch = o;
                        break;
                    }
                },
                Rule::Pat3 { i, o } => {
                    if sz == 3 && i.iter().any(|v| *ch == *v) {
                        *ch = o;
                        break;
                    }
                },
            }
        }
    }
}

fn text_to_u16s(t: &Vec<Vec<char>>) -> (usize, Vec<u16>) {
    let chunk_sz = if t.len() % 2 == 0 { 2 } else { 3 };
    let num_chunks = t.len() / chunk_sz;
    let mut result = Vec::<u16>::new();
    for v in 0 .. num_chunks {
        for h in 0 .. num_chunks {
            result.push(image_chunk_to_u16(t, v, h, chunk_sz));
        }
    }
    (chunk_sz, result)
}

fn image_chunk_to_u16(t: &Vec<Vec<char>>, v: usize, h: usize,
                      chunk_sz: usize) -> u16 {
    let mut result = 0;
    let v_start = chunk_sz * v;
    let h_start = chunk_sz * h;
    for v in v_start .. v_start + chunk_sz {
        for c in t[v][h_start .. h_start + chunk_sz].iter() {
            result = (result << 1) + (if *c == '#' { 1 } else { 0 });
        }
    }
    result
}

fn u16s_to_text(t: &Vec<u16>, hsz: usize, sz: usize) -> Vec<Vec<char>> {
    assert!(t.len() == hsz * hsz);
    let mut result: Vec<Vec<char>> = Vec::new();
    for (i, ch) in t.iter().enumerate() {
        let mut bit = sz * sz;
        let v_start = sz * (i / hsz);
        for v in v_start .. v_start + sz {
            if result.len() <= v {
                result.push(Vec::<char>::new());
            }
            for _ in 0 .. sz {
                bit -= 1;
                let c = if *ch & (1 << bit) != 0 { '#' } else { '.' };
                result[v].push(c);
            }
        }
    }
    result
}

#[allow(dead_code)]
fn read_input0() -> Vec<Rule> {
    let sample_lines: Vec<String> ="
        ../.# => ##./#../...
        .#./..#/### => #..#/..../..../#..#
        ".split('\n').map(|s| String::from(s.trim())).
        collect();
    lines_to_rules(&sample_lines)
}

fn lines_to_rules(ls: &Vec<String>) -> Vec<Rule> {
    //println!("{:?}", ls);
    ls.iter().map(|s| s.trim()).filter(|s| s.len() > 1).
        map(|s| s.split("=>").map(|s| String::from(s.trim())).
            collect::<Vec<_>>()).
        map(|vs| text_to_rule(&vs)).collect()
}

/* 3 2   1 3   0 1   2 0   2 3
 * 1 0   0 2   2 3   3 1   0 1
 */
static PATTERNS2X2: [[u8;4];4] = [
    //[ 3, 2,   1, 0 ],
    [ 1, 3,   0, 2 ],
    [ 0, 1,   2, 3 ],
    [ 2, 0,   3, 1 ],
    [ 2, 3,   0, 1 ],
];

/* 8 7 6   2 5 8   0 1 2   6 3 0   6 7 8
 * 5 4 3   1 4 7   3 4 5   7 4 1   3 4 5
 * 2 1 0   0 3 6   6 7 8   8 5 2   0 1 2
 */
static PATTERNS3X3: [[u8;9];4] = [
    //[ 8, 7, 6,   5, 4, 3,   2, 1, 0 ],
    [ 2, 5, 8,   1, 4, 7,   0, 3, 6 ],
    [ 0, 1, 2,   3, 4, 5,   6, 7, 8 ],
    [ 6, 3, 0,   7, 4, 1,   8, 5, 2 ],
    [ 6, 7, 8,   3, 4, 5,   0, 1, 2 ],
];

fn text_to_rule(t: &Vec<String>) -> Rule {
    assert!(t.len() == 2);
    let (isz, ival) = str_to_bin(&t[0]);
    let (osz, oval) = str_to_bin(&t[1]);
    let mut ipats = [0u16; 8];
    ipats[0] = ival;
    for i in 1 .. 8 {
        let src = ipats[((i - 1) / 4) << 2];
        let pat_idx = (i - 1) % 4;
        let pat: &[u8] =
            if isz == 2 { &PATTERNS2X2[pat_idx] }
            else { &PATTERNS3X3[pat_idx] };
        for b in 0 .. pat.len() {
            ipats[i] =
                (ipats[i] << 1) +
                if src & (1u16 << pat[b]) != 0 { 1 } else { 0 };
        }
    }
    assert!(isz + 1 == osz);
    match isz {
        2 => {
            Rule::Pat2 { i: ipats, o: oval }
        },
        3 => Rule::Pat3 { i: ipats, o: oval },
        _ => panic!("Unsupported pattern {}", t[0]),
    }
}

fn str_to_bin(s: &String) -> (usize, u16) {
    let size = match s.len() {
        5 => 2,
        11 => 3,
        19 => 4,
        _ => panic!("Unsupported pattern {}", s),
    };
    let mut v = 0;
    for (i, c) in s.chars().enumerate() {
        if i % (size + 1) == size {
            //println!("{} {} {}", i, c, s);
            assert!(c == '/');
            continue;
        }
        v = (v << 1) + (if c == '#' { 1 } else { 0 });
    }
    (size, v)
}

#[allow(dead_code)]
fn read_input1() -> Vec<Rule> {
    let fname = "21.input";
    let mut input_file = File::open(fname).unwrap();
    let mut s = String::new();
    let r = input_file.read_to_string(&mut s);
    assert!(r.is_ok());
    let input =s.split('\n').map(|s| String::from(s)).collect();
    lines_to_rules(&input)
}

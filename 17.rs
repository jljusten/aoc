/* Advent of Code 2017, Day 17 */

/* Jordan Justen : this file is public domain */

fn main() {
    let input = read_input1();
    let mut state = vec!(0u32; 1);
    state.reserve(50000000);
    let mut pos = 0usize;
    let mut len = 1;

    for i in 1 .. 2018 {
        pos = (pos + input) % len;
        state.insert(pos + 1, i);
        pos += 1;
        len += 1;
    }

    println!("part1: {}", state[pos + 1]);
}

#[allow(dead_code)]
fn read_input0() -> usize {
    3
}

#[allow(dead_code)]
fn read_input1() -> usize {
    367
}

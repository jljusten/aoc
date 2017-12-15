/* Advent of Code 2017, Day 15 */

/* Jordan Justen : this file is public domain */

fn main() {
    let (a_start, b_start) = read_input1();
    let mut a = a_start;
    let mut b = b_start;

    let mut counter = 0;
    for _ in 0 .. 40000000 {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;
        if a & 0xffff == b & 0xffff {
            counter += 1;
        }
    }

    println!("part1: {}", counter);

    counter = 0;
    a = a_start;
    b = b_start;
    for _ in 0 .. 5000000 {
        loop {
            a = (a * 16807) % 2147483647;
            if a & 3 == 0 {
                break;
            }
        }
        loop {
            b = (b * 48271) % 2147483647;
            if b & 7 == 0 {
                break;
            }
        }
        if a & 0xffff == b & 0xffff {
            counter += 1;
        }
    }

    println!("part2: {}", counter);
}

#[allow(dead_code)]
fn read_input0() -> (u64, u64) {
    /* Sample input */
    (65, 8921)
}

#[allow(dead_code)]
fn read_input1() -> (u64, u64) {
    /* My puzzle input */
    (722, 354)
}

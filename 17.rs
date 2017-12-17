/* Advent of Code 2017, Day 17 */

/* Jordan Justen : this file is public domain */

fn main() {
    const COUNT: usize = 50000000;
    let mut prev = Vec::new();
    let mut next = Vec::new();
    prev.reserve(2018.min(COUNT + 1));
    next.reserve(2018.min(COUNT + 1));
    prev.push(0);
    next.push(0);

    let input = read_input1();
    let mut pos = 0usize;
    let mut idx = 0usize;
    let mut len = 1;

    for i in 1 .. 2018.min(COUNT + 1) {
        let next_idx = (idx + input) % len;
        if next_idx > idx {
            for _ in 0 .. next_idx - idx {
                pos = next[pos];
            }
        } else {
            for _ in 0 .. idx - next_idx {
                pos = prev[pos];
            }
        }
        idx = next_idx;
        prev.push(0);
        next.push(0);
        prev[i] = pos;
        let new_node_next = next[pos];
        next[i] = new_node_next;
        next[pos] = i;
        prev[new_node_next] = i;
        pos = i;
        idx += 1;
        len += 1;
    }

    println!("part1: {}", next[pos]);

    /* For part 2 we no longer need to keep track of the full list. We
     * only care what follows item `0`. Item `0` is always at the
     * front, so we just watch for new items that would have been
     * inserted just after item `0`. Whenever this happens, we just
     * note what the new item would have been after `0`.
     */
    let mut part2 = next[0];
    for i in 2018 .. COUNT + 1 {
        let next_idx = (idx + input) % len;
        idx = next_idx;
        if idx == 0 {
            part2 = i;
        }
        idx += 1;
        len += 1;
    }

    println!("part2: {}", part2);
}

#[allow(dead_code)]
fn read_input0() -> usize {
    3
}

#[allow(dead_code)]
fn read_input1() -> usize {
    367
}

/* Advent of Code 2017, Day 24 */

/* Jordan Justen : this file is public domain */

use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = read_input1();
    let (max_all, max_longest) = build_bridges(&input);
    println!("part1: {}", max_all);
    println!("part2: {}", max_longest);
}

fn build_bridges(input: &Vec<(u32, u32)>) -> (u32, u32) {
    let mut avail = vec![true; input.len()];
    let mut by_size = HashMap::<u32, Vec<usize>>::new();

    for i in 0 .. input.len() {
        by_size.entry(input[i].0).or_insert(Vec::new()).push(i);
        if input[i].0 != input[i].1 {
            by_size.entry(input[i].1).or_insert(Vec::new()).push(i);
        }
    }

    let mut max = 0;
    let mut longest_len = 0;
    let mut max_strength_longest = 0;
    let mut needs = vec![0u32];
    let mut pos_stk = vec![0usize];
    let mut comps = vec![];
    let mut strengths = vec![];
    loop {
        if needs.len() == 0 {
            break;
        }
        let need = *needs.last().unwrap();
        let pos = {
            let pos = pos_stk.last_mut().unwrap();
            let cur_pos = *pos;
            *pos += 1;
            cur_pos
        };

        let num_available_for_needed_num =
            if by_size.contains_key(&need) {
                by_size.get(&need).unwrap().len()
            } else {
                0
            };

        if pos < num_available_for_needed_num {
            let candidate = by_size.get(&need).unwrap()[pos];
            if avail[candidate] {
                avail[candidate] = false;
                let other =
                    if input[candidate].0 == need {
                        input[candidate].1
                    } else {
                        input[candidate].0
                    };
                needs.push(other);
                pos_stk.push(0);
                comps.push(candidate);
                strengths.push(input[candidate].0 + input[candidate].1);
            } else {
                continue;
            }
        } else {
            let bridge_strength = strengths.iter().sum::<u32>();
            max = u32::max(max, bridge_strength);
            if strengths.len() > longest_len {
                longest_len = strengths.len();
                max_strength_longest = bridge_strength;
            } else if strengths.len() == longest_len {
                max_strength_longest = u32::max(max_strength_longest, bridge_strength);
            }
            match comps.pop() {
                Some(freed) => { avail[freed] = true; },
                _ => (),
            }
            needs.pop();
            pos_stk.pop();
            strengths.pop();
        }
    }

    (max, max_strength_longest)
}

#[allow(dead_code)]
fn read_input0() -> Vec<(u32, u32)> {
    let input ="\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10".split('\n').map(|s| String::from(s)).collect();
    lines_to_comps(&input)
}

fn lines_to_comps(ls: &Vec<String>) -> Vec<(u32, u32)> {
    ls.iter().map(|s| s.trim()).filter(|l| l.len() > 0).map(|s| {
        let ports =
            s.split('/').map(|s| u32::from_str(s).unwrap()).
            collect::<Vec<_>>();
        assert!(ports.len() == 2);
        (ports[0], ports[1])
    }).collect()
}

#[allow(dead_code)]
fn read_input1() -> Vec<(u32, u32)> {
    let input ="\
25/13
4/43
42/42
39/40
17/18
30/7
12/12
32/28
9/28
1/1
16/7
47/43
34/16
39/36
6/4
3/2
10/49
46/50
18/25
2/23
3/21
5/24
46/26
50/19
26/41
1/50
47/41
39/50
12/14
11/19
28/2
38/47
5/5
38/34
39/39
17/34
42/16
32/23
13/21
28/6
6/20
1/30
44/21
11/28
14/17
33/33
17/43
31/13
11/21
31/39
0/9
13/50
10/14
16/10
3/24
7/0
50/50".split('\n').map(|s| String::from(s)).collect();
    lines_to_comps(&input)
}

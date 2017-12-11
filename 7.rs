/* Advent of Code 2017, Day 7 */

/* Jordan Justen : this file is public domain */

use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let input = get_input();
    let mut weights = Vec::<u32>::new();
    let mut programs = Vec::<String>::new();
    let mut program_pos = HashMap::<String, usize>::new();
    let mut children_names = Vec::<Vec<String>>::new();

    /* First map string based input into data structures */
    for line in input.iter() {
        let prog_name = &line[0];
        program_pos.insert(prog_name.clone(), programs.len());
        let parens: &[_] = &['(', ')'];
        let weight = u32::from_str(line[1].trim_matches(parens)).unwrap();
        weights.push(weight);
        if line.len() > 2 {
            let ch_names: Vec<String> =
                line[3..].iter().
                    map(|s| String::from(s.trim_right_matches(','))).
                    collect::<Vec<_>>();
            children_names.push(ch_names);
        } else {
            children_names.push(Vec::new());
        }
        programs.push(prog_name.clone());
    }

    /* Remap children from names to positions */
    let children_pos: Vec<Vec<usize>> =
        children_names.iter().map(|ns| ns.iter().map(|n| program_pos[n]).collect()).
        collect::<Vec<_>>();

    /* Mark parent positions. -1 indicates no parents */
    let mut parents = vec!(-1i32; programs.len());
    for (parent_pos, ch_pos) in children_pos.iter().enumerate() {
        for c in ch_pos {
            assert!(parents[*c] < 0);
            parents[*c] = parent_pos as i32;
        }
    }

    /* Find roots. According to puzzle, there should only be 1 root. */
    let mut roots = Vec::<usize>::new();
    for i in 0usize .. programs.len() {
        if parents[i] < 0 {
            roots.push(i);
        }
    }
    assert!(roots.len() == 1);
    println!("part1: {}", programs[roots[0]]);

    /* Calculate total weight of each program  */
    let mut num_weight_calc = 0;
    let mut weight_ready = children_pos.iter().map(|ns| ns.len() < 1).collect::<Vec<_>>();
    let mut weight_done = vec!(false; programs.len());
    let mut total_weight = vec!(0u32; programs.len());
    while num_weight_calc < programs.len() {
        let old_num_calc = num_weight_calc;
        for i in 0 .. programs.len() {
            if !weight_done[i] && weight_ready[i] {
                total_weight[i] = weights[i] +
                    children_pos[i].iter().map(|p| total_weight[*p]).sum::<u32>();
                weight_done[i] = true;
                num_weight_calc += 1;
            }
        }
        for i in 0 .. programs.len() {
            if !weight_ready[i] {
                weight_ready[i] = children_pos[i].iter().all(|p| weight_done[*p]);
            }
        }
        assert!(old_num_calc < num_weight_calc);
    }

    /* Look for inbalanced programs based on the total weights.
     * According to the puzzle description, only one needs to be
     * adjusted, and we make sure it is the one with the lowest total
     * weight indicating the closest to the top of the stack.
     */
    let mut min_bad_pos = 0;
    let mut weight_adjust = 0i32;
    let mut bad_pos_found = false;
    for i in 0 .. programs.len() {
        let mut weight_map = HashMap::new();
        for ci in children_pos[i].iter() {
            *weight_map.entry(total_weight[*ci]).or_insert(0) += 1;
        }
        assert!(weight_map.len() <= 2);
        if weight_map.len() > 1 {
            let mut bad_pos = 0;
            let mut good_pos = 0;
            for ci in children_pos[i].iter() {
                if weight_map[&total_weight[*ci]] == 1 {
                    bad_pos = *ci;
                } else {
                    good_pos = *ci;
                }
            }
            let adjust =
                total_weight[good_pos] as i32 - total_weight[bad_pos] as i32;
            if !bad_pos_found {
                min_bad_pos = bad_pos;
                weight_adjust = adjust;
                bad_pos_found = true;
            } else {
                assert!(adjust == weight_adjust);
                if total_weight[bad_pos] < total_weight[min_bad_pos] {
                    min_bad_pos = bad_pos;
                }
            }
        }
    }

    let old_weight = weights[min_bad_pos];
    let new_weight = old_weight as i32 + weight_adjust;
    println!("part2: {}, {} -> {}", programs[min_bad_pos], old_weight,
             new_weight);
}

fn get_input() -> Vec<Vec<String>> {
    let fname = "7.input";
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    let mut s: Vec<Vec<String>> = Vec::new();
    for (_, rln) in input_file.lines().enumerate() {
        let ln = rln.unwrap();
        let vln = Vec::from_iter(ln.split_whitespace().
                                 map(|s| String::from(s)));
        s.push(vln);
    }
    s
}

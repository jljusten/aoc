/* Advent of Code 2017, Day 12 */

/* Jordan Justen : this file is public domain */

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let input = read_input1();
    let mut paths = vec!(vec!(false; input.len()); input.len());
    for (i, connections) in input.iter().enumerate() {
        for j in connections {
            paths[i][*j] = true;
        }
    }

    /* This is pretty gross. :( */
    loop {
        let mut progress = false;
        for i in 0 .. paths.len() {
            for j in 0 .. paths.len() {
                if paths[i][j] {
                    for k in 0 .. paths.len() {
                        if paths[j][k] && !paths[i][k] {
                            paths[i][k] = true;
                            progress = true;
                        }
                    }
                }
            }
        }
        if !progress {
            break;
        }
        println!("progress");
    }

    let result1 =
        paths[0].iter().map(|b| if *b { 1 } else { 0 }).
        collect::<Vec<u32>>().iter().sum::<u32>();
    println!("part1: {}", result1);

    let mut groups = vec!(false; input.len());
    for i in 0 .. input.len() {
        let mut new = true;
        for j in 0 .. i {
            if paths[j][i] {
                new = false;
                break;
            }
        }
        if new {
            groups[i] = true;
        }
    }

    let result2 =
        groups.into_iter().map(|b| if b { 1 } else { 0 }).
        collect::<Vec<u32>>().iter().sum::<u32>();
    println!("part2: {}", result2);
}

#[allow(dead_code)]
fn read_input0() -> Vec<Vec<usize>> {
    let sample_lines: Vec<String> = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5".split('\n').map(|s| s.replace(&" <->", &",")).collect();
    let sample_line_vecs: Vec<Vec<&str>> =
        sample_lines.iter().map(|s| s.split(", ").collect::<Vec<_>>()).
        collect();
    let sample_nums: Vec<Vec<usize>> =
        sample_line_vecs.iter().
        map(|anums| anums.iter().
            map(|anum| usize::from_str(anum).unwrap()).collect::<Vec<usize>>()).
        collect();
    for (i, nums) in sample_nums.iter().enumerate() { assert!(i == nums[0]); }
    sample_nums.iter().map(|nums| nums[1..].iter().
                           map(|n| *n).collect::<Vec<_>>()).
        collect::<Vec<_>>()
}

#[allow(dead_code)]
fn read_input1() -> Vec<Vec<usize>> {
    let fname = "12.input";
    let mut input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    let sample_lines_from_file: Vec<String> =
        input_file.lines().map(|l| String::from(l.unwrap().
                                                trim().replace(" <->", ","))).
        collect();
    let sample_lines: Vec<String> =
        sample_lines_from_file.into_iter().filter(|l| l.len() > 0).collect();
    let sample_line_vecs: Vec<Vec<&str>> =
        sample_lines.iter().map(|s| s.split(", ").collect::<Vec<_>>()).
        collect();
    let sample_nums: Vec<Vec<usize>> =
        sample_line_vecs.iter().
        map(|anums| anums.iter().
            map(|anum| usize::from_str(anum).unwrap()).collect::<Vec<usize>>()).
        collect();
    for (i, nums) in sample_nums.iter().enumerate() { assert!(i == nums[0]); }
    sample_nums.iter().map(|nums| nums[1..].iter().
                           map(|n| *n).collect::<Vec<_>>()).
        collect::<Vec<_>>()
}

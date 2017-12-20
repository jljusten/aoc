/* Advent of Code 2017, Day 20 */

/* Jordan Justen : this file is public domain */

use std::cmp::min;
use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Clone,Debug)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone,Debug)]
struct Point {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

fn main() {
    let input = read_input1();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &Vec<Point>) -> usize {
    let mut watching: Vec<usize> = (0 .. input.len()).collect();

    let mut min_a = i64::max_value();
    for i in input {
        min_a = min(min_a, i.a.x.abs() + i.a.y.abs() + i.a.z.abs());
    }
    watching.retain(|i| {
        let p = &input[*i];
        p.a.x.abs() + p.a.y.abs() + p.a.z.abs() == min_a
    });

    let mut min_v = i64::max_value();
    for i in watching.iter() {
        let p = &input[*i];
        min_v = min(min_v, p.v.x.abs() + p.v.y.abs() + p.v.z.abs());
    }
    watching.retain(|i| {
        let p = &input[*i];
        p.v.x.abs() + p.v.y.abs() + p.v.z.abs() == min_v
    });

    assert!(watching.len() == 1);
    watching[0]
}

fn step_point(p: &mut Point) {
    p.v.x += p.a.x;
    p.p.x += p.v.x;
    p.v.y += p.a.y;
    p.p.y += p.v.y;
    p.v.z += p.a.z;
    p.p.z += p.v.z;
}

fn part2(input: &Vec<Point>) -> usize {
    let mut state = input.clone();
    let mut collided = vec!(false; input.len());
    let mut no_collision_time = 0;
    let mut watching: Vec<usize> = (0 .. input.len()).collect();
    loop {
        watching.sort_by(|a, b| order_vec3(&state[*a].p, &state[*b].p));
        for i in 0 .. watching.len() - 1 {
            let a = watching[i];
            let b = watching[i + 1];
            if order_vec3(&state[a].p, &state[b].p) == Ordering::Equal {
                collided[a] = true;
                collided[b] = true;
            }
        }
        let old_watch_len = watching.len();
        watching.retain(|i| !collided[*i]);
        if old_watch_len > watching.len() {
            no_collision_time = 0;
        } else  {
            no_collision_time += 1;
        }
        for i in watching.iter() {
            step_point(&mut state[*i]);
        }
        //println!("watching {:?}", watching.iter().take(10).collect::<Vec<_>>());
        //println!("watching {}", watching.len());
        if no_collision_time > 1000 {
            break;
        }
    }
    watching.len()
}

fn order_vec3(a: &Vec3, b: &Vec3) -> Ordering {
    if a.z != b.z {
        a.z.cmp(&b.z)
    } else if a.y != b.y {
        a.y.cmp(&b.y)
    } else {
        a.x.cmp(&b.x)
    }
}

#[allow(dead_code)]
fn read_input0() -> Vec<Point> {
    let sample_lines: Vec<String> = "\
        p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
        p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>
        ".split('\n').map(|s| String::from(s)).collect();
    lines_to_points(&sample_lines)
}

fn lines_to_points(ls: &Vec<String>) -> Vec<Point> {
    ls.iter().map(|s| s.trim()).filter(|l| l.len() > 0).map(|s| {
        let p = str_to_vec3(s, 0);
        let vpos = s.find("v=").unwrap();
        let v = str_to_vec3(s, vpos);
        let apos = s.find("a=").unwrap();
        let a = str_to_vec3(s, apos);
        Point { p: p, v: v, a: a }
    }).collect()
}

fn str_to_vec3(s: &str, o: usize) -> Vec3 {
    let ss = if o > 0 { s.get(o..).unwrap() } else { s };
    let start = ss.find("<").unwrap() + 1;
    let end = ss.find(">").unwrap();
    let vec3_str = ss.get(start..end).unwrap();
    let vec3 =
        vec3_str.split(',').map(|s| i64::from_str(s.trim()).unwrap()).
        collect::<Vec<_>>();
    Vec3 { x: vec3[0], y: vec3[1], z: vec3[2] }
}

#[allow(dead_code)]
fn read_input1() -> Vec<Point> {
    let fname = "20.input";
    let mut input_file = File::open(fname).unwrap();
    let mut s = String::new();
    let r = input_file.read_to_string(&mut s);
    assert!(r.is_ok());
    let input =s.split('\n').map(|s| String::from(s)).collect();
    lines_to_points(&input)
}

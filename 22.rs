/* Advent of Code 2017, Day 22 */

/* Jordan Justen : this file is public domain */

use std::collections::HashMap;

#[derive(Clone,Copy,Debug,PartialEq)]
enum Health {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

type World = HashMap<(i32, i32), Health>;

#[derive(Clone,Copy,Debug)]
enum State {
    Down { x: i32, y: i32 },
    Up { x: i32, y: i32 },
    Left { x: i32, y: i32 },
    Right { x: i32, y: i32 },
}

fn main() {
    let world = read_input1();
    part1(&world);
    part2(&world);
}

#[allow(dead_code)]
fn part1(input: &World) {
    let mut world = input.clone();
    let mut state = State::Up { x: 0, y: 0 };
    let mut burst_caused_infection = 0;
    for _ in 0 .. 10000 {
        let infected = world.len();
        state = step_part1(&mut world, &mut state);
        if world.len() > infected {
            burst_caused_infection += 1;
        }
    }
    println!("part1: {}", burst_caused_infection);
}

fn step_part1(world: &mut World, state: &State) -> State {
    let (x, y) = get_pos(state);
    let was_infected = world.contains_key(&(x, y));
    let turned = match (was_infected, state) {
        (true, &State::Up { x, y }) | (false, &State::Down { x, y }) =>
            State::Right { x, y },
        (true, &State::Right { x, y }) | (false, &State::Left { x, y }) =>
            State::Down { x, y },
        (true, &State::Down { x, y }) | (false, &State::Up { x, y }) =>
            State::Left { x, y },
        (true, &State::Left { x, y }) | (false, &State::Right { x, y }) =>
            State::Up { x, y },
    };
    if was_infected {
        world.remove(&(x, y));
    } else {
        world.insert((x, y), Health::Infected);
    }
    match turned {
        State::Up { x, y } => State::Up { x, y: y + 1 },
        State::Down { x, y } => State::Down { x, y: y - 1 },
        State::Right { x, y } => State::Right { x: x + 1, y },
        State::Left { x, y } => State::Left { x: x - 1, y },
    }
}

fn part2(input: &World) {
    let mut world = input.clone();
    let mut state = State::Up { x: 0, y: 0 };
    let mut burst_caused_infection = 0;
    for _ in 0 .. 10000000 {
        let result = step_part2(&mut world, &mut state);
        state = result.0;
        if result.1 {
            burst_caused_infection += 1;
        }
    }
    println!("part2: {}", burst_caused_infection);
}

fn step_part2(world: &mut World, state: &State) -> (State, bool) {
    let (x, y) = get_pos(state);
    let health =
        if world.contains_key(&(x, y)) {
            *world.get(&(x, y)).unwrap()
        } else {
            Health::Clean
        };
    let turned = match health {
        Health::Clean => match *state {
            State::Up { x, y } => State::Left { x, y },
            State::Left { x, y } => State::Down { x, y },
            State::Down { x, y } => State::Right { x, y },
            State::Right { x, y } => State::Up { x, y },
        },
        Health::Weakened => *state,
        Health::Infected => match *state {
            State::Up { x, y } => State::Right { x, y },
            State::Left { x, y } => State::Up { x, y },
            State::Down { x, y } => State::Left { x, y },
            State::Right { x, y } => State::Down { x, y },
        },
        Health::Flagged => match *state {
            State::Up { x, y } => State::Down { x, y },
            State::Left { x, y } => State::Right { x, y },
            State::Down { x, y } => State::Up { x, y },
            State::Right { x, y } => State::Left { x, y },
        },
    };
    let new_health = match health {
        Health::Clean => Health::Weakened,
        Health::Weakened => Health::Infected,
        Health::Infected => Health::Flagged,
        Health::Flagged => Health::Clean,
    };
    match new_health {
        Health::Clean => {
            world.remove(&(x, y));
        },
        Health::Weakened | Health::Infected | Health::Flagged => {
            world.insert((x, y), new_health);
        },
    }
    let moved = match turned {
        State::Up { x, y } => State::Up { x, y: y + 1 },
        State::Down { x, y } => State::Down { x, y: y - 1 },
        State::Right { x, y } => State::Right { x: x + 1, y },
        State::Left { x, y } => State::Left { x: x - 1, y },
    };
    (moved, new_health == Health::Infected)
}

#[allow(dead_code)]
fn get_pos(state: &State) -> (i32, i32) {
    match state {
        &State::Down { x, y } => ( x, y ),
        &State::Up { x, y } => ( x, y ),
        &State::Left { x, y } => ( x, y ),
        &State::Right { x, y } => ( x, y ),
    }
}

#[allow(dead_code)]
fn read_input0() -> World {
    let mut w: World = HashMap::new();
    w.insert((1, 1), Health::Infected);
    w.insert((-1, 0), Health::Infected);
    w
}

#[allow(dead_code)]
fn read_input1() -> World {
    let lines: Vec<String> ="
#..#...##.#.###.#.#.#...#
.####....#..##.#.##....##
...#..#.#.#......##..#..#
##.####.#.##........#...#
##.#....##..#.####.###...
#..#..###...#.#..##..###.
.##.##..#.####.#.#.....##
#....#......######...#...
..#.#.##.#..#...##.#.####
#.#..#.....#..####.#.#.##
...##.#..##.###.###......
###..#.####.#..#####..#..
...##.##.#.###.#..##.#.##
.####.#.##.#####.##.##..#
#.##.#...##.#.###.###..#.
..#.#..#..#..##..###...##
##.##.#..#.###....###..##
.#...###..#####.#..#.#.##
....##..####.##...#..#.##
#..#..###..#..###...#..##
.##.#.###..####.#.#..##.#
..###.#....#...###...##.#
.#...##.##.####...##.####
###.#.#.####.##.###..#...
#.#######.#######..##.#.#
        ".split('\n').map(|s| String::from(s.trim())).filter(|s| s.len() > 0).
        collect();
    let num_lines = lines.len();
    assert!(num_lines > 0);
    let ln_len = lines[0].len();
    assert!(lines.len() % 2 == 1 && ln_len % 2 == 1);
    assert!(lines.iter().all(|l| l.len() == ln_len));
    let mut w: World = HashMap::new();
    for (ln_num, ln) in lines.iter().enumerate() {
        let y: i32 = (num_lines as i32 / 2) - ln_num as i32;
        for (ch_num, ch) in ln.chars().enumerate() {
            if ch == '#' {
                let x: i32 = ch_num as i32 - (ln_len as i32 / 2);
                w.insert((x, y), Health::Infected);
            }
        }
    }
    w
}

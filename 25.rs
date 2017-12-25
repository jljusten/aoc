/* Advent of Code 2017, Day 25 */

/* Jordan Justen : this file is public domain */

use std::collections::HashSet;

#[derive(Debug)]
enum Dir { Left, Right }

#[derive(Debug)]
struct Action {
    write: u8,
    dir: Dir,
    next: char,
}

#[derive(Debug)]
struct Inst {
    read: [Action; 2],
}

#[derive(Debug)]
struct Program {
    start: char,
    inst: Vec<Inst>,
    count: u32,
}

#[derive(Debug)]
struct State {
    state: u8,
    cursor: i32,
    tape: HashSet<i32>,
}

fn main() {
    let input = read_input1();
    let mut state = State {
        state: (input.start as u32 - 'A' as u32) as u8,
        cursor: 0,
        tape: HashSet::new(),
    };
    for _ in 0 .. input.count {
        run_step(&input, &mut state);
    }
    println!("part1: {}", state.tape.len());
}

fn run_step(prog: &Program, state: &mut State) {
    let read = if state.tape.contains(&state.cursor) { 1u8 } else { 0u8 };
    let inst = &prog.inst[state.state as usize].read[read as usize];
    match inst.write {
        0u8 => {
            if state.tape.contains(&state.cursor) {
                state.tape.remove(&state.cursor);
            }
        },
        1u8 => {
            if !state.tape.contains(&state.cursor) {
                state.tape.insert(state.cursor);
            }
        },
        _ => panic!("unhandled write value {}", inst.write),
    }
    match inst.dir {
        Dir::Left => state.cursor -= 1,
        Dir::Right => state.cursor += 1,
    }
    state.state = (inst.next as u32 - 'A' as u32) as u8;
}

#[allow(dead_code)]
fn read_input0() -> Program {
    Program {
        start: 'A',
        inst: vec![
            Inst {
                read: [
                    Action {
                        write: 1,
                        dir: Dir::Right,
                        next: 'B',
                    },
                    Action {
                        write: 0,
                        dir: Dir::Left,
                        next: 'B',
                    }
                ]
            },
            Inst {
                read: [
                    Action {
                        write: 1,
                        dir: Dir::Left,
                        next: 'A',
                    },
                    Action {
                        write: 1,
                        dir: Dir::Right,
                        next: 'A',
                    }
                ]
            },
        ],
        count: 6,
    }
}

#[allow(dead_code)]
fn read_input1() -> Program {
    Program {
        start: 'A',
        inst: vec![
            Inst { /* A */
                read: [
                    Action {
                        write: 1,
                        dir: Dir::Right,
                        next: 'B',
                    },
                    Action {
                        write: 0,
                        dir: Dir::Left,
                        next: 'B',
                    }
                ]
            },
            Inst { /* B */
                read: [
                    Action {
                        write: 0,
                        dir: Dir::Right,
                        next: 'C',
                    },
                    Action {
                        write: 1,
                        dir: Dir::Left,
                        next: 'B',
                    }
                ]
            },
            Inst { /* C */
                read: [
                    Action {
                        write: 1,
                        dir: Dir::Right,
                        next: 'D',
                    },
                    Action {
                        write: 0,
                        dir: Dir::Left,
                        next: 'A',
                    }
                ]
            },
            Inst { /* D */
                read: [
                    Action {
                        write: 1,
                        dir: Dir::Left,
                        next: 'E',
                    },
                    Action {
                        write: 1,
                        dir: Dir::Left,
                        next: 'F',
                    }
                ]
            },
            Inst { /* E */
                read: [
                    Action {
                        write: 1,
                        dir: Dir::Left,
                        next: 'A',
                    },
                    Action {
                        write: 0,
                        dir: Dir::Left,
                        next: 'D',
                    }
                ]
            },
            Inst { /* F */
                read: [
                    Action {
                        write: 1,
                        dir: Dir::Right,
                        next: 'A',
                    },
                    Action {
                        write: 1,
                        dir: Dir::Left,
                        next: 'E',
                    }
                ]
            },
        ],
        count: 12586542,
    }
}

/*
Begin in state A.
Perform a diagnostic checksum after 12586542 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 0.
    - Move one slot to the right.
    - Continue with state C.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state B.

In state C:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state D.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state A.

In state D:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state E.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state F.

In state E:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state D.

In state F:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state E.

*/

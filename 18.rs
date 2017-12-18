/* Advent of Code 2017, Day 18 */

/* Jordan Justen : this file is public domain */

use std::collections::LinkedList;
use std::str::FromStr;

#[derive(Clone,Debug)]
enum RegOrImm {
    Reg { r: char },
    Imm { i: i64 },
}

#[derive(Clone,Debug)]
enum Inst {
    Snd { src: char },
    Set { dst: char, src: RegOrImm },
    Add { dst: char, src: RegOrImm },
    Mul { dst: char, src: RegOrImm },
    Mod { dst: char, src: RegOrImm },
    Rcv { dst: char },
    Jgz { cond: RegOrImm, dst: RegOrImm },
}

fn main() {
    let input = read_input1();
    println!("part1: {}", run_program(&input));
    println!("part2: {}", part2(input));
}

fn run_program(insts: &Vec<Inst>) -> i64 {
    let mut regs = [0i64; 26];
    let mut freq = 0;
    let mut ip = 0;
    let mut rcv_count = 0;
    while ip < insts.len() {
        let mut inc_ip = true;
        let inst = &insts[ip];
        match inst {
            &Inst::Snd { src } => freq = regs[src as usize - 'a' as usize],
            &Inst::Set { dst, ref src } => {
                let src_val = get_val(&regs, src);
                regs[dst as usize - 'a' as usize] = src_val;
            },
            &Inst::Add { dst, ref src } => {
                let src_val = get_val(&regs, src);
                regs[dst as usize - 'a' as usize] += src_val;
            },
            &Inst::Mul { dst, ref src } => {
                let src_val = get_val(&regs, src);
                regs[dst as usize - 'a' as usize] *= src_val;
            },
            &Inst::Mod { dst, ref src } => {
                let src_val = get_val(&regs, src);
                regs[dst as usize - 'a' as usize] %= src_val;
            },
            &Inst::Rcv { dst } => {
                let reg_val = regs[dst as usize - 'a' as usize];
                if reg_val != 0 {
                    regs[dst as usize - 'a' as usize] = freq;
                    if rcv_count == 0 {
                        return freq;
                    }
                    rcv_count += 1;
                }
            },
            &Inst::Jgz { ref cond, ref dst } => {
                let cond_val = match cond {
                    &RegOrImm::Reg { r } => regs[r as usize - 'a' as usize],
                    &RegOrImm::Imm { i } => i,
                };
                if cond_val > 0 {
                    let ip_change = get_val(&regs, dst);
                    if ip_change >= 0 { ip += ip_change as usize; }
                    else { ip -= (-ip_change) as usize };
                    inc_ip = false;
                }
            },
        }
        if inc_ip {
            ip += 1;
        }
    }
    0
}

fn part2(insts: Vec<Inst>) -> i64 {
    let (mut regs0, mut regs1) = ([0i64; 26], [0i64; 26]);
    regs1['p' as usize - 'a' as usize] = 1;
    let (mut ip0, mut ip1) = (0, 0);
    let (mut blocked0, mut blocked1) = (false, false);
    let mut q0 = LinkedList::<i64>::new();
    let mut q1 = LinkedList::<i64>::new();
    let mut p1_sends = 0;
    while ip0 < insts.len() && (!blocked0 || !blocked1) {
        blocked0 = run_inst(&insts, &mut regs0, &mut ip0, &mut q0, &mut q1);
        let qlen = q1.len();
        blocked1 = run_inst(&insts, &mut regs1, &mut ip1, &mut q1, &mut q0);
        if q1.len() > qlen {
            p1_sends += 1;
        }
    }
    p1_sends
}

fn run_inst(insts: &Vec<Inst>, regs: &mut [i64], ip: &mut usize,
            sndq: &mut LinkedList<i64>, rcvq: &mut LinkedList<i64>) -> bool {
    let mut blocked = false;
    let mut inc_ip = true;
    let inst = &insts[*ip];
    match inst {
        &Inst::Snd { src } => {
            sndq.push_back(regs[src as usize - 'a' as usize]);
        }
        &Inst::Set { dst, ref src } => {
            let src_val = get_val(&regs, src);
            regs[dst as usize - 'a' as usize] = src_val;
        },
        &Inst::Add { dst, ref src } => {
            let src_val = get_val(&regs, src);
            regs[dst as usize - 'a' as usize] += src_val;
        },
        &Inst::Mul { dst, ref src } => {
            let src_val = get_val(&regs, src);
            regs[dst as usize - 'a' as usize] *= src_val;
        },
        &Inst::Mod { dst, ref src } => {
            let src_val = get_val(&regs, src);
            regs[dst as usize - 'a' as usize] %= src_val;
        },
        &Inst::Rcv { dst } => {
            let val = rcvq.pop_front();
            match val {
                Some(o) => regs[dst as usize - 'a' as usize] = o,
                None => blocked = true,
            }
        },
        &Inst::Jgz { ref cond, ref dst } => {
            let cond_val = match cond {
                &RegOrImm::Reg { r } => regs[r as usize - 'a' as usize],
                &RegOrImm::Imm { i } => i,
            };
            if cond_val > 0 {
                let ip_change = get_val(&regs, dst);
                if ip_change >= 0 { *ip += ip_change as usize; }
                else { *ip -= (-ip_change) as usize };
                inc_ip = false;
            }
        },
    }
    if !blocked && inc_ip {
        *ip += 1;
    }
    blocked
}

fn get_val(regs: &[i64], spec: &RegOrImm) -> i64 {
    match spec {
        &RegOrImm::Reg { r } => regs[r as usize - 'a' as usize],
        &RegOrImm::Imm { i } => i,
    }
}

#[allow(dead_code)]
fn read_input0() -> Vec<Inst> {
    let sample_lines: Vec<String> = "\
        set a 1
        add a 2
        mul a a
        mod a 5
        snd a
        set a 0
        rcv a
        jgz a -1
        set a 1
        jgz a -2".split('\n').map(|s| String::from(s)).collect();
    lines_to_insts(&sample_lines)
}

fn lines_to_insts(ls: &Vec<String>) -> Vec<Inst> {
    ls.iter().map(|s| s.trim()).filter(|l| l.len() > 0).map(|s| {
        let mut strs = s.split_whitespace().collect::<Vec<_>>();
        let rest = strs.split_off(1);
        let fst = rest[0].chars().next().unwrap();
        let snd =
            if rest.len() > 1 { rest[1].chars().next().unwrap() }
            else { '_' };
        let cmd = strs[0];
        match cmd {
            "set"|"add"|"mul"|"mod"|"jgz" => {
                let snd_enc = match snd {
                    'a' ... 'z' => RegOrImm::Reg { r: snd },
                    _ => RegOrImm::Imm { i: i64::from_str(rest[1]).unwrap() },
                };
                let inst = match cmd {
                    "set" => Inst::Set { dst: fst, src: snd_enc },
                    "add" => Inst::Add { dst: fst, src: snd_enc },
                    "mul" => Inst::Mul { dst: fst, src: snd_enc },
                    "mod" => Inst::Mod { dst: fst, src: snd_enc },
                    "jgz" => {
                        let cond = match fst {
                            'a' ... 'z' => RegOrImm::Reg { r: fst },
                            _ => RegOrImm::Imm { i: i64::from_str(rest[0]).
                                                 unwrap() },
                        };
                        Inst::Jgz { cond: cond, dst: snd_enc }
                    },
                    _ => panic!("unknown instruction {}", cmd)
                };
                inst
            },
            "snd" => Inst::Snd { src: fst },
            "rcv" => Inst::Rcv { dst: fst },
            _ => panic!("unknown instruction {}", cmd)
        }
    }).collect()
}

#[allow(dead_code)]
fn read_input1() -> Vec<Inst> {
    let input ="\
set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 735
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19".split('\n').map(|s| String::from(s)).collect();
    lines_to_insts(&input)
}

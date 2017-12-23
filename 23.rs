/* Advent of Code 2017, Day 23 */

/* Jordan Justen : this file is public domain */

use std::str::FromStr;

#[derive(Clone,Copy,Debug,PartialEq)]
enum RegOrImm {
    Reg { r: char },
    Imm { i: i64 },
}

#[derive(Clone,Copy,Debug)]
enum Inst {
    Set { dst: char, src: RegOrImm },
    Sub { dst: char, src: RegOrImm },
    Mul { dst: char, src: RegOrImm },
    Jnz { cond: RegOrImm, dst: RegOrImm },
}

fn main() {
    let input = read_input1();
    println!("part1: {}", run_program(&input));
    println!("part2: {}", optimized_program());
}

fn run_program(insts: &Vec<Inst>) -> i64 {
    let mut regs = [0i64; 26];
    let mut ip = 0;
    let mut mul_count = 0;
    while ip < insts.len() {
        let mut inc_ip = true;
        let inst = &insts[ip];
        match inst {
            &Inst::Set { dst, ref src } => {
                let src_val = get_val(&regs, src);
                regs[dst as usize - 'a' as usize] = src_val;
            },
            &Inst::Sub { dst, ref src } => {
                let src_val = get_val(&regs, src);
                regs[dst as usize - 'a' as usize] -= src_val;
            },
            &Inst::Mul { dst, ref src } => {
                let src_val = get_val(&regs, src);
                regs[dst as usize - 'a' as usize] *= src_val;
                mul_count += 1;
            },
            &Inst::Jnz { ref cond, ref dst } => {
                let cond_val = match cond {
                    &RegOrImm::Reg { r } => regs[r as usize - 'a' as usize],
                    &RegOrImm::Imm { i } => i,
                };
                if cond_val != 0 {
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
    mul_count
}

fn get_val(regs: &[i64], spec: &RegOrImm) -> i64 {
    match spec {
        &RegOrImm::Reg { r } => regs[r as usize - 'a' as usize],
        &RegOrImm::Imm { i } => i,
    }
}

/*

Slightly transformed assembly:

        set b 65
        set c b
        jnz a 2
        jnz 1 5
        mul b 100
        sub b -100000
        set c b
        sub c -17000

loop1:  set f 1
        set d 2
back13: set e 2
back8:  set g d
        mul g e
        sub g b
        jnz g skip1
        set f 0
skip1:  add e 1
        set g e
        sub g b
        jnz g back8
        add d 1
        set g d
        sub g b
        jnz g back13
        jnz f skip2
        add h 1
skip2:  set g b
        sub g c
        jnz g skip3
        jnz 1 done
skip3:  add b 17
        jnz 1 loop1
done:

It appears that the program checks for non-prime numbers in the b
register. Each iteration, it increases b by 17, until b matches c.

*/

fn optimized_program() -> i64 {
    let mut b = 65 * 100 - (-100000);
    let c = b - (-17000);
    let mut h = 0;
    assert!(c > b && (c - b) % 17 == 0);
    loop {
        let mut prime = true;
        for i in 2 .. {
            if i * i > b {
                break;
            }
            if b % i == 0 {
                prime = false;
                break;
            }
        }
        if !prime {
            h += 1;
        }
        if b - c == 0 {
            break;
        }
        b += 17;
    }
    h
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
            "set"|"sub"|"mul"|"jnz" => {
                let snd_enc = match snd {
                    'a' ... 'z' => RegOrImm::Reg { r: snd },
                    _ => RegOrImm::Imm { i: i64::from_str(rest[1]).unwrap() },
                };
                let inst = match cmd {
                    "set" => Inst::Set { dst: fst, src: snd_enc },
                    "sub" => Inst::Sub { dst: fst, src: snd_enc },
                    "mul" => Inst::Mul { dst: fst, src: snd_enc },
                    "jnz" => {
                        let cond = match fst {
                            'a' ... 'z' => RegOrImm::Reg { r: fst },
                            _ => RegOrImm::Imm { i: i64::from_str(rest[0]).
                                                 unwrap() },
                        };
                        Inst::Jnz { cond: cond, dst: snd_enc }
                    },
                    _ => panic!("unknown instruction {}", cmd)
                };
                inst
            },
            _ => panic!("unknown instruction {}", cmd)
        }
    }).collect()
}

#[allow(dead_code)]
fn read_input1() -> Vec<Inst> {
    let input ="\
set b 65
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23".split('\n').map(|s| String::from(s)).collect();
    lines_to_insts(&input)
}

/* Advent of Code 2017, Day 3 */

/* Jordan Justen : this file is public domain */

fn main() {
    part1();
    part2();
}

fn part1() {
    assert_eq!(moves(1), 0);
    assert_eq!(moves(12), 3);
    assert_eq!(moves(23), 2);
    assert_eq!(moves(1024), 31);
    println!("part1: {}", moves(361527));
}

fn moves(n: u32) -> u32 {
    if n <= 1 {
        return 0;
    }
    let l = layer(n);
    let side = 2 * l + 1;
    let start = (side - 2).pow(2) + 1;
    let end = start + 4 * (side - 1);
    let circ = n - start;
    let side_pos = circ % ((end - start) / 4);
    let side_dist = side_pos as i32 - (side as i32 / 2) + 1;
    side / 2 + side_dist.abs() as u32
}

fn layer(n: u32) -> u32 {
    for l in 0.. {
        let side: u32 = 2 * l + 1;
        if side.pow(2) >= n {
            return l;
        }
    }
    0
}

fn part2() {
    let input = 361527;

    let mut memo = Vec::<u32>::new();
    test_memo_pos();
    let (mut x, mut y) = (0i32, 0i32);
    let mut dir = 0;
    let mut side_2 = 0;

    loop {
        let memo_idx = memo_pos(x, y);
        let near: [u32;8] = [ memo_pos(x + 1, y), memo_pos(x + 1, y + 1),
                              memo_pos(x, y + 1), memo_pos(x - 1, y + 1),
                              memo_pos(x - 1, y), memo_pos(x - 1, y - 1),
                              memo_pos(x, y - 1), memo_pos(x + 1, y - 1) ];

        let mut val = 0;
        if x == 0 && y == 0 {
            val = 1;
        } else {
            for near_pos in &near {
                if *near_pos < memo_idx {
                    val += memo[*near_pos as usize];
                }
            }
        }
        if val > input {
            println!("part2: {}", val);
            return;
        }
        memo.push(val);

        match dir {
            0 => {
                x += 1;
                side_2 += 1;
                dir += 1;
            },
            1 => {
                y += 1;
                if y == side_2 {
                    dir += 1;
                }
            },
            2 => {
                x -= 1;
                if x == -side_2 {
                    dir += 1;
                }
            },
            3 => {
                y -= 1;
                if y == -side_2 {
                    dir += 1;
                }
            },
            4 => {
                x += 1;
                if x == side_2 {
                    dir = 0;
                }
            },
            _ => ()
        }
    }
}

fn test_memo_pos() {
    assert_eq!(memo_pos(0, 0), 0);
    assert_eq!(memo_pos(1, 0), 1);
    assert_eq!(memo_pos(1, 1), 2);
    assert_eq!(memo_pos(0, 1), 3);
    assert_eq!(memo_pos(-1, 1), 4);
    assert_eq!(memo_pos(-1, 0), 5);
    assert_eq!(memo_pos(-1, -1), 6);
    assert_eq!(memo_pos(0, -1), 7);
    assert_eq!(memo_pos(1, -1), 8);
    assert_eq!(memo_pos(2, -1), 9);
    assert_eq!(memo_pos(2, 0), 10);
    assert_eq!(memo_pos(2, 1), 11);
    assert_eq!(memo_pos(2, 2), 12);
    assert_eq!(memo_pos(1, 2), 13);
    assert_eq!(memo_pos(0, 2), 14);
    assert_eq!(memo_pos(-1, 2), 15);
    assert_eq!(memo_pos(-2, 2), 16);
    assert_eq!(memo_pos(-2, 1), 17);
    assert_eq!(memo_pos(-2, 0), 18);
    assert_eq!(memo_pos(-2, -1), 19);
    assert_eq!(memo_pos(-2, -2), 20);
    assert_eq!(memo_pos(-1, -2), 21);
    assert_eq!(memo_pos(0, -2), 22);
    assert_eq!(memo_pos(1, -2), 23);
    assert_eq!(memo_pos(2, -2), 24);
    assert_eq!(memo_pos(3, -2), 25);
    assert_eq!(memo_pos(3, 3), 30);
    assert_eq!(memo_pos(-3, 3), 36);
    assert_eq!(memo_pos(-3, -3), 42);
    assert_eq!(memo_pos(3, -3), 48);
}

fn memo_pos(x: i32, y: i32) -> u32 {
    if x == 0 && y == 0 {
        return 0;
    }
    let (x_a, y_a)= (x.abs() as u32, y.abs() as u32);
    let l = if x_a > y_a { x_a } else { y_a };
    let side = 2 * l + 1;
    let side_2 = side / 2;
    let start = (side - 2).pow(2);

    if x >= 0 && y > -(side_2 as i32) {
        return start + (y + side_2 as i32 - 1) as u32 +
            (side as i32 / 2 - x) as u32;
    } else if y > -(side_2 as i32) {
        return start + (side - 1) + (side_2 as i32 - y) as u32 +
            (side_2 as i32 - x - 1) as u32;
    } else {
        return start + 3 * (side - 1) - 1 + (side_2 as i32 + x) as u32;
    }
}

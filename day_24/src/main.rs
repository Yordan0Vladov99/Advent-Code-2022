use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

use gcd::Gcd;

fn part_1() {
    let mut path = File::open("input.txt").unwrap();
    let mut input = String::new();
    path.read_to_string(&mut input).unwrap();
    let lines = input.trim().split('\n').skip(1).collect::<Vec<_>>();

    let r = lines.len() - 1;
    let c = lines[0].len() - 2;
    let mut blizzards = vec![HashSet::<(i32, i32)>::new(); 4];

    for (row, line) in lines.iter().enumerate() {
        for (col, item) in line[1..].chars().enumerate() {
            match item {
                '<' => {
                    blizzards[0].insert((row as i32, col as i32));
                }
                '>' => {
                    blizzards[1].insert((row as i32, col as i32));
                }
                '^' => {
                    blizzards[2].insert((row as i32, col as i32));
                }
                'v' => {
                    blizzards[3].insert((row as i32, col as i32));
                }
                _ => {}
            }
        }
    }

    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    queue.push_back((0, -1, 0));
    let mut seen = HashSet::new();
    let target = (r as i32, (c - 1) as i32);
    let lcm: i32 = (r * c / r.gcd(c)) as i32;

    while !queue.is_empty() {
        let (mut time, cr, cc) = queue.pop_front().unwrap();
        //println!("{} {} {}", time, cr, cc);
        time += 1;

        for (dr, dc) in vec![(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)].iter() {
            let nr = cr + dr;
            let nc = cc + dc;

            if (nr, nc) == target {
                println!("{}", time);
                return;
            }

            if (nr < 0 || nc < 0 || nr >= r as i32 || nc >= c as i32) && (nr, nc) != (-1, 0) {
                continue;
            }

            let mut fail = false;

            if (nr, nc) != (-1, 0) {
                for (i, tr, tc) in vec![(0, 0, -1), (1, 0, 1), (2, -1, 0), (3, 1, 0)].iter() {
                    let t = (
                        (nr - tr * time).rem_euclid(r as i32),
                        (nc - tc * time).rem_euclid(c as i32),
                    );
                    if blizzards.get(*i).unwrap().contains(&t) {
                        fail = true;
                        break;
                    }
                }
                if !fail {
                    let key = (nr, nc, time.rem_euclid(lcm));
                    if seen.contains(&key) {
                        continue;
                    }

                    seen.insert(key);
                    queue.push_back((time, nr, nc));
                }
            }
        }
    }
}

fn part_2() {
    let mut path = File::open("input.txt").unwrap();
    let mut input = String::new();
    path.read_to_string(&mut input).unwrap();
    let lines = input.trim().split('\n').skip(1).collect::<Vec<_>>();

    let r = lines.len() - 1;
    let c = lines[0].len() - 2;
    let mut blizzards = vec![HashSet::<(i32, i32)>::new(); 4];

    for (row, line) in lines.iter().enumerate() {
        for (col, item) in line[1..].chars().enumerate() {
            match item {
                '<' => {
                    blizzards[0].insert((row as i32, col as i32));
                }
                '>' => {
                    blizzards[1].insert((row as i32, col as i32));
                }
                '^' => {
                    blizzards[2].insert((row as i32, col as i32));
                }
                'v' => {
                    blizzards[3].insert((row as i32, col as i32));
                }
                _ => {}
            }
        }
    }

    let mut queue: VecDeque<(i32, i32, i32, i32)> = VecDeque::new();
    queue.push_back((0, -1, 0, 0));
    let mut seen = HashSet::new();
    let targets = vec![(r as i32, (c - 1) as i32), (-1, 0)];
    let lcm: i32 = (r * c / r.gcd(c)) as i32;

    while !queue.is_empty() {
        let (mut time, cr, cc, stage) = queue.pop_front().unwrap();
        //println!("{} {} {} {}", time, cr, cc, stage);
        time += 1;

        for (dr, dc) in vec![(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)].iter() {
            let nr = cr + dr;
            let nc = cc + dc;

            let mut nstage = stage;

            if (nr, nc) == *targets.get((stage.rem_euclid(2)) as usize).unwrap() {
                if stage == 2 {
                    println!("{}", time);
                    return;
                }
                nstage += 1;
            }

            if (nr < 0 || nc < 0 || nr >= r as i32 || nc >= c as i32)
                && !targets.contains(&(nr, nc))
            {
                continue;
            }

            let mut fail = false;

            if !targets.contains(&(nr, nc)) {
                for (i, tr, tc) in vec![(0, 0, -1), (1, 0, 1), (2, -1, 0), (3, 1, 0)].iter() {
                    let t = (
                        (nr - tr * time).rem_euclid(r as i32),
                        (nc - tc * time).rem_euclid(c as i32),
                    );
                    if blizzards.get(*i).unwrap().contains(&t) {
                        fail = true;
                        break;
                    }
                }
            }
            if !fail {
                let key = (nr, nc, nstage, time.rem_euclid(lcm));
                if seen.contains(&key) {
                    continue;
                }

                seen.insert(key);
                queue.push_back((time, nr, nc, nstage));
            }
        }
    }
}

fn main() {
    part_2();
}

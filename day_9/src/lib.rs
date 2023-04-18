use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

pub fn close_distance(head: (i32, i32), tail: &mut (i32, i32)) {
    if tail.0 == head.0 {
        if head.1 - tail.1 < -1 {
            tail.1 -= 1;
        } else if head.1 - tail.1 > 1 {
            tail.1 += 1;
        }
    } else if tail.1 == head.1 {
        if head.0 - tail.0 < -1 {
            tail.0 -= 1
        } else if head.0 - tail.0 > 1 {
            tail.0 += 1;
        }
    } else if head.0 - tail.0 < -1 {
        tail.0 -= 1;
        if head.1 < tail.1 {
            tail.1 -= 1;
        } else {
            tail.1 += 1;
        }
    } else if head.0 - tail.0 > 1 {
        tail.0 += 1;
        if head.1 < tail.1 {
            tail.1 -= 1;
        } else {
            tail.1 += 1;
        }
    } else if head.1 - tail.1 < -1 {
        tail.1 -= 1;
        if head.0 < tail.0 {
            tail.0 -= 1;
        } else {
            tail.0 += 1;
        }
    } else if head.1 - tail.1 > 1 {
        tail.1 += 1;
        if head.0 < tail.0 {
            tail.0 -= 1;
        } else {
            tail.0 += 1;
        }
    }
}

pub fn part_1() {
    let (mut head, mut tail) = ((0, 0), (0, 0));
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tail);
    let directions = get_input("input.txt".to_string());
    for line in directions.iter() {
        let args = line.split(" ").collect::<Vec<&str>>();
        let iters = args[1].parse::<usize>().unwrap();
        let direction = match args[0] {
            "D" => (1, 0),
            "R" => (0, 1),
            "L" => (0, -1),
            "U" => (-1, 0),
            _ => (0, 0),
        };

        for _ in 0..iters {
            head = (head.0 + direction.0, head.1 + direction.1);
            close_distance(head, &mut tail);
            visited.insert(tail);
        }
    }
    println!("{}", visited.len());
}

pub fn part_2() {
    let mut body = [(0, 0); 10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(body[9]);
    let directions = get_input("input.txt".to_string());
    for line in directions.iter() {
        let args = line.split(" ").collect::<Vec<&str>>();
        let iters = args[1].parse::<usize>().unwrap();
        let direction = match args[0] {
            "D" => (1, 0),
            "R" => (0, 1),
            "L" => (0, -1),
            "U" => (-1, 0),
            _ => (0, 0),
        };

        for _ in 0..iters {
            body[0] = (body[0].0 + direction.0, body[0].1 + direction.1);
            for i in 0..9 {
                close_distance(body[i], &mut body[i + 1]);
            }
            visited.insert(body[9]);
        }
    }
    println!("{}", visited.len());
}

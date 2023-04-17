use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

pub fn part_1() {
    let input = get_input("input.txt".to_string());
    let mut cols: Vec<Vec<char>> = vec![Vec::new(); 9];
    for line in input[..8].iter() {
        for i in (0..line.len()).step_by(4) {
            if line.as_bytes()[i + 1] != ' ' as u8 {
                cols[i / 4].push(line.as_bytes()[i + 1] as char);
            }
        }
    }

    cols.iter_mut().for_each(|x| x.reverse());

    for line in input[10..].iter() {
        let args: Vec<&str> = line.split(" ").collect();
        let iterations: usize = args[1].parse().unwrap();
        let source: usize = args[3].parse().unwrap();
        let dest: usize = args[5].parse().unwrap();
        for _ in 0..iterations {
            let val = cols[source - 1].pop().unwrap();
            cols[dest - 1].push(val);
        }
    }

    for i in 0..9 {
        print!("{}", cols[i].pop().unwrap())
    }
    println!()
}

pub fn part_2() {
    let input = get_input("input.txt".to_string());
    let mut cols: Vec<Vec<char>> = vec![Vec::new(); 9];
    for line in input[..8].iter() {
        for i in (0..line.len()).step_by(4) {
            if line.as_bytes()[i + 1] != ' ' as u8 {
                cols[i / 4].push(line.as_bytes()[i + 1] as char);
            }
        }
    }

    cols.iter_mut().for_each(|x| x.reverse());

    for line in input[10..].iter() {
        let args: Vec<&str> = line.split(" ").collect();
        let iterations: usize = args[1].parse().unwrap();
        let source: usize = args[3].parse().unwrap();
        let dest: usize = args[5].parse().unwrap();
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..iterations {
            let val = cols[source - 1].pop().unwrap();
            temp.push(val);
        }
        for _ in 0..iterations {
            let val = temp.pop().unwrap();
            cols[dest - 1].push(val);
        }
    }

    for i in 0..9 {
        print!("{}", cols[i].pop().unwrap())
    }
    println!()
}

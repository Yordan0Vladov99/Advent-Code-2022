use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

fn all_unique(code: &VecDeque<char>, size: usize) -> bool {
    let set: HashSet<char> = HashSet::from_iter(code.iter().cloned());
    set.len() == size
}
pub fn part_1() {
    let input = &get_input("input.txt".to_string())[0];
    let mut code: VecDeque<char> = input[0..4].chars().collect();

    if all_unique(&code, 4) {
        println!("4");
        return;
    }
    code.pop_front();
    for (i, ch) in input[4..].chars().into_iter().enumerate() {
        code.push_back(ch);
        if all_unique(&code, 4) {
            println!("{}", i + 5);
            return;
        }
        code.pop_front();
    }
}

pub fn part_2() {
    let input = &get_input("input.txt".to_string())[0];
    let mut code: VecDeque<char> = input[0..14].chars().collect();

    if all_unique(&code, 14) {
        println!("14");
        return;
    }
    code.pop_front();
    for (i, ch) in input[14..].chars().into_iter().enumerate() {
        code.push_back(ch);
        if all_unique(&code, 14) {
            println!("{}", i + 15);
            return;
        }
        code.pop_front();
    }
}

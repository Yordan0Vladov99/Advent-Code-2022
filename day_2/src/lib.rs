use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

pub fn part_2() {
    let pairs = get_input("input.txt".to_string());
    let mut score = 0;
    for pair in pairs.iter() {
        let hands: Vec<char> = pair.split(" ").map(|x| x.chars().nth(0).unwrap()).collect();
        let first = hands[0] as i32 - 'A' as i32;
        let result = hands[1] as i32 - 'X' as i32;
        //first
        // 0 - Rock
        // 1 - Paper
        // 2 - Scissors

        //result
        //0 - loss
        //1 - draw
        //2 - win
        let second = match result {
            0 => (first - 1).rem_euclid(3),
            1 => first,
            2 => (first + 1).rem_euclid(3),
            _ => 0,
        };
        score += second + 1 + result * 3;
    }
    println!("{}", score)
}

pub fn part_1() {
    let pairs = get_input("input.txt".to_string());
    let mut score = 0;
    for pair in pairs.iter() {
        let hands: Vec<char> = pair.split(" ").map(|x| x.chars().nth(0).unwrap()).collect();
        let first = hands[0] as i32 - 'A' as i32;
        let second = hands[1] as i32 - 'X' as i32;
        // 0 - Rock
        // 1 - Paper
        // 2 - Scissors
        let result = second
            + 1
            + match (second - first).rem_euclid(3) {
                0 => 3,
                1 => 6,
                _ => 0,
            };
        score += result;
    }
    println!("{}", score)
}

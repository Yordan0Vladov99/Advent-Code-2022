use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}
fn str_to_unsigned_array(pair: &str) -> Vec<u32> {
    pair.split("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse().expect("Invalid unsigned int."))
        .collect()
}
pub fn part_1() {
    let mut overlaps: u32 = 0;
    let input = get_input("input.txt".to_string());
    for line in input.iter() {
        let pair: Vec<&str> = line.split(",").collect();
        let first: Vec<u32> = str_to_unsigned_array(pair[0]);
        let second: Vec<u32> = str_to_unsigned_array(pair[1]);
        if (first[0] >= second[0] && first[1] <= second[1])
            || (second[0] >= first[0] && second[1] <= first[1])
        {
            overlaps += 1;
        }
    }
    println!("{}", overlaps);
}

pub fn part_2() {
    let mut overlaps: u32 = 0;
    let input = get_input("input.txt".to_string());
    for line in input.iter() {
        let pair: Vec<&str> = line.split(",").collect();
        let first: Vec<u32> = str_to_unsigned_array(pair[0]);
        let second: Vec<u32> = str_to_unsigned_array(pair[1]);
        if (first[0] >= second[0] && first[1] <= second[1])
            || (second[0] >= first[0] && second[1] <= first[1])
            || (first[0] < second[0] && first[1] >= second[0])
            || (second[0] < first[0] && second[1] >= first[0])
        {
            overlaps += 1;
        }
    }
    println!("{}", overlaps);
}

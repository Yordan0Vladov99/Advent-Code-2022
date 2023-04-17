use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

fn update_top_calories(top_calories: &mut [i32], index: usize, entry: i32) {
    for i in (index + 1..top_calories.len()).rev() {
        top_calories[i] = top_calories[i - 1];
    }
    top_calories[index] = entry;
}

pub fn part_1() {
    let calories = get_input("input.txt".to_string());
    let mut max_calories = 0;
    let mut cur_calories = 0;

    for calorie in calories.iter() {
        if calorie.len() == 0 {
            if cur_calories > max_calories {
                max_calories = cur_calories
            }
            cur_calories = 0;
        } else {
            cur_calories += calorie.parse::<i32>().expect("Invalid i32.");
        }
    }
    println!("{}", max_calories)
}

pub fn part_2() {
    let calories = get_input("input.txt".to_string());
    let mut max_calories = [0; 3];
    let mut cur_calories = 0;

    for calorie in calories.iter() {
        if calorie.len() == 0 {
            for i in 0..max_calories.len() {
                if cur_calories > max_calories[i] {
                    update_top_calories(&mut max_calories, i, cur_calories);
                    break;
                }
            }
            cur_calories = 0;
        } else {
            cur_calories += calorie.parse::<i32>().expect("Invalid i32.");
        }
    }
    println!("{}", max_calories.iter().sum::<i32>())
}

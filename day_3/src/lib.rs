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
pub fn part_2() {
    let mut item_sum: u32 = 0;
    let items = get_input("input.txt".to_string());
    for i in (0..items.len()).step_by(3) {
        let mut sets: Vec<HashSet<char>> = vec![HashSet::new(); 2];
        for j in 0..2 {
            items[i + j].chars().for_each(|x| {
                sets[j].insert(x);
            });
        }
        let common = *items[i + 2]
            .chars()
            .filter(|x| sets[0].contains(x) && sets[1].contains(x))
            .collect::<Vec<char>>()
            .get(0)
            .unwrap();
        item_sum += 1 + match &common {
            'a'..='z' => common as u32 - 'a' as u32,
            'A'..='Z' => common as u32 - 'A' as u32 + 26,
            _ => 0,
        }
    }
    println!("{}", item_sum);
}
pub fn part_1() {
    let mut item_sum: u32 = 0;
    let items = get_input("input.txt".to_string());
    for item in items.iter() {
        let mut items_set: HashSet<char> = HashSet::new();
        item[..item.len() / 2].chars().for_each(|x| {
            items_set.insert(x);
        });
        let common = *item[item.len() / 2..]
            .chars()
            .filter(|x| items_set.contains(x))
            .collect::<Vec<char>>()
            .get(0)
            .unwrap();
        item_sum += 1 + match &common {
            'a'..='z' => common as u32 - 'a' as u32,
            'A'..='Z' => common as u32 - 'A' as u32 + 26,
            _ => 0,
        }
    }
    println!("{}", item_sum);
}

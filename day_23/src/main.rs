use day_23::{Direction, DirectionGroup, Elf};
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use strum::IntoEnumIterator;
fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

fn is_alone(elf: &Elf, positions: &HashMap<(i32, i32), i32>) -> bool {
    for direction in Direction::iter() {
        let pos = (
            elf.position.0 + direction.value().0,
            elf.position.1 + direction.value().1,
        );
        if positions.contains_key(&pos) {
            return false;
        }
    }
    return true;
}

fn add_position(elf: &Elf, positions: &mut HashMap<(i32, i32), Vec<i32>>) {
    if !positions.contains_key(&elf.position) {
        positions.insert(elf.position, Vec::new());
    }
    positions.get_mut(&elf.position).unwrap().push(elf.index);
}

fn print_free_space(elves: &Vec<Elf>) {
    let (mut min_x, mut min_y) = elves.get(0).unwrap().position;
    let (mut max_x, mut max_y) = elves.get(0).unwrap().position;

    for elf in elves {
        let (x, y) = elf.position;
        max_x = cmp::max(max_x, x);
        max_y = cmp::max(max_y, y);

        min_x = cmp::min(min_x, x);
        min_y = cmp::min(min_y, y);
    }

    let spaces = (max_x - min_x + 1).abs() * (max_y - min_y + 1).abs();
    let free_spaces = spaces - elves.len() as i32;
    /*
    println!(
        "min_x: {}; max_x: {}, min_y: {}, max_y: {}",
        min_x, max_x, min_y, max_y
    );
    */
    println!("{}", free_spaces);
}

fn part_1() {
    let grid: Vec<String> = get_input("input.txt".to_string());

    let mut elves_count = 0;
    let mut elves: Vec<Elf> = Vec::new();
    for row in grid.iter().cloned().enumerate() {
        for col in row.1.chars().enumerate() {
            if col.1 == '#' {
                elves_count += 1;
                elves.push(Elf {
                    index: elves_count,
                    position: (row.0 as i32, col.0 as i32),
                })
            }
        }
    }

    let mut active_direction_group: DirectionGroup = DirectionGroup::N;
    for _ in 0..10 {
        let mut default_positions: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut old_positions: HashMap<(i32, i32), i32> = HashMap::new();
        let mut potential_positions: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

        for elf in &elves {
            default_positions.insert(elf.index, elf.position);
            old_positions.insert(elf.position, elf.index);
        }

        for elf in &elves {
            let mut dg = active_direction_group.clone();
            let (x, y) = elf.position;
            let mut can_move = false;
            let is_alone = is_alone(&elf, &old_positions);

            if is_alone {
                add_position(&elf, &mut potential_positions);
                continue;
            }

            for _ in 0..4 {
                let mut has_neighbour = false;
                let directions = dg.group();
                for direction in &directions {
                    let (dir_x, dir_y) = direction.value();
                    let (new_x, new_y) = (x + dir_x, y + dir_y);
                    if old_positions.contains_key(&(new_x, new_y)) {
                        has_neighbour = true;
                        break;
                    }
                }
                if !has_neighbour {
                    can_move = true;
                    let (dir_x, dir_y) = dg.direction().value();
                    let new_pos = (x + dir_x, y + dir_y);
                    if !potential_positions.contains_key(&new_pos) {
                        potential_positions.insert(new_pos, Vec::new());
                    }
                    let index = elf.index;
                    potential_positions.get_mut(&new_pos).unwrap().push(index);
                    break;
                }

                dg = dg.turn();
            }
            if !can_move {
                add_position(&elf, &mut potential_positions);
            }
        }

        let mut new_positions: Vec<Elf> = Vec::new();
        for position in potential_positions.keys() {
            if potential_positions.get(position).unwrap().len() == 1 {
                let id = potential_positions.get(position).unwrap().get(0).unwrap();
                new_positions.push(Elf {
                    index: *id,
                    position: *position,
                })
            } else {
                for elf in potential_positions.get(position).unwrap().iter() {
                    let elf_position = default_positions.get(elf).unwrap();
                    new_positions.push(Elf {
                        index: *elf,
                        position: *elf_position,
                    })
                }
            }
        }
        elves = new_positions;
        active_direction_group = active_direction_group.turn();
    }

    print_free_space(&elves)
}

fn part_2() {
    let grid: Vec<String> = get_input("input.txt".to_string());

    let mut elves_count = 0;
    let mut elves: Vec<Elf> = Vec::new();
    for row in grid.iter().cloned().enumerate() {
        for col in row.1.chars().enumerate() {
            if col.1 == '#' {
                elves_count += 1;
                elves.push(Elf {
                    index: elves_count,
                    position: (row.0 as i32, col.0 as i32),
                })
            }
        }
    }

    let mut active_direction_group: DirectionGroup = DirectionGroup::N;
    let mut has_movement = true;
    let mut round = 0;
    while has_movement {
        round += 1;
        has_movement = false;
        let mut default_positions: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut old_positions: HashMap<(i32, i32), i32> = HashMap::new();
        let mut potential_positions: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

        for elf in &elves {
            default_positions.insert(elf.index, elf.position);
            old_positions.insert(elf.position, elf.index);
        }

        for elf in &elves {
            let mut dg = active_direction_group.clone();
            let (x, y) = elf.position;
            let mut can_move = false;
            let is_alone = is_alone(&elf, &old_positions);
            has_movement |= !is_alone;
            if is_alone {
                add_position(&elf, &mut potential_positions);
                continue;
            }

            for _ in 0..4 {
                let mut has_neighbour = false;
                let directions = dg.group();
                for direction in &directions {
                    let (dir_x, dir_y) = direction.value();
                    let (new_x, new_y) = (x + dir_x, y + dir_y);
                    if old_positions.contains_key(&(new_x, new_y)) {
                        has_neighbour = true;
                        break;
                    }
                }
                if !has_neighbour {
                    can_move = true;
                    let (dir_x, dir_y) = dg.direction().value();
                    let new_pos = (x + dir_x, y + dir_y);
                    if !potential_positions.contains_key(&new_pos) {
                        potential_positions.insert(new_pos, Vec::new());
                    }
                    let index = elf.index;
                    potential_positions.get_mut(&new_pos).unwrap().push(index);
                    break;
                }

                dg = dg.turn();
            }
            if !can_move {
                add_position(&elf, &mut potential_positions);
            }
        }

        let mut new_positions: Vec<Elf> = Vec::new();
        for position in potential_positions.keys() {
            if potential_positions.get(position).unwrap().len() == 1 {
                let id = potential_positions.get(position).unwrap().get(0).unwrap();
                new_positions.push(Elf {
                    index: *id,
                    position: *position,
                })
            } else {
                for elf in potential_positions.get(position).unwrap().iter() {
                    let elf_position = default_positions.get(elf).unwrap();
                    new_positions.push(Elf {
                        index: *elf,
                        position: *elf_position,
                    })
                }
            }
        }
        elves = new_positions;
        active_direction_group = active_direction_group.turn();
    }

    print!("{}", round);
}
fn main() {
    part_2();
}

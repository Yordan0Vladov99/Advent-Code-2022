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
    let input = get_input("input.txt".to_string());
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in input.iter() {
        grid.push(line.chars().map(|x| x as u32 - '0' as u32).collect());
    }
    let height = grid.len();
    let width = grid[0].len();

    let mut max_visibility = 0;

    for row in 1..height {
        for col in 1..width {
            let mut left = 0;
            for l in (0..col).rev() {
                left += 1;
                if grid[row][l] >= grid[row][col] {
                    break;
                }
            }

            let mut right = 0;
            for r in col + 1..width {
                right += 1;
                if grid[row][r] >= grid[row][col] {
                    break;
                }
            }

            let mut up = 0;

            for u in (0..row).rev() {
                up += 1;
                if grid[u][col] >= grid[row][col] {
                    break;
                }
            }

            let mut down = 0;

            for d in row + 1..height {
                down += 1;
                if grid[d][col] >= grid[row][col] {
                    break;
                }
            }

            let visibility = up * down * left * right;
            if visibility > max_visibility {
                max_visibility = visibility;
            }
        }
    }
    println!("{}", max_visibility);
}

pub fn part_1() {
    let input = get_input("input.txt".to_string());
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in input.iter() {
        grid.push(line.chars().map(|x| x as u32 - '0' as u32).collect());
    }
    let height = grid.len();
    let width = grid[0].len();

    let mut visible: Vec<Vec<bool>> = Vec::new();
    visible.push(vec![true; width]);
    let mut row = vec![false; width];
    row[0] = true;
    row[width - 1] = true;
    for _ in 0..height - 2 {
        visible.push(row.clone());
    }
    visible.push(vec![true; width]);

    for row in 1..height - 1 {
        let mut max = grid[row][0];
        for col in 1..width - 1 {
            if grid[row][col] > max {
                visible[row][col] = true;
                max = grid[row][col];
            }
        }
    }

    for row in 1..height - 1 {
        let mut max = grid[row][width - 1];
        for col in (1..width - 1).rev() {
            if grid[row][col] > max {
                visible[row][col] = true;
                max = grid[row][col];
            }
        }
    }

    for col in 1..width - 1 {
        let mut max = grid[0][col];
        for row in 1..height - 1 {
            if grid[row][col] > max {
                visible[row][col] = true;
                max = grid[row][col];
            }
        }
    }

    for col in 1..width - 1 {
        let mut max = grid[height - 1][col];
        for row in (1..height - 1).rev() {
            if grid[row][col] > max {
                visible[row][col] = true;
                max = grid[row][col];
            }
        }
    }

    let mut visible_counter = 0;
    for row in 0..height {
        for col in 0..width {
            if visible[row][col] {
                visible_counter += 1;
            }
        }
    }

    println!("{}", visible_counter)
}

mod file;

use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use file::SystemFile;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

fn get_system_size(root: Rc<RefCell<SystemFile>>, size: &mut u32, directory_size: &mut u32) {
    let mut child_directory_size: u32 = 0;
    for child in root.borrow().children.iter() {
        get_system_size(Rc::clone(child), size, &mut child_directory_size);
    }
    child_directory_size += root.borrow().size;
    *directory_size += child_directory_size;
    if root.borrow().size == 0 && child_directory_size < 100_000 {
        *size += child_directory_size;
    }
}

fn get_smallest_deletable_directory(
    root: Rc<RefCell<SystemFile>>,
    size: &mut u32,
    directory_size: &mut u32,
    needed_size: u32,
) {
    let mut child_directory_size: u32 = 0;
    for child in root.borrow().children.iter() {
        get_smallest_deletable_directory(
            Rc::clone(child),
            size,
            &mut child_directory_size,
            needed_size,
        );
    }
    child_directory_size += root.borrow().size;
    *directory_size += child_directory_size;
    if root.borrow().size == 0
        && child_directory_size < *size
        && child_directory_size >= needed_size
    {
        *size = child_directory_size;
    }
}

fn get_total_system_size(root: Rc<RefCell<SystemFile>>, size: &mut u32) {
    *size += root.borrow().size;
    for child in root.borrow().children.iter() {
        get_total_system_size(Rc::clone(child), size);
    }
}

pub fn part_1() {
    let input = get_input("input.txt".to_string());
    let mut i = 1;
    let mut root_directory = Rc::new(RefCell::new(SystemFile {
        name: "/".to_string(),
        size: 0,
        children: Vec::new(),
        parent: Option::None,
    }));
    let mut current_directory = Rc::clone(&root_directory);

    while i < input.len() {
        let args: Vec<&str> = input[i].split(" ").collect();
        if args[1] == "ls" {
            i += 1;
            while i < input.len() {
                let file: Vec<&str> = input[i].split(" ").collect();
                if file[0] == "$" {
                    break;
                } else if file[0] == "dir" {
                    let mut dir = Rc::new(RefCell::new(SystemFile {
                        name: file[1].to_string(),
                        size: 0,
                        children: Vec::new(),
                        parent: Some(Rc::clone(&current_directory)),
                    }));

                    current_directory
                        .borrow_mut()
                        .children
                        .push(Rc::clone(&dir));
                } else {
                    let mut dir = Rc::new(RefCell::new(SystemFile {
                        name: file[1].to_string(),
                        size: file[0].parse::<u32>().unwrap(),
                        children: Vec::new(),
                        parent: Some(Rc::clone(&current_directory)),
                    }));

                    current_directory
                        .borrow_mut()
                        .children
                        .push(Rc::clone(&dir));
                }
                i += 1;
            }
        } else if args[1] == "cd" {
            current_directory = match args[2] {
                ".." => Rc::clone(&current_directory.borrow_mut().parent.as_mut().unwrap()),
                _ => {
                    let mut res = Rc::clone(&current_directory);
                    for child in current_directory.borrow().children.iter() {
                        if child.borrow().name == args[2] {
                            res = Rc::clone(child);
                        }
                    }
                    res
                }
            };
            i += 1;
        }
    }
    let mut size: u32 = 0;
    let mut directory_size: u32 = 0;
    get_system_size(root_directory, &mut size, &mut directory_size);
    println!("{}", size);
}

pub fn part_2() {
    let input = get_input("input.txt".to_string());
    let mut i = 1;
    let mut root_directory = Rc::new(RefCell::new(SystemFile {
        name: "/".to_string(),
        size: 0,
        children: Vec::new(),
        parent: Option::None,
    }));
    let mut current_directory = Rc::clone(&root_directory);

    while i < input.len() {
        let args: Vec<&str> = input[i].split(" ").collect();
        if args[1] == "ls" {
            i += 1;
            while i < input.len() {
                let file: Vec<&str> = input[i].split(" ").collect();
                if file[0] == "$" {
                    break;
                } else if file[0] == "dir" {
                    let mut dir = Rc::new(RefCell::new(SystemFile {
                        name: file[1].to_string(),
                        size: 0,
                        children: Vec::new(),
                        parent: Some(Rc::clone(&current_directory)),
                    }));

                    current_directory
                        .borrow_mut()
                        .children
                        .push(Rc::clone(&dir));
                } else {
                    let mut dir = Rc::new(RefCell::new(SystemFile {
                        name: file[1].to_string(),
                        size: file[0].parse::<u32>().unwrap(),
                        children: Vec::new(),
                        parent: Some(Rc::clone(&current_directory)),
                    }));

                    current_directory
                        .borrow_mut()
                        .children
                        .push(Rc::clone(&dir));
                }
                i += 1;
            }
        } else if args[1] == "cd" {
            current_directory = match args[2] {
                ".." => Rc::clone(&current_directory.borrow_mut().parent.as_mut().unwrap()),
                _ => {
                    let mut res = Rc::clone(&current_directory);
                    for child in current_directory.borrow().children.iter() {
                        if child.borrow().name == args[2] {
                            res = Rc::clone(child);
                        }
                    }
                    res
                }
            };
            i += 1;
        }
    }
    let mut size: u32 = 0;
    get_total_system_size(Rc::clone(&root_directory), &mut size);

    let mut directory_size: u32 = 0;
    let needed_space: i32 = (30_000_000 as i32 - 70_000_000 + size as i32)
        .try_into()
        .unwrap();

    get_smallest_deletable_directory(
        root_directory,
        &mut size,
        &mut directory_size,
        needed_space as u32,
    );
    println!("{}", size);
}

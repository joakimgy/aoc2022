#![allow(dead_code)]
use crate::utils;
use std::collections::HashMap;
struct Directory {
    name: String,
    path: String,
    subdirs: Vec<Directory>,
    files: Vec<File>,
    parent: String,
}

impl Directory {
    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }
}

struct File {
    name: String,
    size: usize,
}

pub fn task1() -> usize {
    let input = utils::read_file("src/input.txt");
    let rows: Vec<&str> = input.split('\n').collect();
    let mut directories: HashMap<String, Directory> = HashMap::new();
    directories.insert(
        "/".to_string(),
        Directory {
            name: "Root".to_string(),
            path: "".to_string(),
            subdirs: Vec::new(),
            files: Vec::new(),
            parent: "".to_string(),
        },
    );

    let mut path: String = "".to_string();
    for command in rows {
        if command.contains("$ ls") {
            println!("{command}");
            continue;
        } else if command.contains("$ cd") {
            path = cd(&path, command);
            println!("New path: {path}");
        } else {
            add_to_directory(&path, command, &mut directories)
        }
    }

    return 0;
}

fn add_to_directory(path: &String, command: &str, directories: &mut HashMap<String, Directory>) {
    if command.contains("dir") {
        println!("Dir: {command}");
    } else {
        println!("File: {command}");
        if directories.contains_key(path) {
            println!("Directory exists");
        } else {
            let cmd_list: Vec<&str> = command.split(" ").collect();
            let file = File {
                name: cmd_list[1].to_string(),
                size: cmd_list[0].parse::<usize>().expect("Size does not exist"),
            };
            let current_directory = directories.get(path).expect("Directory should exist");
            current_directory.add_file(file);

            directories.insert(
                path.to_string(),
                Directory {
                    name: "Root".to_string(),
                    path: "".to_string(),
                    subdirs: Vec::new(),
                    files: Vec::new(),
                    parent: "".to_string(),
                },
            );
        }
    }
}

fn cd(old_path: &String, command: &str) -> String {
    let cd_arg = command
        .split(" ")
        .last()
        .expect("Command should not be empty");
    if cd_arg == "/" {
        return "".to_string();
    } else if cd_arg == ".." {
        let mut path_directories: Vec<&str> = old_path.split("/").collect();
        path_directories.pop();
        let new_path = path_directories.join("/");
        return new_path;
    } else {
        return old_path.to_string() + "/" + cd_arg;
    }
}

pub fn task2() -> usize {
    return 0;
}

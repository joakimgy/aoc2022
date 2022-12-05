#![allow(dead_code)]
use crate::utils;
use regex::Regex;
use std::collections::HashMap;

pub fn task1() -> i32 {
    let input = utils::read_file("src/day5.txt");
    let rows: Vec<&str> = utils::split_on_newline(input.as_str());
    let crates: Vec<&str> = rows[0].split('\n').collect();
    let instructions: Vec<&str> = rows[1].split('\n').collect();

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();

    let stack_indexes = crates_to_stack_indexes(&crates);
    for index in stack_indexes {
        let empty_list: Vec<char> = Vec::new();
        stacks.insert(index, empty_list);
    }

    print_stacks(&stacks);

    let re = Regex::new(r"\[(.*?)\]").unwrap();

    for (i, c) in crates.iter().enumerate().rev() {
        let length = crates.len();
        if length - 1 == i {
            continue;
        }
        println!("\nCrate: {c}");
        let chars: Vec<char> = c.chars().collect();
        let columns = chars.chunks(4);
        for (j, coc) in columns.enumerate() {
            let parsed: String = coc.iter().collect();
            let index = j + 1;
            println!("col {index}: {parsed}");
            match re.find(&parsed) {
                Some(res) => {
                    let r: Vec<char> = res.as_str().chars().collect();
                    let letter = r[1];
                    let item = item_to_add_to_stack(&stacks, &index, letter);
                    stacks.insert(index, item);
                    println!("Match: {letter}");
                }
                None => {
                    println!("No match");
                }
            };
        }
    }
    print_top_of_stacks(&stacks);
    print_instructions(instructions);

    return 0;
}

fn item_to_add_to_stack(stacks: &HashMap<usize, Vec<char>>, key: &usize, value: char) -> Vec<char> {
    let current_value = stacks.get(key).expect("Stack key should not be empty");
    let mut copy_value = current_value.to_owned();
    copy_value.push(value);
    return copy_value;
}

fn print_stacks(stacks: &HashMap<usize, Vec<char>>) {
    for (i, v) in stacks {
        let length = v.len();
        println!("Key: {i} with length {length}");
    }
}

fn print_top_of_stacks(stacks: &HashMap<usize, Vec<char>>) {
    for (i, v) in stacks {
        let length = v.len();
        match v.last() {
            Some(res) => {
                println!("Key: {i} with length {length} ({res})");
            }
            None => {
                println!("Key: {i} with length {length}");
            }
        }
    }
}

fn print_instructions(instructions: Vec<&str>) {
    println!("");
    for i in instructions {
        println!("Instruction {i}");
    }
}

fn crates_to_stack_indexes(crates: &Vec<&str>) -> Vec<usize> {
    let stack_indexes = crates.last().expect("List should not be empty");
    let stack_index_chars: Vec<char> = stack_indexes.chars().filter(|&c| c != ' ').collect();
    return stack_index_chars
        .iter()
        .map(|i| utils::char_to_usize(&i))
        .collect();
}

pub fn task2() -> i32 {
    return 0;
}

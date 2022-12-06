#![allow(dead_code)]
use crate::utils;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn task1() -> String {
    let input = utils::read_file("src/day5.txt");
    let rows: Vec<&str> = utils::split_on_newline(input.as_str());
    let crates: Vec<&str> = rows[0].split('\n').collect();
    let instructions: Vec<&str> = rows[1].split('\n').collect();

    let mut stacks = init_stack_from_crates(&crates);
    populate_stack_with_crates(&mut stacks, &crates);

    //print_top_of_stacks(&stacks);

    let parsed_instructions = parse_instructions(&instructions);
    execute_instructions(&parsed_instructions, &mut stacks);

    let top_of_stack: String = get_top_of_stacks(&stacks).iter().collect();
    return top_of_stack;
}

pub fn task2() -> String {
    let input = utils::read_file("src/day5.txt");
    let rows: Vec<&str> = utils::split_on_newline(input.as_str());
    let crates: Vec<&str> = rows[0].split('\n').collect();
    let instructions: Vec<&str> = rows[1].split('\n').collect();

    let mut stacks = init_stack_from_crates(&crates);
    populate_stack_with_crates(&mut stacks, &crates);

    //print_top_of_stacks(&stacks);

    let parsed_instructions = parse_instructions(&instructions);
    execute_instructions_task2(&parsed_instructions, &mut stacks);

    let top_of_stack: String = get_top_of_stacks(&stacks).iter().collect();
    return top_of_stack;
}

struct Instruction {
    from: usize,
    to: usize,
    quantity: usize,
}

fn execute_instructions(instructions: &Vec<Instruction>, stacks: &mut HashMap<usize, Vec<char>>) {
    for instruction in instructions {
        move_n_crates(
            instruction.quantity,
            instruction.from,
            instruction.to,
            stacks,
        );
    }
}

fn execute_instructions_task2(
    instructions: &Vec<Instruction>,
    stacks: &mut HashMap<usize, Vec<char>>,
) {
    for instruction in instructions {
        move_n_crates_at_once(
            instruction.quantity,
            instruction.from,
            instruction.to,
            stacks,
        );
    }
}

fn parse_instructions(instructions: &Vec<&str>) -> Vec<Instruction> {
    return instructions
        .iter()
        .filter(|&&i| i != "")
        .map(|&i| {
            let l: Vec<&str> = i.split(" ").collect();
            let quantity = l
                .get(1)
                .expect("Quantity not found")
                .parse::<usize>()
                .unwrap();

            let from = l
                .get(3)
                .expect("Quantity not found")
                .parse::<usize>()
                .unwrap();

            let to = l
                .get(5)
                .expect("Quantity not found")
                .parse::<usize>()
                .unwrap();

            Instruction {
                from: from,
                to: to,
                quantity: quantity,
            }
        })
        .collect();
}

fn move_n_crates_at_once(n: usize, from: usize, to: usize, stacks: &mut HashMap<usize, Vec<char>>) {
    let mut from_stack = stacks.get(&from).expect("Stack does not exist").to_owned();
    let mut crates_to_move = from_stack.split_off(from_stack.len() - n);
    stacks.insert(from, from_stack);
    let mut to_stack = stacks.get(&to).expect("Stack does not exist").to_owned();
    to_stack.append(&mut crates_to_move);
    stacks.insert(to, to_stack);
}

fn move_n_crates(n: usize, from: usize, to: usize, stacks: &mut HashMap<usize, Vec<char>>) {
    for _ in 0..n {
        move_crate(from, to, stacks);
    }
}

fn move_crate(from: usize, to: usize, stacks: &mut HashMap<usize, Vec<char>>) {
    let mut from_stack = stacks.get(&from).expect("Stack does not exist").to_owned();
    let crate_to_move = from_stack.pop().expect("Crate does not exist");
    stacks.insert(from, from_stack);
    let mut to_stack = stacks.get(&to).expect("Stack does not exist").to_owned();
    to_stack.push(crate_to_move);
    stacks.insert(to, to_stack);
}

fn populate_stack_with_crates(stacks: &mut HashMap<usize, Vec<char>>, crates: &Vec<&str>) {
    let re = Regex::new(r"\[(.*?)\]").unwrap();

    for (i, c) in crates.iter().enumerate().rev() {
        let length = crates.len();
        if length - 1 == i {
            continue;
        }
        let chars: Vec<char> = c.chars().collect();
        let columns = chars.chunks(4);
        for (j, coc) in columns.enumerate() {
            let parsed: String = coc.iter().collect();
            let index = j + 1;
            match re.find(&parsed) {
                Some(res) => {
                    let r: Vec<char> = res.as_str().chars().collect();
                    let letter = r[1];
                    let item = item_to_add_to_stack(&stacks, &index, letter);
                    stacks.insert(index, item);
                }
                None => {}
            };
        }
    }
}

fn init_stack_from_crates(crates: &Vec<&str>) -> HashMap<usize, Vec<char>> {
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();

    let stack_indexes = stack_indexes_from_crates(crates);
    for index in stack_indexes {
        let empty_list: Vec<char> = Vec::new();
        stacks.insert(index, empty_list);
    }

    return stacks;
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

fn get_top_of_stacks(stacks: &HashMap<usize, Vec<char>>) -> Vec<char> {
    let keys = stacks.keys().sorted();
    let mut top_of_stacks: Vec<char> = Vec::new();
    for key in keys {
        let v = stacks.get(key).expect("Key does not exist");
        match v.last() {
            Some(res) => {
                top_of_stacks.push(*res);
            }
            None => {}
        }
    }
    return top_of_stacks;
}

fn print_top_of_stacks(stacks: &HashMap<usize, Vec<char>>) {
    let top = get_top_of_stacks(stacks);
    println!("Top of stack:");
    for c in top {
        print!("{c}");
    }
    println!("");
}

fn print_raw_instructions(instructions: &Vec<&str>) {
    println!("");
    for i in instructions {
        println!("Instruction {i}");
    }
}

fn print_instructions(instructions: &Vec<Instruction>) {
    println!("");
    for i in instructions {
        let quantity = i.quantity;
        let from = i.from;
        let to = i.to;
        println!("Instruction {quantity} from {from} to {to}");
    }
}

fn stack_indexes_from_crates(crates: &Vec<&str>) -> Vec<usize> {
    let stack_indexes = crates.last().expect("List should not be empty");
    let stack_index_chars: Vec<char> = stack_indexes.chars().filter(|&c| c != ' ').collect();
    return stack_index_chars
        .iter()
        .map(|i| utils::char_to_usize(&i))
        .collect();
}

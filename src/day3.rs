#![allow(dead_code)]
use std::ops::Div;

use crate::utils;

pub fn task1() -> usize {
    let input = utils::read_file("src/day3.txt");
    let rows: Vec<&str> = input.split('\n').collect();
    return rows.iter().map(|&row| priority_of_compartment(row)).sum();
}

fn priority_of_compartment(text: &str) -> usize {
    let no_items_per_compartment = text.len().div(2);
    let (compartment1, compartment2) = text.split_at(no_items_per_compartment);
    let char = compartment1
        .chars()
        .find(|&c| compartment2.chars().any(|o| c == o))
        .expect("Character not found");
    let priority = priority_of_character(char);
    //println!("Char {char} with priority {priority}");
    return priority;
}

pub fn task2() -> usize {
    let input = utils::read_file("src/day3.txt");
    let rows: Vec<&str> = input.split('\n').collect();
    let groups = rows.chunks(3);
    return groups
        .map(|group| {
            let char = character_of_group(group.to_vec());
            priority_of_character(char.to_owned())
        })
        .sum();
}

fn character_of_group(group: Vec<&str>) -> &char {
    let res = ITEMS
        .iter()
        .find(|&item| {
            group
                .to_owned()
                .into_iter()
                .all(|g| g.chars().any(|i| item.to_owned() == i))
        })
        .expect("Cannot find character of group");
    return res;
}

const ITEMS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn priority_of_character(char: char) -> usize {
    return ITEMS
        .into_iter()
        .position(|c| c == char)
        .expect("Can't fnd prio of char")
        + 1;
}

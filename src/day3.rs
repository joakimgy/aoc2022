#![allow(dead_code)]
use std::ops::Div;

use crate::utils;

pub fn task1() -> i32 {
    let input = utils::read_file("src/day3.txt");
    let rows: Vec<&str> = input.split('\n').collect();
    return rows.iter().map(|&row| priority_of_compartment(row)).sum();
}

fn priority_of_compartment(text: &str) -> i32 {
    let no_items_per_compartment = text.len().div(2);
    let (compartment1, compartment2) = text.split_at(no_items_per_compartment);
    let maybe_char = compartment1
        .chars()
        .find(|&c| compartment2.chars().any(|o| c == o));
    let char = match maybe_char {
        Some(c) => c,
        None => {
            println!("Cannot find character");
            return -1;
        }
    };
    let priority = priority_of_character(char);
    //println!("Char {char} with priority {priority}");
    return priority;
}

fn priority_of_character(char: char) -> i32 {
    return match char {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 1 + 26,
        'B' => 2 + 26,
        'C' => 3 + 26,
        'D' => 4 + 26,
        'E' => 5 + 26,
        'F' => 6 + 26,
        'G' => 7 + 26,
        'H' => 8 + 26,
        'I' => 9 + 26,
        'J' => 10 + 26,
        'K' => 11 + 26,
        'L' => 12 + 26,
        'M' => 13 + 26,
        'N' => 14 + 26,
        'O' => 15 + 26,
        'P' => 16 + 26,
        'Q' => 17 + 26,
        'R' => 18 + 26,
        'S' => 19 + 26,
        'T' => 20 + 26,
        'U' => 21 + 26,
        'V' => 22 + 26,
        'W' => 23 + 26,
        'X' => 24 + 26,
        'Y' => 25 + 26,
        'Z' => 26 + 26,
        _ => {
            println!("Invalid character");
            return -1;
        }
    };
}

pub fn task2() -> i32 {
    return 0;
}

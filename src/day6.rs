#![allow(dead_code)]
use crate::utils;
use std::collections::HashSet;
use std::hash::Hash;

pub fn task1() -> usize {
    let input: Vec<char> = utils::read_file("src/day6.txt").chars().collect();
    for i in 0..input.len() {
        match input.get(i..(i + 4)) {
            Some(chars) => {
                if has_unique_elements(chars.iter()) {
                    return i + 4;
                }
            }
            None => {
                println!("Could not get slice")
            }
        };
    }
    return 0;
}

pub fn task2() -> usize {
    let input: Vec<char> = utils::read_file("src/day6.txt").chars().collect();
    for i in 0..input.len() {
        match input.get(i..(i + 14)) {
            Some(chars) => {
                if has_unique_elements(chars.iter()) {
                    return i + 14;
                }
            }
            None => {
                println!("Could not get slice")
            }
        };
    }
    return 0;
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

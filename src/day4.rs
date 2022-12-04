#![allow(dead_code)]
use crate::utils;

pub fn task1() -> i32 {
    let input = utils::read_file("src/day4.txt");
    let pairs = input.split('\n');

    let mut count = 0;
    for pair in pairs {
        let elf_pair: Vec<&str> = pair.split(',').collect();
        let elf1 = elf_pair[0];
        let elf2 = elf_pair[1];
        let elf1_range = numbers_from_id_range(elf1);
        let elf2_range = numbers_from_id_range(elf2);

        let fully_contained = full_contained(elf1_range, elf2_range);
        if fully_contained == true {
            count = count + 1;
        }
    }

    return count;
}

pub fn numbers_from_id_range(range: &str) -> Vec<i32> {
    let range_list: Vec<&str> = range.split('-').collect();
    let lower_bound = range_list[0]
        .parse::<i32>()
        .expect("Could not parse lower bound");
    let higher_bound = range_list[1]
        .parse::<i32>()
        .expect("Could not parse higher bound");
    return (lower_bound..(higher_bound + 1)).collect::<Vec<i32>>();
}

pub fn full_contained(range1: Vec<i32>, range2: Vec<i32>) -> bool {
    let overlap1 = range1.iter().all(|f| range2.contains(f));
    let overlap2 = range2.iter().all(|f| range1.contains(f));
    return overlap1 || overlap2;
}

pub fn task2() -> i32 {
    let input = utils::read_file("src/day4.txt");
    let pairs = input.split('\n');

    let mut count = 0;
    for pair in pairs {
        let elf_pair: Vec<&str> = pair.split(',').collect();
        let elf1 = elf_pair[0];
        let elf2 = elf_pair[1];
        let elf1_range = numbers_from_id_range(elf1);
        let elf2_range = numbers_from_id_range(elf2);

        let overlapping = overlaps(elf1_range, elf2_range);
        if overlapping == true {
            count = count + 1;
        }
    }

    return count;
}

pub fn overlaps(range1: Vec<i32>, range2: Vec<i32>) -> bool {
    let overlap1 = range1.iter().any(|f| range2.contains(f));
    let overlap2 = range2.iter().any(|f| range1.contains(f));
    return overlap1 || overlap2;
}

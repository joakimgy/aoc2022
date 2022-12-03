use crate::utils;

pub fn day1_task1() -> i32 {
    let input = utils::read_file("src/day1.txt");
    let elves: Vec<&str> = utils::split_on_newline(input.as_str());

    let sum = elves
        .iter()
        .map(|&elf| sum_of_elf(elf))
        .max()
        .expect("List is empty");
    return sum;
}

fn sum_of_elf(elf: &str) -> i32 {
    return elf
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().expect("Cannot parse to int"))
        .sum();
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
pub mod utils;

fn main() {
    day(6);
}

fn day(day: i32) {
    match day {
        1 => {
            let day1_1 = day1::day1_task1();
            println!("Day 1_1 answer: {day1_1}");
            let day1_2 = day1::day1_task2();
            println!("Day 1_2 answer: {day1_2}");
        }
        2 => {
            let day2_1 = day2::task1();
            println!("Day 2_1 answer: {day2_1}");
            let day2_2 = day2::task2();
            println!("Day 2_2 answer: {day2_2}");
        }
        3 => {
            let day3_1 = day3::task1();
            println!("Day 3_1 answer: {day3_1}");
            let day3_2 = day3::task2();
            println!("Day 3_2 answer: {day3_2}");
        }
        4 => {
            let day4_1 = day4::task1();
            println!("Day 4_1 answer: {day4_1}");
            let day4_2 = day4::task2();
            println!("Day 4_2 answer: {day4_2}");
        }
        5 => {
            let day5_1 = day5::task1();
            println!("Day 5_1 answer: {day5_1}");
            let day5_2 = day5::task2();
            println!("Day 5_2 answer: {day5_2}");
        }
        6 => {
            let day6_1 = day6::task1();
            println!("Day 6_1 answer: {day6_1}");
            let day6_2 = day6::task2();
            println!("Day 6_2 answer: {day6_2}");
        }
        _ => return,
    }
}

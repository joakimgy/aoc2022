#![allow(dead_code)]
use crate::utils;

const ROCK: &str = "X";
const PAPER: &str = "Y";
const SCISSORS: &str = "Z";
const OPP_ROCK: &str = "A";
const OPP_PAPER: &str = "B";
const OPP_SCISSORS: &str = "C";

pub fn task1() -> i32 {
    let input = utils::read_file("src/day2.txt");
    let strategy_guide: Vec<&str> = input.split('\n').collect();
    return strategy_guide.iter().map(|&row| points_of_round(row)).sum();
}

fn points_of_round(round: &str) -> i32 {
    let round_shapes: Vec<&str> = round.split(' ').collect();
    let opponent_shape = round_shapes[0];
    let your_shape = round_shapes[1];
    let points_from_shape = points_from_move(your_shape);
    let points_from_round = points_from_round(your_shape, opponent_shape);
    let points = points_from_shape + points_from_round;
    //println!("{opponent_shape} - {your_shape} ({points })");
    return points;
}

fn points_from_move(shape: &str) -> i32 {
    return match shape {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
        _ => {
            println!("Invalid shape");
            0
        }
    };
}

fn points_from_round(your_shape: &str, opponent_shape: &str) -> i32 {
    if your_shape == ROCK && opponent_shape == OPP_ROCK {
        return 3;
    } else if your_shape == ROCK && opponent_shape == OPP_PAPER {
        return 0;
    } else if your_shape == ROCK && opponent_shape == OPP_SCISSORS {
        return 6;
    } else if your_shape == SCISSORS && opponent_shape == OPP_SCISSORS {
        return 3;
    } else if your_shape == SCISSORS && opponent_shape == OPP_ROCK {
        return 0;
    } else if your_shape == SCISSORS && opponent_shape == OPP_PAPER {
        return 6;
    } else if your_shape == PAPER && opponent_shape == OPP_PAPER {
        return 3;
    } else if your_shape == PAPER && opponent_shape == OPP_ROCK {
        return 6;
    } else if your_shape == PAPER && opponent_shape == OPP_SCISSORS {
        return 0;
    } else {
        println!("Case not covered ({your_shape} - {opponent_shape})");
        return 0;
    }
}

pub fn task2() -> i32 {
    let input = utils::read_file("src/day2.txt");
    let strategy_guide: Vec<&str> = input.split('\n').collect();
    return strategy_guide
        .iter()
        .map(|&row| play_round_and_calc_points(row))
        .sum();
}

fn play_round_and_calc_points(round: &str) -> i32 {
    let round_shapes: Vec<&str> = round.split(' ').collect();
    let opponent_shape = round_shapes[0];
    let strategy_shape = round_shapes[1];
    let your_shape = find_your_move(strategy_shape, opponent_shape);
    let points = points_from_round(&your_shape, opponent_shape) + points_from_move(&your_shape);
    //println!("{opponent_shape} - {your_shape} ({points }) - strategy {strategy_shape}");
    return points;
}

const LOSE: &str = "X";
const DRAW: &str = "Y";
const WIN: &str = "Z";
fn find_your_move(strategy_shape: &str, opponent_shape: &str) -> String {
    return match strategy_shape {
        LOSE => match opponent_shape {
            OPP_PAPER => ROCK,
            OPP_ROCK => SCISSORS,
            OPP_SCISSORS => PAPER,
            _ => {
                println!("Invalid opponent shape");
                "-"
            }
        },
        DRAW => match opponent_shape {
            OPP_PAPER => PAPER,
            OPP_ROCK => ROCK,
            OPP_SCISSORS => SCISSORS,
            _ => {
                println!("Invalid opponent shape");
                "-"
            }
        },
        WIN => match opponent_shape {
            OPP_PAPER => SCISSORS,
            OPP_ROCK => PAPER,
            OPP_SCISSORS => ROCK,
            _ => {
                println!("Invalid opponent shape");
                "-"
            }
        },
        _ => {
            println!("Invalid shape");
            "-"
        }
    }
    .to_owned();
}

/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */

use std::env;
use std::fs;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

use std::fmt::Display;
use std::time::Duration;
use std::time::Instant;

fn time_solution<T>(func: impl FnOnce(&str) -> Option<T>, input: &str) -> Option<(T, Duration)> {
    let timer = Instant::now();
    let result = func(input);
    let elapsed = timer.elapsed();

    result.map(|result| (result, elapsed))
}

pub fn print_result<T: Display>(func: impl FnOnce(&str) -> Option<T>, input: &str, part: u8) {
    match time_solution(func, input) {
        Some((result, elapsed)) => {
            println!(
                "{}Part {}{}: {} {}(elapsed: {:.2?}){}",
                ANSI_BOLD, part, ANSI_RESET, result, ANSI_ITALIC, elapsed, ANSI_RESET
            );
        }
        None => {
            println!("{}Part {}{}: not solved.", ANSI_BOLD, part, ANSI_RESET)
        }
    }
}

#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        fn main() {
            let input = aoc::template::read_file("inputs", $day);
            aoc::template::print_result(part_one, &input, 1);
            aoc::template::print_result(part_two, &input, 2);
        }
    };
}

#[must_use]
pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{day:02}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file").trim().to_string()
}

/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 * Prefer `./helpers.rs` if you want to extract code from your solutions.
 */
use std::cmp;
use std::env;
use std::fs;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

pub mod helpers;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("src").join(folder).join(format!("{:02}.txt", day));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

fn average_duration(numbers: &[Duration]) -> u128 {
    numbers.iter().map(|d| d.as_nanos()).sum::<u128>() / numbers.len() as u128
}

fn format_duration(duration: &Duration, iterations: u64) -> String {
    format!("(avg. time: {:.1?} @ {} samples)", duration, iterations)
}

pub fn bench<I: Clone, T>(func: impl Fn(I) -> T, input: I, base_time: &Duration) -> String {
    let mut stdout = stdout();

    print!("> {}benchmarking...{}", ANSI_ITALIC, ANSI_RESET);
    let _ = stdout.flush();

    let bench_iterations = cmp::min(
        100000,
        cmp::max(
            Duration::from_secs(2).as_nanos() / cmp::max(base_time.as_nanos(), 10),
            10,
        ),
    );

    let mut timers: Vec<Duration> = vec![];

    for _ in 0..bench_iterations {
        let cloned = input.clone();
        let timer = Instant::now();
        func(cloned);
        timers.push(timer.elapsed());
    }

    print!("\r");

    let avg_time = Duration::from_nanos(average_duration(&timers) as u64);
    format_duration(&avg_time, bench_iterations as u64)
}

#[macro_export]
macro_rules! parse {
    ($parser:ident, $input:expr) => {{
        use advent_of_code::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
        use std::time::Instant;

        let timer = Instant::now();
        let result = $parser($input);
        let base_time = timer.elapsed();

        if $input != "" {
            print!("parser: ");
            let time = advent_of_code::bench($parser, $input, &base_time);
            println!("parser: ✓ {}", time);
        }

        result
    }};
}

#[macro_export]
macro_rules! solve {
    ($part:expr, $solver:ident, $input:expr) => {{
        use advent_of_code::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
        use std::fmt::Display;
        use std::time::Instant;

        let timer = Instant::now();
        let result = $solver($input);
        let base_time = timer.elapsed();

        match result {
            Some(result) => {
                print!("part {}: {}{}{} ", $part, ANSI_BOLD, result, ANSI_RESET);
                println!(
                    "part {}: {}{}{} {}",
                    $part,
                    ANSI_BOLD,
                    result,
                    ANSI_RESET,
                    advent_of_code::bench($solver, $input, &base_time)
                );
            }
            None => {
                print!("not solved.\n")
            }
        }
    }};
}

#[macro_export]
macro_rules! main {
    ($day:expr) => {
        fn main() {
            let input = advent_of_code::read_file("inputs", $day);
            let parsed = advent_of_code::parse!(parse, &input);
            advent_of_code::solve!(1, part_one, parsed.clone());
            advent_of_code::solve!(2, part_two, parsed.clone());
        }
    };
}

use std::cmp;
/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 * Prefer `./helpers.rs` if you want to extract code from your solutions.
 */
use std::env;
use std::fs;
use std::time::{Duration, Instant};

pub mod helpers;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

fn average_duration(numbers: &[Duration]) -> u128 {
    numbers.iter().map(|d| d.as_nanos()).sum::<u128>() / numbers.len() as u128
}

pub fn run_timed<I: Copy, T>(func: impl Fn(I) -> T, input: I) -> (T, Duration, u64) {
    let timer = Instant::now();
    let result = func(input);
    let elapsed = timer.elapsed();

    let mut timers: Vec<Duration> = vec![];

    let bench_iterations = cmp::max(Duration::from_secs(2).as_nanos() / elapsed.as_nanos(), 5);

    for _ in 0..bench_iterations {
        let timer = Instant::now();
        func(input);
        let elapsed = timer.elapsed();
        timers.push(elapsed);
    }

    (
        result,
        Duration::from_nanos(average_duration(&timers) as u64),
        bench_iterations as u64,
    )
}

pub fn format_duration(duration: &Duration, iterations: u64) -> String {
    format!(
        "{}(avg. time: {:.2?} / {} samples){}",
        ANSI_ITALIC, duration, iterations, ANSI_RESET
    )
}

#[macro_export]
macro_rules! parse {
    ($parser:ident, $input:expr) => {{
        use advent_of_code::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
        let (result, elapsed, iterations) = advent_of_code::run_timed($parser, $input);
        println!("🎄 {}Parser{} 🎄", ANSI_BOLD, ANSI_RESET);
        println!("{}", advent_of_code::format_duration(&elapsed, iterations));
        result
    }};
}

#[macro_export]
macro_rules! solve {
    ($part:expr, $solver:ident, $input:expr) => {{
        use advent_of_code::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
        use std::fmt::Display;
        use std::time::Instant;

        fn print_result<I: Copy, T: Display>(func: impl Fn(I) -> Option<T>, input: I) {
            let (result, elapsed, iterations) = advent_of_code::run_timed(func, input);
            match result {
                Some(result) => {
                    println!(
                        "{} {}",
                        result,
                        advent_of_code::format_duration(&elapsed, iterations),
                    );
                }
                None => {
                    println!("not solved.")
                }
            }
        }

        println!("🎄 {}Part {}{} 🎄", ANSI_BOLD, $part, ANSI_RESET);
        print_result($solver, $input);
    }};
}

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd.join("src").join(folder).join(format!("{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

fn parse_time(val: &str, postfix: &str) -> f64 {
    val.split(postfix).next().unwrap().parse().unwrap()
}

pub fn parse_exec_time(output: &str) -> f64 {
    output.lines().fold(0_f64, |acc, l| {
        if !l.contains("avg. time:") {
            acc
        } else {
            let timing = l.split("(avg. time: ").last().unwrap();
            // use `contains` istd. of `ends_with`: string may contain ANSI escape sequences.
            // for possible time formats, see: https://github.com/rust-lang/rust/blob/1.64.0/library/core/src/time.rs#L1176-L1200
            if timing.contains("ns /") {
                acc // range below rounding precision.
            } else if timing.contains("µs /") {
                acc + parse_time(timing, "µs /") / 1000_f64
            } else if timing.contains("ms /") {
                acc + parse_time(timing, "ms /")
            } else if timing.contains("s /") {
                acc + parse_time(timing, "s /") * 1000_f64
            } else {
                acc
            }
        }
    })
}

/// copied from: https://github.com/rust-lang/rust/blob/1.64.0/library/std/src/macros.rs#L328-L333
#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < 1.0e-6,
            "{} is not approximately equal to {}",
            *a,
            *b
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_exec_time() {
        assert_approx_eq!(
            parse_exec_time(&format!(
                "🎄 Part 1 🎄\n0 (avg. time: 74.13ns / 10000 samples){}\n🎄 Part 2 🎄\n0 (avg. time: 50ns / 10000 samples){}",
                ANSI_RESET, ANSI_RESET
            )),
            0_f64
        );

        assert_approx_eq!(
            parse_exec_time("🎄 Part 1 🎄\n0 (avg. time: 755µs / 10000 samples)\n🎄 Part 2 🎄\n0 (avg. time: 700µs / 9000 samples)"),
            1.455_f64
        );

        assert_approx_eq!(
            parse_exec_time("🎄 Part 1 🎄\n0 (avg. time: 70µs / 100 samples)\n🎄 Part 2 🎄\n0 (avg. time: 1.45ms / 10 samples)"),
            1.52_f64
        );

        assert_approx_eq!(
            parse_exec_time(
                "🎄 Part 1 🎄\n0 (avg. time: 10.3s / 1 samples)\n🎄 Part 2 🎄\n0 (avg. time: 100.50ms / 0 samples)"
            ),
            10400.50_f64
        );
    }
}

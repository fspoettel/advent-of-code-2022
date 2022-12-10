/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 * Prefer `./helpers.rs` if you want to extract code from your solutions.
 */
use std::env;
use std::fs;

pub mod helpers;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string.
pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("src").join(folder).join(format!("{:02}.txt", day));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// The parse macro wraps a parsing function with formatted console output.
/// When run with --release, parser code will be benchmarked.
#[macro_export]
macro_rules! parse {
    ($parser:ident, $input:expr) => {{
        use advent_of_code::{runner, ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
        use std::time::Instant;

        let (result, duration) = runner::run_timed($parser, $input, |_| {
            print!("parser: ");
        });

        print!("\r");
        println!("parser: ✓ {}", duration);
        result
    }};
}

/// solve! wraps a solution part with formatted console output.
/// When run with `--release`, parser code will be benchmarked.
#[macro_export]
macro_rules! solve {
    ($day:expr, $part:expr, $solver:ident, $input:expr) => {{
        use advent_of_code::{aoc_cli, runner, ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};

        let (result, duration) = runner::run_timed($solver, $input, |result| {
            if let Some(result) = result {
                if result.to_string().contains("\n") {
                    print!("part {}: ", $part);
                } else {
                    print!("part {}: {}{}{} ", $part, ANSI_BOLD, result, ANSI_RESET);
                }
            } else {
                print!("part {}: ✖", $part);
            }
        });

        match result {
            Some(result) => {
                print!("\r");
                if result.to_string().contains("\n") {
                    println!("part {}: {}", $part, duration);
                    println!("{}{}{}", ANSI_BOLD, result, ANSI_RESET);
                } else {
                    println!(
                        "part {}: {}{}{} {}",
                        $part, ANSI_BOLD, result, ANSI_RESET, duration
                    );
                }
                aoc_cli::submit_result(result, $day, $part);
            }
            None => {
                print!("\r");
                println!("part {}: not solved.", $part);
            }
        }
    }};
}

/// main! produced a block setting up the input, parse! and solve! for each part.
#[macro_export]
macro_rules! main {
    ($day:expr) => {
        fn main() {
            let input = advent_of_code::read_file("inputs", $day);
            let parsed = advent_of_code::parse!(parse, &input);
            advent_of_code::solve!($day, 1, part_one, parsed.clone());
            advent_of_code::solve!($day, 2, part_two, parsed.clone());
        }
    };
}

/// Encapsulates code that interacts with solution functions.
pub mod runner {
    use super::{ANSI_ITALIC, ANSI_RESET};
    use std::cmp;
    use std::io::{stdout, Write};
    use std::time::{Duration, Instant};

    /// Run a solution part. The behavior differs depending on whether
    /// we are running a release or debug build:
    ///  1. in debug, the function is executed once.
    ///  2. in release, the function is benched (approx. 1 second of execution time or 10 runs, whatever take longer.)
    pub fn run_timed<I: Clone, T>(
        func: impl Fn(I) -> T,
        input: I,
        hook: impl Fn(&T),
    ) -> (T, String) {
        let timer = Instant::now();
        let result = func(input.clone());
        let base_time = timer.elapsed();

        hook(&result);

        let duration = match cfg!(debug_assertions) {
            true => format_duration(&base_time, 1),
            false => bench(func, input, &base_time),
        };

        (result, duration)
    }

    fn average_duration(numbers: &[Duration]) -> u128 {
        numbers.iter().map(|d| d.as_nanos()).sum::<u128>() / numbers.len() as u128
    }

    fn format_duration(duration: &Duration, iterations: u64) -> String {
        format!("(avg. time: {:.1?} @ {} samples)", duration, iterations)
    }

    fn bench<I: Clone, T>(func: impl Fn(I) -> T, input: I, base_time: &Duration) -> String {
        let mut stdout = stdout();

        print!("> {}benching{}", ANSI_ITALIC, ANSI_RESET);
        let _ = stdout.flush();

        let bench_iterations = cmp::min(
            100000,
            cmp::max(
                Duration::from_secs(1).as_nanos() / cmp::max(base_time.as_nanos(), 10),
                10,
            ),
        );

        let mut timers: Vec<Duration> = vec![];

        for _ in 0..bench_iterations {
            // need a clone here to make the borrow checker happy.
            let cloned = input.clone();
            let timer = Instant::now();
            func(cloned);
            timers.push(timer.elapsed());
        }

        let avg_time = Duration::from_nanos(average_duration(&timers) as u64);
        format_duration(&avg_time, bench_iterations as u64)
    }
}

/// Wrapper module around the "aoc-cli" CLI.
pub mod aoc_cli {
    use std::{
        env,
        fmt::Display,
        process::{self, Command, Output, Stdio},
    };

    #[derive(Debug)]
    pub enum AocCliError {
        CommandNotFound,
        CommandNotCallable,
        BadExitStatus(Output),
        IoError,
    }

    impl Display for AocCliError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AocCliError::CommandNotFound => write!(f, "aoc-cli is not present in environment."),
                AocCliError::CommandNotCallable => write!(f, "aoc-cli could not be called."),
                AocCliError::BadExitStatus(_) => {
                    write!(f, "aoc-cli exited with a non-zero status.")
                }
                AocCliError::IoError => write!(f, "could not write output files to file system."),
            }
        }
    }

    pub fn check() -> Result<(), AocCliError> {
        Command::new("aoc")
            .arg("-V")
            .output()
            .map_err(|_| AocCliError::CommandNotFound)?;
        Ok(())
    }

    pub fn read(day: u8) -> Result<Output, AocCliError> {
        let puzzle_path = get_puzzle_path(day);

        let args = build_args(
            "read",
            &[
                "--description-only".into(),
                "--puzzle-file".into(),
                puzzle_path,
            ],
            day,
        );

        call_aoc_cli(&args)
    }

    pub fn download(day: u8) -> Result<Output, AocCliError> {
        let input_path = get_input_path(day);
        let puzzle_path = get_puzzle_path(day);

        let args = build_args(
            "download",
            &[
                "--overwrite".into(),
                "--input-file".into(),
                input_path.to_string(),
                "--puzzle-file".into(),
                puzzle_path.to_string(),
            ],
            day,
        );

        let output = call_aoc_cli(&args)?;

        if output.status.success() {
            println!("---");
            println!("🎄 Successfully wrote input to \"{}\".", &input_path);
            println!("🎄 Successfully wrote puzzle to \"{}\".", &puzzle_path);
            Ok(output)
        } else {
            Err(AocCliError::BadExitStatus(output))
        }
    }

    /// Parse the arguments passed to `solve` and try to submit one part of the solution if:
    ///  1. we are in `--release` mode.
    ///  2. aoc-cli is installed.
    pub fn submit_result<T: Display>(
        result: T,
        day: u8,
        part: u8,
    ) -> Option<Result<Output, AocCliError>> {
        let args: Vec<String> = env::args().collect();
        if !args.contains(&"--submit".into()) {
            return None;
        }

        if args.len() < 3 {
            eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
            process::exit(1);
        }

        let part_index = args.iter().position(|x| x == "--submit").unwrap() + 1;
        let part_submit = match args[part_index].parse::<u8>() {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
                process::exit(1);
            }
        };

        if part_submit != part {
            return None;
        }

        if cfg!(debug_assertions) {
            eprintln!("--submit has no effect in debug mode.");
            return None;
        }

        if check().is_err() {
            eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
            process::exit(1);
        }

        // workaround: the argument order is inverted for submit.
        let mut args = build_args("submit", &[], day);
        args.push(part.to_string());
        args.push(result.to_string());

        Some(call_aoc_cli(&args))
    }

    fn get_input_path(day: u8) -> String {
        let day_padded = format!("{:02}", day);
        format!("src/inputs/{}.txt", day_padded)
    }

    fn get_puzzle_path(day: u8) -> String {
        let day_padded = format!("{:02}", day);
        format!("src/puzzles/{}.md", day_padded)
    }

    fn get_year() -> Option<u16> {
        match std::env::var("AOC_YEAR") {
            Ok(x) => x.parse().ok().or(None),
            Err(_) => None,
        }
    }

    fn build_args(command: &str, args: &[String], day: u8) -> Vec<String> {
        let mut cmd_args = args.to_vec();

        if let Some(year) = get_year() {
            cmd_args.push("--year".into());
            cmd_args.push(year.to_string());
        }

        cmd_args.append(&mut vec!["--day".into(), day.to_string(), command.into()]);

        cmd_args
    }

    fn call_aoc_cli(args: &[String]) -> Result<Output, AocCliError> {
        println!("Calling >aoc with: {}", args.join(" "));
        Command::new("aoc")
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|_| AocCliError::CommandNotCallable)
    }
}

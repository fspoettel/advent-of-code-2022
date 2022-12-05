/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use advent_of_code::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
use std::io;

pub struct Timings {
    part_1: Option<String>,
    part_2: Option<String>,
    parser: Option<String>,
    total_nanos: f64,
}

#[derive(Debug)]
pub enum Error {
    BrokenPipe,
    Parser(String),
    IO(io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

pub fn get_path_for_bin(day: usize) -> String {
    let day_padded = format!("{:02}", day);
    format!("./src/bin/{}.rs", day_padded)
}

mod child_commands {
    use super::{get_path_for_bin, Error};
    use std::{
        io::{BufRead, BufReader},
        path::Path,
        process::{Command, Stdio},
        thread,
    };

    /// Run the solution bin for a given day
    pub fn run_solution(day: usize) -> Result<Vec<String>, Error> {
        let day_padded = format!("{:02}", day);

        // skip command invocation for days that have not been scaffolded yet.
        if !Path::new(&get_path_for_bin(day)).exists() {
            return Ok(vec![]);
        }

        let mut args = vec!["run", "--quiet", "--bin", &day_padded];
        if cfg!(not(debug_assertions)) {
            // mirror `--release` flag to child invocations.
            args.push("--release");
        }

        // spawn child command with piped stdout/stderr.
        // forward output to stdout/stderr while grabbing stdout lines.

        let mut cmd = Command::new("cargo")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = BufReader::new(cmd.stdout.take().ok_or(super::Error::BrokenPipe)?);
        let stderr = BufReader::new(cmd.stderr.take().ok_or(super::Error::BrokenPipe)?);

        let mut output = vec![];

        let thread = thread::spawn(move || {
            stderr.lines().for_each(|line| {
                eprintln!("{}", line.unwrap());
            });
        });

        for line in stdout.lines() {
            let line = line.unwrap();
            println!("{}", line);
            output.push(line);
        }

        thread.join().unwrap();
        cmd.wait()?;

        Ok(output)
    }

    pub fn parse_exec_time(output: &[String]) -> super::Timings {
        let mut timings = super::Timings {
            part_1: None,
            part_2: None,
            parser: None,
            total_nanos: 0_f64,
        };

        output
            .iter()
            .filter_map(|l| {
                if !l.contains("(avg. time:") {
                    return None;
                }

                let (timing_str, nanos) = match parse_time(l) {
                    Some(v) => v,
                    None => {
                        eprintln!("Could not parse timings from line: {l}");
                        return None;
                    }
                };

                let part = l.split(':').next()?;
                Some((part, timing_str, nanos))
            })
            .for_each(|(part, timing_str, nanos)| {
                match part {
                    "part 1" => {
                        timings.part_1 = Some(timing_str.into());
                    }
                    "part 2" => {
                        timings.part_2 = Some(timing_str.into());
                    }
                    "parser" => {
                        timings.parser = Some(timing_str.into());
                    }
                    s => {
                        eprintln!("tried to collect timing for unknown solution part: {}", s)
                    }
                };

                timings.total_nanos += nanos;
            });

        timings
    }

    fn parse_to_float(s: &str, postfix: &str) -> Option<f64> {
        s.split(postfix).next()?.parse().ok()
    }

    fn parse_time(line: &str) -> Option<(&str, f64)> {
        // for possible time formats, see: https://github.com/rust-lang/rust/blob/1.64.0/library/core/src/time.rs#L1176-L1200
        let str_timing = line.split("(avg. time:").last()?.split('@').next()?.trim();

        let parsed_timing = match str_timing {
            s if s.contains("ns") => s.split("ns").next()?.parse::<f64>().ok(),
            s if s.contains("µs") => parse_to_float(s, "µs").map(|x| x * 1000_f64),
            s if s.contains("ms") => parse_to_float(s, "ms").map(|x| x * 1000000_f64),
            s => parse_to_float(s, "s").map(|x| x * 1000000000_f64),
        }?;

        Some((str_timing, parsed_timing))
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
        use super::parse_exec_time;

        #[test]
        fn test_well_formed() {
            let res = parse_exec_time(&[
                "parser: ✓ (avg. time: 7.3µs @ 6579 samples)".into(),
                "part 1: 0 (avg. time: 74.13ns @ 100000 samples)".into(),
                "part 2: 10 (avg. time: 74.13ms @ 99999 samples)".into(),
                "".into(),
            ]);
            assert_approx_eq!(res.total_nanos, 74137374.13_f64);
            assert_eq!(res.parser.unwrap(), "7.3µs");
            assert_eq!(res.part_1.unwrap(), "74.13ns");
            assert_eq!(res.part_2.unwrap(), "74.13ms");
        }

        #[test]
        fn test_patterns_in_input() {
            let res = parse_exec_time(&[
                "parser: ✓    (avg. time: 1s @ 5 samples)".into(),
                "part 1: @ @ @ ms (avg. time: 2s @ 5 samples)".into(),
                "part 2: 10s (avg. time: 100ms @ 1 samples)".into(),
                "".into(),
            ]);
            assert_approx_eq!(res.total_nanos, 3100000000_f64);
            assert_eq!(res.parser.unwrap(), "1s");
            assert_eq!(res.part_1.unwrap(), "2s");
            assert_eq!(res.part_2.unwrap(), "100ms");
        }

        #[test]
        fn test_missing_parts() {
            let res = parse_exec_time(&[
                "parser: ✓ (avg. time: 1ms @ 6579 samples)".into(),
                "part 1: not solved.".into(),
                "part 2: not solved.".into(),
                "".into(),
            ]);
            assert_approx_eq!(res.total_nanos, 1000000_f64);
            assert_eq!(res.parser.unwrap(), "1ms");
            assert_eq!(res.part_1.is_none(), true);
            assert_eq!(res.part_2.is_none(), true);
        }
    }
}

mod readme {
    use super::{Error, Timings};
    use std::fs;

    static MARKER: &str = "<!--- benchmarking table --->";

    pub struct TablePosition {
        pos_start: usize,
        pos_end: usize,
    }

    fn locate_table(readme: &str) -> Result<TablePosition, Error> {
        let matches: Vec<_> = readme.match_indices(MARKER).collect();

        if matches.len() > 2 {
            return Err(Error::Parser(
                "{}: too many occurences of marker in README.".into(),
            ));
        }

        let pos_start = matches
            .first()
            .map(|m| m.0)
            .ok_or_else(|| Error::Parser("Could not find table start position.".into()))?;

        let pos_end = matches
            .last()
            .map(|m| m.0 + m.1.len())
            .ok_or_else(|| Error::Parser("Could not find table end position.".into()))?;

        Ok(TablePosition { pos_start, pos_end })
    }

    fn construct_table(prefix: &str, timings: Vec<super::Timings>, total_millis: f64) -> String {
        let header = format!("{prefix} Benchmarks");

        let mut lines: Vec<String> = vec![
            MARKER.into(),
            header,
            "".into(),
            "| Day | Parser | Part 1 | Part 2 |".into(),
            "| :---: | :---: | :---: | :---:  |".into(),
        ];

        timings.into_iter().enumerate().for_each(|(i, timing)| {
            let path = super::get_path_for_bin(i + 1);
            lines.push(format!(
                "| [Day {}]({}) | `{}` | `{}` | `{}` |",
                i + 1,
                path,
                timing.parser.unwrap_or_else(|| "-".into()),
                timing.part_1.unwrap_or_else(|| "-".into()),
                timing.part_2.unwrap_or_else(|| "-".into())
            ));
        });

        lines.push("".into());
        lines.push(format!("**Total: {:.2}ms**", total_millis));
        lines.push(MARKER.into());

        lines.join("\n")
    }

    pub fn update(timings: Vec<Timings>, total_millis: f64) -> Result<(), Error> {
        let path = "README.md";
        let mut readme = String::from_utf8_lossy(&fs::read(path)?).to_string();

        let positions = locate_table(&readme)?;
        let table = construct_table("##", timings, total_millis);
        readme.replace_range(positions.pos_start..positions.pos_end, &table);

        fs::write(path, &readme)?;
        Ok(())
    }
}

fn main() {
    let mut timings = vec![];

    (1..=25).enumerate().for_each(|(i, day)| {
        if i > 0 {
            println!();
        }

        println!("{}Day {}{}", ANSI_BOLD, day, ANSI_RESET);
        println!("------");

        let output = child_commands::run_solution(day).unwrap();

        if output.is_empty() {
            println!("Not solved.");
        } else {
            let val = child_commands::parse_exec_time(&output);
            timings.push(val);
        }
    });

    let total_millis = timings.iter().map(|x| x.total_nanos).sum::<f64>() / 1000000_f64;

    println!(
        "\n{}Total:{} {}{:.2}ms{}",
        ANSI_BOLD, ANSI_RESET, ANSI_ITALIC, total_millis, ANSI_RESET
    );

    if cfg!(not(debug_assertions)) {
        println!();
        match readme::update(timings, total_millis) {
            Ok(_) => println!("Successfully updated README with benchmarks."),
            Err(_) => {
                eprintln!("Failed to update readme with benchmarks.");
            }
        }
    }
}

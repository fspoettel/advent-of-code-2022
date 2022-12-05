/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use std::path::PathBuf;
use std::process::Stdio;
use std::{env::temp_dir, process::Command};
use std::{fs, process};

struct Args {
    day: u8,
    year: Option<i16>,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    Ok(Args {
        day: args.free_from_str()?,
        year: args.opt_value_from_str(["-y", "--year"])?,
    })
}

fn remove_dir(path: &PathBuf) {
    #[allow(unused_must_use)]
    {
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_) => {
                return;
            }
        };

        for entry in dir {
            match entry {
                Ok(entry) => fs::remove_file(entry.path()),
                Err(_) => continue,
            };
        }

        fs::remove_dir(path);
    }
}

fn exit_with_status(status: i32, path: &PathBuf) -> ! {
    remove_dir(path);
    process::exit(status);
}

fn main() {
    // acquire a temp file path to write aoc-cli output to.
    // aoc-cli expects this file not to be present - delete just in case.
    let tmp_dir = temp_dir();
    remove_dir(&tmp_dir);

    let mut input_file_path = tmp_dir.clone();
    input_file_path.push("aoc_input_tmp");

    let mut puzzle_file_path = tmp_dir.clone();
    puzzle_file_path.push("aoc_puzzle_tmp");

    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Failed to process arguments: {}", e);
            exit_with_status(1, &tmp_dir);
        }
    };

    let day_padded = format!("{:02}", args.day);
    let input_path = format!("src/inputs/{}.txt", day_padded);
    let puzzle_path = format!("src/puzzles/{}.md", day_padded);

    // check if aoc binary exists and is callable.
    if Command::new("aoc").arg("-V").output().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        exit_with_status(1, &tmp_dir);
    }

    let mut cmd_args = vec![];

    if let Some(year) = args.year {
        cmd_args.push("--year".into());
        cmd_args.push(year.to_string());
    }

    cmd_args.append(&mut vec![
        "--input-file".into(),
        input_file_path.to_string_lossy().to_string(),
        "--puzzle-file".into(),
        puzzle_file_path.to_string_lossy().to_string(),
        "--day".into(),
        args.day.to_string(),
        "download".into(),
    ]);

    println!("Downloading input with >aoc {}", cmd_args.join(" "));

    match Command::new("aoc")
        .args(cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(cmd_output) => {
            if !cmd_output.status.success() {
                exit_with_status(1, &tmp_dir);
            }
        }
        Err(e) => {
            eprintln!("failed to spawn aoc-cli: {}", e);
            exit_with_status(1, &tmp_dir);
        }
    }

    match fs::copy(&input_file_path, &input_path) {
        Ok(_) => {
            println!("---");
            println!("🎄 Successfully wrote input to \"{}\".", &input_path);
        }
        Err(e) => {
            eprintln!("could not copy downloaded input to input file: {}", e);
            exit_with_status(1, &tmp_dir);
        }
    }

    match fs::copy(&puzzle_file_path, &puzzle_path) {
        Ok(_) => {
            println!("🎄 Successfully wrote puzzle to \"{}\".", &puzzle_path);
        }
        Err(e) => {
            eprintln!("could not copy downloaded puzzle to puzzle file: {}", e);
            exit_with_status(1, &tmp_dir);
        }
    }

    exit_with_status(0, &tmp_dir);
}

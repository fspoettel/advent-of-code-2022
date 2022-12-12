/// Module that updates the readme me with timing information.
/// The approach taken is similar to how `aoc-readme-stars` handles this.
use std::{fs, io};

static MARKER: &str = "<!--- benchmarking table --->";

#[derive(Debug)]
pub enum Error {
    Parser(String),
    IO(io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

pub struct Timings {
    pub part_1: Option<String>,
    pub part_2: Option<String>,
    pub parser: Option<String>,
    pub total_nanos: f64,
}

pub struct TablePosition {
    pos_start: usize,
    pos_end: usize,
}

pub fn get_path_for_bin(day: usize) -> String {
    let day_padded = format!("{:02}", day);
    format!("./src/bin/{}.rs", day_padded)
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

fn construct_table(prefix: &str, timings: Vec<Timings>, total_millis: f64) -> String {
    let header = format!("{prefix} Benchmarks");

    let mut lines: Vec<String> = vec![
        MARKER.into(),
        header,
        "".into(),
        "| Day | Parser | Part 1 | Part 2 |".into(),
        "| :---: | :---: | :---: | :---:  |".into(),
    ];

    timings.into_iter().enumerate().for_each(|(i, timing)| {
        let path = get_path_for_bin(i + 1);
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

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Number {
    #[serde(rename(deserialize = "id_type"))]
    r#type: u32,
    direction: DirectionType,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.r#type, self.direction)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum DirectionType {
    #[serde(rename(deserialize = "up"))]
    Up,
    #[serde(rename(deserialize = "down"))]
    Down,
}

impl fmt::Display for DirectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let direction = match self {
            DirectionType::Up => "up",
            DirectionType::Down => "down",
        };
        write!(f, "{direction}")
    }
}

fn run(file: &str) {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_path(file)
        .expect("Error: Failed to read from file");

    let numbers = rdr.deserialize::<Number>();
    for number in numbers {
        match number {
            Ok(n) => println!("{n}"),
            Err(e) => {
                eprintln!("Error: Failed to deserialize {e}");
                continue;
            }
        }
    }
}

fn main() {
    let file = std::env::args().nth(1).unwrap_or_else(|| {
        eprint!("Error: Missing CSV file name. Usage: cargo run -- <csv-file.csv>");
        std::process::exit(1);
    });

    run(&file);
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use clap::Parser;
use anyhow::{anyhow, Result};
use crate::weather::{Measurement, Weather};

mod weather;

#[derive(Parser)]
pub struct Args {
    pub filename: String
}

fn main() {
    let args = Args::parse();
    let map = match read_measurements(&args.filename) {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error: {}", e.to_string());
            exit(-1)
        }
    };

    let mut items = map.into_iter()
        .map(|(name,measurement)| Weather::new(name, measurement))
        .collect::<Vec<_>>();

    items.sort_by(|i, j| i.name.cmp(&j.name));

    match print_solution(&items) {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e.to_string()),
    }
}

fn read_measurements(filename: &str) -> Result<HashMap<String,Measurement>> {
    let mut map = HashMap::<String,Measurement>::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        let mut parts = line.split(";");
        let (name ,value) = match (parts.next(), parts.next()) {
            (Some(name), Some(value)) => (name, value.parse::<f64>()?),
            _ => continue
        };

        match map.get_mut(name) {
            Some(item) => item.add(value),
            _ => { map.insert(name.to_string(), Measurement::new(value)); }
        };
    }

    Ok(map)
}

fn print_solution(items: &Vec<Weather>) -> Result<()> {
    let first = items.first().ok_or(anyhow!("empty collection"))?.summarize();

    println!("{}", '{');
    print!("    {}", first);
    for summary in items.iter().skip(1).map(|item| item.summarize()) {
        print!(",\n    {}", summary);
    }
    print!("\n{}", '}');

    Ok(())
}
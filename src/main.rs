use std::{collections::HashMap, env};
use serde::Serialize;

mod parser;
mod event;

#[derive(Serialize)]
pub struct MatchKills<'a> {
    total_kills: u32,
    players: Vec<&'a str>,
    kills: HashMap<&'a str, i32>,
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err("Insufficient arguments")
    } else {
        let file = std::fs::read_to_string(&args[1]).map_err(|_| "Cannot read file")?;
        let lines = file.lines();
        for line in lines {
            println!("{}", line);
            let log = parser::parse_line(line);
            println!("{:?}", log);
        }
        // let report = generate_report(&file);
        // println!("{}", serde_json::to_string_pretty(&report).unwrap());
        Ok(())
    }
}
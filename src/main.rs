use std::{collections::HashMap, env};
use serde::Serialize;

mod parser;
mod event;

#[derive(Serialize, Debug)]
pub struct MatchKills<'a> {
    total_kills: u32,
    players: Vec<&'a str>,
    kills: HashMap<&'a str, i32>,
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    let mut match_struct = HashMap::new();
    let mut match_num = 0;

    if args.len() < 2 {
        Err("Insufficient arguments")
    } else {
        let file = std::fs::read_to_string(&args[1]).map_err(|_| "Cannot read file")?;
        let lines = file.lines();
        for line in lines {
            let log = parser::parse_line(line);
            let Ok((_, event)) = log else {
                println!("Error parsing line: {}", line);
                continue;
            };
            match event {
                event::Event::Kill(kill) => {
                    let match_number = format!("match_{}", match_num);
                    match_struct.entry(match_number.clone()).or_insert(MatchKills {
                        total_kills: 0,
                        players: vec![],
                        kills: HashMap::new(),
                    });
                    let match_kill = match_struct.get_mut(&match_number).unwrap();
                    match_kill.total_kills += 1;
                    if kill.killer == "<world>" {
                        match_kill.kills.entry(kill.victim).or_insert(0);
                        match_kill.kills.entry(kill.victim).and_modify(|e| *e -= 1);
                        if !match_kill.players.contains(&kill.victim) {
                            match_kill.players.push(kill.victim);
                        }
                    } else {
                        match_kill.kills.entry(kill.killer).or_insert(0);
                        match_kill.kills.entry(kill.killer).and_modify(|e| *e += 1);
                        match_kill.kills.entry(kill.victim).or_insert(0);
                        if !match_kill.players.contains(&kill.killer) {
                            match_kill.players.push(kill.killer);
                        }
                        if !match_kill.players.contains(&kill.victim) {
                            match_kill.players.push(kill.victim);
                        }
                    }
                }
                event::Event::InitGame => {
                    match_num += 1;
                }
                event::Event::ShutdownGame => {}
                _ => {}
            }
        }
        println!("{}", serde_json::to_string_pretty(&match_struct).unwrap());
        Ok(())
    }
}
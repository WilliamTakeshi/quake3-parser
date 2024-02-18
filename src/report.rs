use std::collections::HashMap;

use serde::Serialize;

use crate::event::{Event, MeansOfDeath};
use crate::parser;

#[derive(Serialize, Debug)]
pub struct MatchKills {
    total_kills: u32,
    players: Vec<String>,
    kills: HashMap<String, i32>,
    kills_by_means: HashMap<MeansOfDeath, u32>,
}

pub fn group_game_data(s: String) -> Result<HashMap<String, MatchKills>, &'static str> {
    let mut match_struct = HashMap::new();
    let mut match_num = 0;
    let lines = s.lines();
    for line in lines {
        let log = parser::parse_line(&line);
        let Ok((_, event)) = log else {
            println!("Error parsing line: {}", &line);
            continue;
        };
        match event {
            Event::Kill(kill) => {
                let match_number = format!("match_{}", match_num);
                match_struct
                    .entry(match_number.clone())
                    .or_insert(MatchKills {
                        total_kills: 0,
                        players: vec![],
                        kills: HashMap::new(),
                        kills_by_means: HashMap::new(),
                    });
                let match_kill = match_struct.get_mut(&match_number).unwrap();
                match_kill.total_kills += 1;
                match_kill
                    .kills_by_means
                    .entry(kill.mean_of_death)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                if kill.killer == "<world>" {
                    match_kill
                        .kills
                        .entry(kill.victim.to_string())
                        .and_modify(|e| *e -= 1)
                        .or_insert(-1);
                    if !match_kill.players.contains(&kill.victim.to_string()) {
                        match_kill.players.push(kill.victim.to_string());
                    }
                } else {
                    match_kill
                        .kills
                        .entry(kill.killer.to_string())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                    match_kill.kills.entry(kill.victim.to_string()).or_insert(0);
                    if !match_kill.players.contains(&kill.killer.to_string()) {
                        match_kill.players.push(kill.killer.to_string());
                    }
                    if !match_kill.players.contains(&kill.victim.to_string()) {
                        match_kill.players.push(kill.victim.to_string());
                    }
                }
            }
            Event::ClientUserinfoChanged(player) => {
                let match_number = format!("match_{}", match_num);
                match_struct
                    .entry(match_number.clone())
                    .or_insert(MatchKills {
                        total_kills: 0,
                        players: vec![],
                        kills: HashMap::new(),
                        kills_by_means: HashMap::new(),
                    });
                let match_kill = match_struct.get_mut(&match_number).unwrap();
                match_kill.kills.entry(player.to_string()).or_insert(0);
                if !match_kill.players.contains(&player.to_string()) {
                    match_kill.players.push(player.to_string());
                }
            }
            Event::InitGame => {
                match_num += 1;
            }
            Event::ShutdownGame => {}
            _ => {}
        }
    }

    Ok(match_struct)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_game_data() {
        let file = std::fs::read_to_string("test.log").unwrap();
        let report: HashMap<String, MatchKills> = group_game_data(file.to_string()).unwrap();

        let match_0 = report.get("match_1").unwrap();
        assert_eq!(match_0.total_kills, 0);
        assert_eq!(match_0.players.len(), 1);
        assert_eq!(match_0.kills.get("Isgalamido").unwrap(), &0);
        assert_eq!(match_0.kills_by_means, HashMap::new());
        let match_1 = report.get("match_2").unwrap();
        assert_eq!(match_1.total_kills, 11);
        assert_eq!(match_1.players.len(), 3);
        assert_eq!(match_1.kills.get("Isgalamido").unwrap(), &-5);
        assert_eq!(match_1.kills.get("Dono da Bola").unwrap(), &0);
        assert_eq!(match_1.kills.get("Mocinha").unwrap(), &0);
        assert_eq!(match_1.kills_by_means.get(&MeansOfDeath::ModRocketSplash).unwrap(), &3);
        assert_eq!(match_1.kills_by_means.get(&MeansOfDeath::ModFalling).unwrap(), &1);
        assert_eq!(match_1.kills_by_means.get(&MeansOfDeath::ModTriggerHurt).unwrap(), &7);
    }
}
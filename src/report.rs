use std::collections::HashMap;
use std::collections::HashSet;

use serde::Serialize;

use crate::event::{Event, MeansOfDeath};
use crate::parser;

#[derive(Serialize, Debug)]
pub struct GameKills {
    total_kills: u32,
    players: HashSet<String>,
    kills: HashMap<String, i32>,
    kills_by_means: HashMap<MeansOfDeath, u32>,
}

pub fn group_game_data(s: String) -> Result<HashMap<String, GameKills>, &'static str> {
    let mut game_struct = HashMap::new();
    let mut game_num = 0;
    let lines = s.lines();
    for line in lines {
        let log = parser::parse_line(&line);
        let Ok((_, event)) = log else {
            println!("Error parsing line: {}", &line);
            continue;
        };
        match event {
            Event::Kill(kill) => {
                let game_number = format!("game_{}", game_num);
                game_struct
                    .entry(game_number.clone())
                    .or_insert(GameKills {
                        total_kills: 0,
                        players: HashSet::new(),
                        kills: HashMap::new(),
                        kills_by_means: HashMap::new(),
                    });
                let game_kill = game_struct.get_mut(&game_number).unwrap();
                game_kill.total_kills += 1;
                game_kill
                    .kills_by_means
                    .entry(kill.mean_of_death)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                if kill.killer == "<world>" {
                    game_kill
                        .kills
                        .entry(kill.victim.to_string())
                        .and_modify(|e| *e -= 1)
                        .or_insert(-1);
                    game_kill.players.insert(kill.victim.to_string());
                } else {
                    game_kill
                        .kills
                        .entry(kill.killer.to_string())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                    game_kill.kills.entry(kill.victim.to_string()).or_insert(0);
                    game_kill.players.insert(kill.killer.to_string());
                    game_kill.players.insert(kill.victim.to_string());
                }
            }
            Event::ClientUserinfoChanged(player) => {
                let game_number = format!("game_{}", game_num);
                game_struct
                    .entry(game_number.clone())
                    .or_insert(GameKills {
                        total_kills: 0,
                        players: HashSet::new(),
                        kills: HashMap::new(),
                        kills_by_means: HashMap::new(),
                    });
                let game_kill = game_struct.get_mut(&game_number).unwrap();
                game_kill.kills.entry(player.to_string()).or_insert(0);
                game_kill.players.insert(player.to_string());
            }
            Event::InitGame => {
                game_num += 1;
            }
            Event::ShutdownGame => {}
            _ => {}
        }
    }

    Ok(game_struct)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_game_data() {
        let file = std::fs::read_to_string("test.log").unwrap();
        let report: HashMap<String, GameKills> = group_game_data(file.to_string()).unwrap();

        let game_0 = report.get("game_1").unwrap();
        assert_eq!(game_0.total_kills, 0);
        assert_eq!(game_0.players.len(), 1);
        assert_eq!(game_0.kills.get("Isgalamido").unwrap(), &0);
        assert_eq!(game_0.kills_by_means, HashMap::new());
        let game_1 = report.get("game_2").unwrap();
        assert_eq!(game_1.total_kills, 11);
        assert_eq!(game_1.players.len(), 3);
        assert_eq!(game_1.kills.get("Isgalamido").unwrap(), &-5);
        assert_eq!(game_1.kills.get("Dono da Bola").unwrap(), &0);
        assert_eq!(game_1.kills.get("Mocinha").unwrap(), &0);
        assert_eq!(game_1.kills_by_means.get(&MeansOfDeath::ModRocketSplash).unwrap(), &3);
        assert_eq!(game_1.kills_by_means.get(&MeansOfDeath::ModFalling).unwrap(), &1);
        assert_eq!(game_1.kills_by_means.get(&MeansOfDeath::ModTriggerHurt).unwrap(), &7);
    }
}
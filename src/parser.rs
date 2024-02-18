use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{digit1, multispace0},
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct KillFeed<'a> {
    pub killer: &'a str,
    pub victim: &'a str,
    pub mean_of_death: MeansOfDeath,
}
#[derive(Debug, PartialEq, Eq)]
pub enum MeansOfDeath {
    ModUnknown,
    ModShotgun,
    ModGauntlet,
    ModMachinegun,
    ModGrenade,
    ModGrenadeSplash,
    ModRocket,
    ModRocketSplash,
    ModPlasma,
    ModPlasmaSplash,
    ModRailgun,
    ModLightning,
    ModBfg,
    ModBfgSplash,
    ModWater,
    ModSlime,
    ModLava,
    ModCrush,
    ModTelefrag,
    ModFalling,
    ModSuicide,
    ModTargetLaser,
    ModTriggerHurt,
    ModNail,
    ModChaingun,
    ModProximityMine,
    ModKamikaze,
    ModJuiced,
    ModGrapple,
}

impl ToString for MeansOfDeath {
    fn to_string(&self) -> String {
        match self {
            MeansOfDeath::ModUnknown => "MOD_UNKNOWN".to_string(),
            MeansOfDeath::ModShotgun => "MOD_SHOTGUN".to_string(),
            MeansOfDeath::ModGauntlet => "MOD_GAUNTLET".to_string(),
            MeansOfDeath::ModMachinegun => "MOD_MACHINEGUN".to_string(),
            MeansOfDeath::ModGrenade => "MOD_GRENADE".to_string(),
            MeansOfDeath::ModGrenadeSplash => "MOD_GRENADE_SPLASH".to_string(),
            MeansOfDeath::ModRocket => "MOD_ROCKET".to_string(),
            MeansOfDeath::ModRocketSplash => "MOD_ROCKET_SPLASH".to_string(),
            MeansOfDeath::ModPlasma => "MOD_PLASMA".to_string(),
            MeansOfDeath::ModPlasmaSplash => "MOD_PLASMA_SPLASH".to_string(),
            MeansOfDeath::ModRailgun => "MOD_RAILGUN".to_string(),
            MeansOfDeath::ModLightning => "MOD_LIGHTNING".to_string(),
            MeansOfDeath::ModBfg => "MOD_BFG".to_string(),
            MeansOfDeath::ModBfgSplash => "MOD_BFG_SPLASH".to_string(),
            MeansOfDeath::ModWater => "MOD_WATER".to_string(),
            MeansOfDeath::ModSlime => "MOD_SLIME".to_string(),
            MeansOfDeath::ModLava => "MOD_LAVA".to_string(),
            MeansOfDeath::ModCrush => "MOD_CRUSH".to_string(),
            MeansOfDeath::ModTelefrag => "MOD_TELEFRAG".to_string(),
            MeansOfDeath::ModFalling => "MOD_FALLING".to_string(),
            MeansOfDeath::ModSuicide => "MOD_SUICIDE".to_string(),
            MeansOfDeath::ModTargetLaser => "MOD_TARGET_LASER".to_string(),
            MeansOfDeath::ModTriggerHurt => "MOD_TRIGGER_HURT".to_string(),
            MeansOfDeath::ModNail => "MOD_NAIL".to_string(),
            MeansOfDeath::ModChaingun => "MOD_CHAINGUN".to_string(),
            MeansOfDeath::ModProximityMine => "MOD_PROXIMITY_MINE".to_string(),
            MeansOfDeath::ModKamikaze => "MOD_KAMIKAZE".to_string(),
            MeansOfDeath::ModJuiced => "MOD_JUICED".to_string(),
            MeansOfDeath::ModGrapple => "MOD_GRAPPLE".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMeansOfDeathError;

impl FromStr for MeansOfDeath {
    type Err = ParseMeansOfDeathError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "MOD_UNKNOWN" => Ok(MeansOfDeath::ModUnknown),
            "MOD_SHOTGUN" => Ok(MeansOfDeath::ModShotgun),
            "MOD_GAUNTLET" => Ok(MeansOfDeath::ModGauntlet),
            "MOD_MACHINEGUN" => Ok(MeansOfDeath::ModMachinegun),
            "MOD_GRENADE" => Ok(MeansOfDeath::ModGrenade),
            "MOD_GRENADE_SPLASH" => Ok(MeansOfDeath::ModGrenadeSplash),
            "MOD_ROCKET" => Ok(MeansOfDeath::ModRocket),
            "MOD_ROCKET_SPLASH" => Ok(MeansOfDeath::ModRocketSplash),
            "MOD_PLASMA" => Ok(MeansOfDeath::ModPlasma),
            "MOD_PLASMA_SPLASH" => Ok(MeansOfDeath::ModPlasmaSplash),
            "MOD_RAILGUN" => Ok(MeansOfDeath::ModRailgun),
            "MOD_LIGHTNING" => Ok(MeansOfDeath::ModLightning),
            "MOD_BFG" => Ok(MeansOfDeath::ModBfg),
            "MOD_BFG_SPLASH" => Ok(MeansOfDeath::ModBfgSplash),
            "MOD_WATER" => Ok(MeansOfDeath::ModWater),
            "MOD_SLIME" => Ok(MeansOfDeath::ModSlime),
            "MOD_LAVA" => Ok(MeansOfDeath::ModLava),
            "MOD_CRUSH" => Ok(MeansOfDeath::ModCrush),
            "MOD_TELEFRAG" => Ok(MeansOfDeath::ModTelefrag),
            "MOD_FALLING" => Ok(MeansOfDeath::ModFalling),
            "MOD_SUICIDE" => Ok(MeansOfDeath::ModSuicide),
            "MOD_TARGET_LASER" => Ok(MeansOfDeath::ModTargetLaser),
            "MOD_TRIGGER_HURT" => Ok(MeansOfDeath::ModTriggerHurt),
            "MOD_NAIL" => Ok(MeansOfDeath::ModNail),
            "MOD_CHAINGUN" => Ok(MeansOfDeath::ModChaingun),
            "MOD_PROXIMITY_MINE" => Ok(MeansOfDeath::ModProximityMine),
            "MOD_KAMIKAZE" => Ok(MeansOfDeath::ModKamikaze),
            "MOD_JUICED" => Ok(MeansOfDeath::ModJuiced),
            "MOD_GRAPPLE" => Ok(MeansOfDeath::ModGrapple),
            _ => Err(ParseMeansOfDeathError),
        }
    }
}

pub fn parse_kill_log(input: &str) -> IResult<&str, KillFeed> {
    // Throw the first part away
    // " 25:52 Kill: 1022 2 22: "
    let (input, _) = multispace0(input)?;
    let (input, _) = digit1(input)?; // hour
    let (input, _) = tag(":")(input)?;
    let (input, _) = digit1(input)?; // minute
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("Kill:")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, (killer, _, victim, _, mean_of_death)) = tuple((
        take_until(" killed "), // killer is everything until " killed "
        tag(" killed "),        // ignore this tag
        take_until(" by "),     // victim is everything until " by "
        tag(" by "),            // ignore this tag
        take_while1(|c: char| c.is_ascii_uppercase() || c == '_'), // take_until("\n"),        // cause is everything until the end of the line
    ))(input)?;

    let Ok(mean_of_death) = mean_of_death.parse::<MeansOfDeath>() else {
        return Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
            mean_of_death,
            nom::error::ErrorKind::Tag,
        )));
    };

    Ok((
        input,
        KillFeed {
            killer,
            victim,
            mean_of_death,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_kill_log() {
        let input = " 25:52 Kill: 1022 2 22: Isgalamido killed Mocinha by MOD_ROCKET";
        let expected = KillFeed {
            killer: "Isgalamido",
            victim: "Mocinha",
            mean_of_death: MeansOfDeath::ModRocket,
        };
        let result = parse_kill_log(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_kill_log_name_with_underscore() {
        let input = " 25:52 Kill: 1022 2 22: Isgal_amido killed Mocinha by MOD_ROCKET";
        let expected = KillFeed {
            killer: "Isgal_amido",
            victim: "Mocinha",
            mean_of_death: MeansOfDeath::ModRocket,
        };
        let result = parse_kill_log(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_kill_log_invalid_mean_of_death() {
        let input = " 25:52 Kill: 1022 2 22: Isgalamido killed Mocinha by MOD_INVALID";

        let result = parse_kill_log(input);
        assert_eq!(
            result,
            Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
                "MOD_INVALID",
                nom::error::ErrorKind::Tag,
            )))
        );
    }
}

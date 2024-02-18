use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, PartialEq)]
pub enum Event<'a> {
    Kill(KillFeed<'a>),
    ClientUserinfoChanged(&'a str),
    InitGame,
    ShutdownGame,
    Ignored,
}

#[derive(Debug, PartialEq, Eq)]
pub struct KillFeed<'a> {
    pub killer: &'a str,
    pub victim: &'a str,
    pub mean_of_death: MeansOfDeath,
}
#[derive(Debug, PartialEq, Eq, Hash)]
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

impl Serialize for MeansOfDeath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
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

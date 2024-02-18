use crate::event::{Event, KillFeed, MeansOfDeath};
use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{digit1, multispace0},
    sequence::tuple,
    IResult,
};

pub fn parse_line(input: &str) -> IResult<&str, Event> {
    // Throw the first part away
    // " 25:52 "
    let (input, _) = multispace0(input)?;
    let (input, _) = digit1(input)?; // hour
    let (input, _) = tag(":")(input)?;
    let (input, _) = digit1(input)?; // minute
    let (input, _) = multispace0(input)?;
    let (input, tag) = take_while1(|c: char| c.is_ascii_alphabetic() || c == ':')(input)?;

    match tag {
        "Kill:" => parse_kill_log(input),
        _ => Ok((input, Event::Ignored)),
    }
}

pub fn parse_kill_log(input: &str) -> IResult<&str, Event> {
    // Throw the first part away
    // " 1022 2 22: "

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
        Event::Kill(KillFeed {
            killer,
            victim,
            mean_of_death,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_kill_log() {
        let input = " 1022 2 22: Isgalamido killed Mocinha by MOD_ROCKET";
        let expected = Event::Kill(KillFeed {
            killer: "Isgalamido",
            victim: "Mocinha",
            mean_of_death: MeansOfDeath::ModRocket,
        });
        let result = parse_kill_log(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_kill_log_name_with_underscore() {
        let input = " 1022 2 22: Isgal_amido killed Mocinha by MOD_ROCKET";
        let expected = Event::Kill(KillFeed {
            killer: "Isgal_amido",
            victim: "Mocinha",
            mean_of_death: MeansOfDeath::ModRocket,
        });
        let result = parse_kill_log(input);
        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn test_parse_kill_log_invalid_mean_of_death() {
        let input = " 1022 2 22: Isgalamido killed Mocinha by MOD_INVALID";

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

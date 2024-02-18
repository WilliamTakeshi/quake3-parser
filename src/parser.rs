use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0},
    sequence::{tuple},
    IResult,
};

#[derive(Debug)]
pub struct KillLog<'a> {
    pub killer: &'a str,
    pub killed: &'a str,
    pub cause: &'a str,
}

pub fn parse_kill_log(input: &str) -> IResult<&str, KillLog> {
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


    let (input, (killer, _, killed, _, cause)) = tuple((
        take_until(" killed "), // killer is everything until " killed "
        tag(" killed "), // ignore this tag
        take_until(" by "), // killed is everything until " by "
        tag(" by "), // ignore this tag
        take_until("\n"), // cause is everything until the end of the line
    ))(input)?;

    Ok((
        input,
        KillLog {
            killer,
            killed,
            cause,
        },
    ))
}

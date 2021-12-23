use crate::RValue;
use nom::combinator::map;
use nom::error::ParseError;
use nom::number::complete::double;
use nom::IResult;

pub fn parse_number<'a, E>(input: &'a str) -> IResult<&'a str, RValue<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(double, RValue::N)(input)
}

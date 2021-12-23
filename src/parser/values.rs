use nom::branch::alt;
use nom::character::complete::{char, multispace1, none_of};
use nom::combinator::map;
use nom::error::{FromExternalError, ParseError};
use nom::multi::{fold_many0, separated_list0};
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, terminated};
use nom::{AsChar, IResult, InputTakeAtPosition};

use crate::parser::number::parse_number;
use crate::parser::whitespace::multispace_no_newline_1;
use crate::parser::{number, quoted_string, string};
use crate::RValue;

fn parse_value<'a, E>(input: &'a str) -> IResult<&'a str, RValue<'a>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        quoted_string::parse_quoted_string,
        map(string::parse_string, |s| match s {
            RValue::S(str_content) => match number::parse_number::<E>(str_content) {
                Ok(("", success_number)) => success_number,
                _ => RValue::S(str_content),
            },
            non_str_val => non_str_val,
        }),
    ))(input)
}

pub(crate) fn parse_value_list<'a, E>(input: &'a str) -> IResult<&'a str, Vec<RValue<'a>>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    separated_list0(multispace_no_newline_1, parse_value)(input)
}

#[cfg(test)]
mod test {
    use nom::error::ErrorKind;

    use super::*;

    #[test]
    fn test_vst_params() {
        let input = "\"VST: Streaming Plugin (Distopik)\" StreamingPlugin.vst 0 \"\" 54811357<56535403445ADD73747265616D696E67> \"\"\n";
        let parsed = parse_value_list::<(_, ErrorKind)>(input).unwrap().1;
    }
}

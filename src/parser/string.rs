use nom::bytes::complete::{take_till1, take_until};
use nom::character::complete::{anychar, char, multispace1};
use nom::character::streaming::space1;
use nom::combinator::map;
use nom::error::{FromExternalError, ParseError};
use nom::sequence::terminated;
use nom::{AsChar, IResult, InputTakeAtPosition};

use crate::RValue;

use super::whitespace::multispace_no_newline_1;

pub fn parse_string<'a, E>(input: &'a str) -> IResult<&'a str, RValue<'a>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    map(take_till1(char::is_whitespace), RValue::S)(input)
}

#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use nom::error::ErrorKind;

    use crate::RValue;

    use super::parse_string;

    #[test]
    fn test_string_simple() {
        assert_matches!(parse_string::<(_, ErrorKind)>("hello world"), Ok((" world", RValue::S("hello"))));
    }

    #[test]
    fn test_string_dotted() {
        assert_matches!(
            parse_string::<(_, ErrorKind)>("StreamingPlugin.vst"),
            Ok(("", RValue::S("StreamingPlugin.vst")))
        );
    }

    #[test]
    fn test_string_angled() {
        assert_matches!(
            parse_string::<(_, ErrorKind)>("54811357<56535403445ADD73747265616D696E67>"),
            Ok(("", RValue::S("54811357<56535403445ADD73747265616D696E67>")))
        );
    }
}

use nom::branch::alt;
use nom::bytes::complete::{escaped_transform, is_not, tag};
use nom::character::complete::{anychar, char};
use nom::combinator::{map, not, value};
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::IResult;

use crate::RValue;

pub fn parse_quoted_string<'a, E>(input: &'a str) -> IResult<&'a str, RValue<'a>, E>
where
    E: ParseError<&'a str>,
{
    map(
        delimited(
            char('"'),
            escaped_transform(
                is_not("\"\\"),
                '\\',
                alt((
                    value("\\", tag("\\")),
                    value("\"", tag("\"")),
                    value("\n", tag("n")),
                    value("\t", tag("t")),
                )),
            ),
            char('"'),
        ),
        RValue::QS,
    )(input)
}

#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use nom::error::ErrorKind;

    use super::*;

    #[test]
    fn simple() {
        let foo = parse_quoted_string::<(&'static str, ErrorKind)>(r#""this is a quoted string""#)
            .unwrap()
            .1;
        assert_matches!(&foo, RValue::QS(qs) if qs == "this is a quoted string");
        assert_matches!(foo.to_string().as_str(), r#""this is a quoted string""#);
    }

    #[test]
    fn escaped_double_quote() {
        let foo =
            parse_quoted_string::<(&'static str, ErrorKind)>(r#""this is a \"quoted\" string""#)
                .unwrap()
                .1;
        assert_matches!(&foo, RValue::QS(qs) if qs == "this is a \"quoted\" string");
        assert_matches!(foo.to_string().as_str(), r#""this is a \"quoted\" string""#);
    }

    #[test]
    fn escaped_tabs() {
        let foo =
            parse_quoted_string::<(&'static str, ErrorKind)>(r#""this is a \tquoted\t string""#)
                .unwrap()
                .1;
        assert_matches!(&foo, RValue::QS(qs) if qs == "this is a \tquoted\t string");
        assert_matches!(foo.to_string().as_str(), r#""this is a \tquoted\t string""#);
    }
}

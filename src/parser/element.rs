use std::collections::HashMap;

use nom::{AsChar, InputTakeAtPosition, IResult};
use nom::branch::alt;
use nom::character::complete::{alphanumeric1, char, multispace0, multispace1};
use nom::combinator::{map, success};
use nom::error::{ErrorKind, FromExternalError, ParseError};
use nom::multi::fold_many0;
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, terminated, tuple};

use crate::{RElement, RFragment, RFragmentId, RValue};
use crate::parser::{number, values};
use crate::parser::whitespace::whitespace_line_ending;

use super::identifier::parse_identifier;
use super::quoted_string::parse_quoted_string;
use super::whitespace::{multispace_no_newline_0, multispace_no_newline_1};

pub(crate) fn parse_attribute<'a, E>(input: &'a str) -> IResult<&'a str, RFragment<'a>, E>
  where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
  map(
    tuple((terminated(parse_identifier, multispace_no_newline_1), values::parse_value_list)),
    |(id, values)| RFragment::Attribute(id, values),
  )(input)
}

pub(crate) fn parse_child_element<'a, E>(input: &'a str) -> IResult<&'a str, RFragment<'a>, E>
  where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
  map(parse_element, RFragment::Child)(input)
}

pub fn parse_bin_data_body<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
  where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
  input.split_at_position_complete(|item| {
    let c = item.as_char();
    !(c.is_alpha() || c.is_digit(16) || c == '=' || c == '+' || c == '/')
  })
}

pub(crate) fn parse_bin_data<'a, E>(input: &'a str) -> IResult<&'a str, RFragment<'a>, E>
  where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
  map(parse_bin_data_body, |s: &str| RFragment::BinData(s.to_string()))(input)
}

pub(crate) fn parse_element_fragment<'a, E>(input: &'a str) -> IResult<&'a str, RFragment<'a>, E>
  where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
  preceded(
    multispace_no_newline_0,
    terminated(
      alt((
        parse_child_element,
        parse_attribute,
        parse_bin_data,
        map(success(""), |_| RFragment::Empty),
      )),
      whitespace_line_ending,
    ),
  )(input)
}

pub(crate) fn parse_element_body<'a, E>(input: &'a str) -> IResult<&'a str, RElement<'a>, E>
  where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
  map(
    tuple((
      terminated(parse_identifier, multispace_no_newline_0),
      terminated(values::parse_value_list, whitespace_line_ending),
      fold_many0(parse_element_fragment, RElement::default, |mut element, fragment| {
        element.content.push(fragment);
        element
      }),
    )),
    |(tag, args, element)| RElement { tag, args, ..element },
  )(input)
}

pub fn parse_element<'a, E>(input: &'a str) -> IResult<&'a str, RElement<'a>, E>
  where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
  delimited(char('<'), parse_element_body, tuple((multispace0, char('>'))))(input)
}

#[cfg(test)]
mod test {
  use nom::error::ErrorKind;

  use crate::is_fragment_attribute;

  use super::*;

  fn contains_attribute(v: &Vec<RFragment>, name: &str) -> bool {
    v.iter().filter_map(is_fragment_attribute(name)).next().is_some()
  }

  #[test]
  fn identifier() {
    assert_matches!(
            parse_identifier::<(&'static str, ErrorKind)>(r#"IDENTIFIER"#),
            Ok((_, "IDENTIFIER"))
        );
  }

  #[test]
  fn simple_tag() {
    assert_matches!(
            parse_element::<(&'static str, ErrorKind)>("<RENDER_CFG\n>"),
            Ok(("", RElement { tag: "RENDER_CFG", .. }))
        );
  }

  #[test]
  fn test_basic_metronome() {
    assert_matches!(
            parse_element::<(&'static str, ErrorKind)>("<METRONOME 6 2\n>"),
            Ok((
                "",
                RElement {
                    tag: "METRONOME",
                    args,
                    ..
                }
            )) if matches!(&args[..], &[RValue::N(6.0), RValue::N(2.0)])
        );
  }

  #[test]
  fn test_full_metronome() {
    let input = r#"<METRONOME 6 2
    VOL 0.25 0.125
    FREQ 800 1600 1
    BEATLEN 4
    SAMPLES "" ""
    PATTERN 2863311530 2863311529
    MULT 1
  >"#;
    assert_matches!(
            parse_element::<(&'static str, ErrorKind)>(input),
            Ok((
                "",
                RElement {
                    tag: "METRONOME",
                    args,
                    content,
                    ..
                }
            )) if matches!(&args[..], &[RValue::N(6.0), RValue::N(2.0)]) && contains_attribute(&content, "VOL") && contains_attribute(&content, "MULT")
        );
  }

  #[test]
  fn test_fx_no_bin() {
    let input = r#"<VST "VST: Streaming Plugin (Distopik)" StreamingPlugin.vst 0 "" 54811357<56535403445ADD73747265616D696E67> ""
        >
        "#;
    let parsed = parse_element::<(_, ErrorKind)>(input).unwrap().1;
  }

  #[test]
  fn test_full_fxlist() {
    let input = r#"<MASTERFXLIST
    SHOW 0
    LASTSEL 0
    DOCKED 0
    BYPASS 0 0 0
    <VST "VST: Streaming Plugin (Distopik)" StreamingPlugin.vst 0 "" 54811357<56535403445ADD73747265616D696E67> ""
      3VpEA+9e7f4CAAAAAQAAAAAAAAACAAAAAAAAAAIAAAABAAAAAAAAAAIAAAAAAAAACAAAAAAAAAAAABAA
      776t3g3wrd4=
      AFByb2dyYW0gMQAQAAAA
    >
    PRESETNAME "Program 1"
    FLOATPOS 1518 499 486 342
    FXID {8565B6E8-202B-9E42-AA07-4B0F7E6955E1}
    WAK 0 0
  >"#;
    let parsed = parse_element::<(_, ErrorKind)>(input).unwrap().1;
  }
}

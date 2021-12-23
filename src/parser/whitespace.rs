use nom::character::complete::{char, line_ending, newline};
use nom::error::{ErrorKind, ParseError};
use nom::sequence::{terminated, tuple};
use nom::{AsChar, IResult, InputTakeAtPosition};

pub fn multispace_no_newline_0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        !(c == ' ' || c == '\t')
    })
}

pub fn whitespace_line_ending<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    terminated(multispace_no_newline_0, line_ending)(input)
}

pub fn multispace_no_newline_1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !(c == ' ' || c == '\t')
        },
        ErrorKind::MultiSpace,
    )
}

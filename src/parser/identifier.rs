use nom::error::{ErrorKind, ParseError};
use nom::{AsChar, IResult, InputTakeAtPosition};

pub fn parse_identifier<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    input.split_at_position1_complete(
        |item| {
            !(match (item, item.is_alpha() | item.is_numeric()) {
                ('_', _) => true,
                (_, true) => true,
                _ => false,
            })
        },
        ErrorKind::IsNot,
    )
}

pub use element::parse_element;

mod element;
mod identifier;
mod number;
mod quoted_string;
mod string;
mod values;
mod whitespace;

#[cfg(test)]
mod integ_test {
    use std::fs;

    use nom::error::ErrorKind;

    use super::element::parse_element;

    #[test]
    fn integ_test_print_debug() {
        let input = include_str!("../../StreamingPlugin.rpp");
        let parsed = parse_element::<(_, ErrorKind)>(input).unwrap().1;
        println!("{parsed:?}");
    }

    #[test]
    fn integ_test_reprint() {
        let input = include_str!("../../StreamingPlugin.rpp");
        let parsed = parse_element::<(_, ErrorKind)>(input).unwrap().1;
        let formatted = parsed.to_string();
        println!("{formatted}");
    }

    #[test]
    fn integ_test_reparse() {
        let input = include_str!("../../StreamingPlugin.rpp");
        let parsed = parse_element::<(_, ErrorKind)>(input).unwrap().1;
        let formatted = parsed.to_string();
        let parsed = parse_element::<(_, ErrorKind)>(&formatted).unwrap().1;
    }

    #[test]
    fn integ_test_rewrite() {
        let input = include_str!("../../StreamingPlugin.rpp");
        let parsed = parse_element::<(_, ErrorKind)>(input).unwrap().1;
        let formatted = parsed.to_string();
        fs::write("StreamingPlugin-Copy.rpp", formatted.into_bytes()).unwrap();
    }
}

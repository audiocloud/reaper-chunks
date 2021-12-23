use crate::{RElement, RValue};

pub struct Project<'a>(RElement<'a>);

impl<'a> Project<'a> {
    pub fn reaper_version(&'a self) -> &'a str {
        self.0
            .args
            .get(1)
            .and_then(|arg| match arg {
                RValue::QS(s) => Some(s.as_str()),
                _ => None,
            })
            .unwrap_or_else(|| "")
    }
}

#[cfg(test)]
mod test {
    use nom::error::ErrorKind;

    use super::*;

    #[test]
    fn simple_rpp_version_test() {
        let input = r#"<REAPER_PROJECT 0.1 "6.42/macOS-arm64" 1640001046
        >"#;
        let element = crate::parser::parse_element::<(_, ErrorKind)>(input)
            .unwrap()
            .1;

        let project = Project(element);
        assert_eq!(project.reaper_version(), "6.42/macOS-arm64")
    }
}

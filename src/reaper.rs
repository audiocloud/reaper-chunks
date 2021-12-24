use crate::{RElement, RValue};

pub struct Project<'a>(RElement<'a>);

impl<'a> Project<'a> {
    pub fn reaper_version(&'a self) -> &'a str {
        self.0.get_str_arg(1)
    }

    pub fn tracks(&'a self) -> Vec<Track<'a>> {
        self.0.elements_of_tag("TRACK").map(Track).collect()
    }
}

pub struct Track<'a>(&'a RElement<'a>);

impl<'a> Track<'a> {
    pub fn name(&'a self) -> &'a str {
        self.0.get_str_attr("NAME")
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

    #[test]
    fn rpp_tracks_test() {
        let input = r#"<REAPER_PROJECT 0.1 "6.42/macOS-arm64" 1640001046
        <TRACK
            NAME "quando una stella"
        >
        <TRACK
            NAME "oltre a queste nubi"
        >
        >"#;
        let element = crate::parser::parse_element::<(_, ErrorKind)>(input)
            .unwrap()
            .1;
        let project = Project(element);
        assert_eq!(project.tracks().len(), 2);
    }
}

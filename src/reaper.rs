use crate::{RElement, RValue};

pub struct Project<'a>(pub RElement<'a>);

impl<'a> Project<'a> {
    pub fn reaper_version(&'a self) -> &'a str {
        self.0.get_str_arg(1)
    }

    pub fn tracks(&'a self) -> Vec<Track<'a>> {
        self.0.children_with_tag("TRACK").map(Track).collect()
    }
}

pub struct Track<'a>(pub &'a RElement<'a>);

impl<'a> Track<'a> {
    pub fn name(&'a self) -> &'a str {
        self.0.get_str_attr("NAME")
    }

    pub fn items(&'a self) -> Vec<Item<'a>> {
        self.0.children_with_tag("ITEM").map(Item).collect()
    }
}

pub struct Item<'a>(pub &'a RElement<'a>);

impl<'a> Item<'a> {
    pub fn len(&'a self) -> f64 {
        self.0.get_num_attr("LENGTH")
    }
}

#[cfg(test)]
mod test {
    use assert_float_eq::*;
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
        assert_eq!(project.tracks()[0].name(), "quando una stella");
        assert_eq!(project.tracks()[1].name(), "oltre a queste nubi");
    }

    #[test]
    fn rpp_items_test() {
        let input = r#"<REAPER_PROJECT 0.1 "6.42/macOS-arm64" 1640001046
        <TRACK
            NAME "quando una stella"
            <ITEM
                LENGTH 5.01
            >
        >
        >"#;
        let element = crate::parser::parse_element::<(_, ErrorKind)>(input)
            .unwrap()
            .1;

        let project = Project(element);
        assert_eq!(project.tracks().len(), 1);
        assert_eq!(project.tracks()[0].items().len(), 1);
        assert_float_relative_eq!(project.tracks()[0].items()[0].len(), 5.01, 0.01);
    }
}

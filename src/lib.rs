#![feature(assert_matches)]
#![feature(iter_intersperse)]

#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::convert::identity;

pub(self) mod parser;
pub(self) mod reaper;

#[derive(Default, Debug, PartialEq)]
pub struct RElement<'a> {
    pub tag: &'a str,
    pub args: Vec<RValue<'a>>,
    pub attributes: HashMap<&'a str, Vec<RValue<'a>>>,
    pub bin_data: Vec<&'a str>,
    pub children: Vec<RElement<'a>>,
    pub fragment_index: Vec<RFragmentId<'a>>,
}

impl<'a> RElement<'a> {
    pub fn get_str_arg(&'a self, index: usize) -> &'a str {
        self.args
            .get(index)
            .and_then(RValue::get_str)
            .unwrap_or_else(|| "")
    }

    pub fn get_str_attr<'b>(&'a self, name: &'b str) -> &'b str
    where
        'a: 'b,
    {
        self.attributes
            .get(name)
            .and_then(|x| x.first())
            .and_then(RValue::get_str)
            .unwrap_or_else(|| "")
    }

    pub fn elements_of_tag<'b>(&'a self, tag: &'b str) -> impl Iterator<Item = &'b RElement<'a>>
    where
        'a: 'b,
    {
        self.children.iter().filter(move |child| child.tag == tag)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RFragmentId<'a> {
    Attribute(&'a str),
    BinData(usize),
    Child(usize),
}

impl<'a> ToString for RElement<'a> {
    fn to_string(&self) -> String {
        self.to_string_with_indent(0)
    }
}

impl<'a> RElement<'a> {
    pub fn to_string_with_indent(&self, indent: usize) -> String {
        let mut rv = String::new();
        let prefix = " ".repeat(indent);
        let inner_prefix = " ".repeat(indent + 1);
        let tag = self.tag;
        let args = Self::value_list_to_string(&self.args);
        let arg_space = if self.args.len() > 0 { " " } else { "" };

        rv.push_str(&format!("{prefix}<{tag}{arg_space}{args}\n"));

        for frag_index in &self.fragment_index {
            match frag_index {
                RFragmentId::Attribute(id) => {
                    if let Some(value_list) = self.attributes.get(id) {
                        let value_list = Self::value_list_to_string(value_list);
                        rv.push_str(&format!("{inner_prefix}{id} {value_list}\n"));
                    }
                }
                RFragmentId::BinData(index) => {
                    if let Some(bin_data) = self.bin_data.get(*index) {
                        rv.push_str(&format!("{inner_prefix}{bin_data}\n"));
                    }
                }
                RFragmentId::Child(index) => {
                    if let Some(child) = self.children.get(*index) {
                        rv.push_str(&child.to_string_with_indent(indent + 1));
                    }
                }
            }
        }

        rv.push_str(&format!("{prefix}>\n"));
        rv
    }

    fn value_list_to_string(values: &Vec<RValue>) -> String {
        values
            .iter()
            .map(ToString::to_string)
            .intersperse_with(|| " ".to_owned())
            .collect()
    }
}

pub(crate) enum RElementFragment<'a> {
    Child(RElement<'a>),
    Attribute(&'a str, Vec<RValue<'a>>),
    BinData(&'a str),
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum RValue<'a> {
    /// Quoted String
    QS(String),
    /// Unquoted String
    S(&'a str),
    /// Integer
    N(f64),
}

impl<'a> RValue<'a> {
    pub fn get_str(&'a self) -> Option<&'a str> {
        match self {
            RValue::QS(s) => Some(s.as_str()),
            RValue::S(s) => Some(s),
            RValue::N(_) => None,
        }
    }
}

impl<'a> ToString for RValue<'a> {
    fn to_string(&self) -> String {
        match self {
            RValue::QS(value) => {
                let escape_seq = |c: char| ['\\', c].map(Some);
                let value = value
                    .chars()
                    .flat_map(|c| match c {
                        '"' => escape_seq('"'),
                        '\t' => escape_seq('t'),
                        '\n' => escape_seq('n'),
                        '\\' => escape_seq('\\'),
                        c => [Some(c), None],
                    })
                    .filter_map(identity)
                    .collect::<String>();
                format!("\"{value}\"")
            }
            RValue::S(s) => s.to_string(),
            RValue::N(n) => n.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_element_str() {
        let v = RValue::S("foo");
        assert_eq!(v.get_str(), Some("foo"));

        let v = RValue::N(5.0);
        assert_eq!(v.get_str(), None);
    }

    #[test]
    fn test_arg_not_exists() {
        let v = RElement {
            tag: "PROJECT",
            args: vec![],
            attributes: Default::default(),
            bin_data: vec![],
            children: vec![],
            fragment_index: vec![],
        };

        assert_eq!(v.get_str_arg(0), "");
    }
}

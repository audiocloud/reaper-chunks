#![feature(assert_matches)]
#![feature(iter_intersperse)]
#[macro_use]
extern crate nom;

use std::collections::HashMap;

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

impl<'a> ToString for RValue<'a> {
    fn to_string(&self) -> String {
        match self {
            RValue::QS(value) => format!("\"{value}\""),
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
    fn test_simple_chunk() {}
}

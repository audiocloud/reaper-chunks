#[cfg(test)]
#[macro_use]
extern crate assert_matches;
#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::convert::identity;

use nom::combinator::value;

pub use parser::parse_element;
pub use reaper::Project;

pub(self) mod parser;
pub(self) mod reaper;

#[derive(Debug, PartialEq)]
pub enum RFragment<'a> {
  Attribute(&'a str, Vec<RValue<'a>>),
  Child(RElement<'a>),
  BinData(String),
  Empty,
}

pub(crate) fn is_fragment_attribute<'a, 'b>(name: &'b str) -> impl Fn(&'a RFragment<'a>) -> Option<&'a Vec<RValue<'a>>> + 'b
  where
    'a: 'b,
{
  move |frag| match frag {
    RFragment::Attribute(attrib_name, values) if *attrib_name == name => Some(values),
    _ => None,
  }
}

pub(crate) fn is_child_tag<'a, 'b>(tag: &'b str) -> impl Fn(&'a RFragment<'a>) -> Option<&'a RElement<'a>> + 'b
  where
    'a: 'b,
{
  move |frag| match frag {
    RFragment::Child(c) if c.tag == tag => Some(c),
    _ => None,
  }
}

#[derive(Default, Debug, PartialEq)]
pub struct RElement<'a> {
  pub tag: &'a str,
  pub args: Vec<RValue<'a>>,
  pub content: Vec<RFragment<'a>>,
}

impl<'a> RElement<'a> {
  pub fn append_attribute(&mut self, name: &'a str, values: Vec<RValue<'a>>) {
    self.content.push(RFragment::Attribute(name, values));
  }

  pub fn append_bin_data(&mut self, data: String) {
    self.content.push(RFragment::BinData(data));
  }

  pub fn append_child(&mut self, child: RElement<'a>) {
    self.content.push(RFragment::Child(child));
  }

  pub fn get_str_arg(&'a self, index: usize) -> Option<&'a str> {
    self.args.get(index).and_then(RValue::get_str)
  }

  pub fn get_str_attr<'b>(&'a self, name: &'b str, index: usize) -> Option<&'b str>
    where
      'a: 'b,
  {
    self.content
      .iter()
      .filter_map(is_fragment_attribute(name))
      .nth(index)
      .and_then(|x| x.first())
      .and_then(RValue::get_str)
  }

  pub fn get_num_attr(&'a self, name: &'a str, index: usize) -> Option<f64> {
    self.content
      .iter()
      .filter_map(is_fragment_attribute(name))
      .nth(index)
      .and_then(|x| x.first())
      .and_then(RValue::get_num)
  }

  pub fn children_with_tag<'b>(&'a self, tag: &'b str) -> impl Iterator<Item=&'a RElement<'a>> + 'b
    where
      'a: 'b,
  {
    self.content.iter().filter_map(is_child_tag(tag))
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

    rv.push_str(&format!(
      "{prefix}<{tag}{arg_space}{args}\n",
      prefix = &prefix,
      tag = &tag,
      arg_space = &arg_space,
      args = &args
    ));

    for frag in &self.content {
      match frag {
        RFragment::Attribute(id, value_list) => {
          let value_list = Self::value_list_to_string(value_list);
          rv.push_str(&format!(
            "{inner_prefix}{id} {value_list}\n",
            inner_prefix = &inner_prefix,
            id = &id,
            value_list = &value_list
          ));
        }
        RFragment::BinData(bin_data) => {
          rv.push_str(&format!(
            "{inner_prefix}{bin_data}\n",
            inner_prefix = &inner_prefix,
            bin_data = &bin_data
          ));
        }
        RFragment::Child(child) => {
          rv.push_str(&child.to_string_with_indent(indent + 1));
        }
        RFragment::Empty => rv.push_str("\n"),
      }
    }

    rv.push_str(&format!("{prefix}>\n", prefix = &prefix));
    rv
  }

  fn value_list_to_string(values: &Vec<RValue>) -> String {
    let mut rv = String::new();
    for (i, value) in values.iter().enumerate() {
      if i != 0 {
        rv.push_str(" ");
      }
      rv.push_str(value.to_string().as_str());
    }
    rv
  }
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

pub struct RValues;

impl RValues {
  pub fn floats<'a, I: IntoIterator<Item=f64>>(values: I) -> Vec<RValue<'a>> {
    let mut rv = vec![];
    for v in values {
      rv.push(RValue::N(v));
    }
    rv
  }

  pub fn ints<'a, I: IntoIterator<Item=i64>>(values: I) -> Vec<RValue<'a>> {
    Self::floats(values.into_iter().map(|i| i as f64))
  }

  pub fn bools<'a, I: IntoIterator<Item=bool>>(values: I) -> Vec<RValue<'a>> {
    Self::ints(values.into_iter().map(|x| if x { 1 } else { 0 }))
  }

  pub fn strings<'a, I: IntoIterator<Item=&'a str>>(values: I) -> Vec<RValue<'a>> {
    let mut rv = vec![];
    for v in values {
      rv.push(RValue::S(v.as_ref()));
    }
    rv
  }

  pub fn quoted_strings<'a, I: IntoIterator<Item=&'a str>>(values: I) -> Vec<RValue<'a>> {
    let mut rv = vec![];
    for v in values {
      rv.push(RValue::QS(v.to_owned()));
    }
    rv
  }

  pub fn int<'a>(i: i64) -> Vec<RValue<'a>> {
    Self::ints([i])
  }

  pub fn float<'a>(f: f64) -> Vec<RValue<'a>> {
    Self::floats([f])
  }

  pub fn bool<'a>(b: bool) -> Vec<RValue<'a>> {
    Self::bools([b])
  }

  pub fn string(s: &str) -> Vec<RValue> {
    Self::strings([s])
  }

  pub fn quoted_string(s: &str) -> Vec<RValue> {
    Self::quoted_strings([s])
  }
}

impl<'a> RValue<'a> {
  pub fn get_str(&'a self) -> Option<&'a str> {
    match self {
      RValue::QS(s) => Some(s.as_str()),
      RValue::S(s) => Some(s),
      RValue::N(_) => None,
    }
  }

  pub fn get_num(&'a self) -> Option<f64> {
    match self {
      RValue::N(f) => Some(*f),
      _ => None,
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
        format!("\"{value}\"", value = &value)
      }
      RValue::S(s) => s.to_string(),
      RValue::N(n) => n.to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
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
      content: vec![],
    };

    assert_eq!(v.get_str_arg(0), None);
    assert_eq!(v.get_num_attr("FOO", 0), None);
  }

  #[test]
  fn test_arg_wrong_type() {
    let v = RElement {
      tag: "PROJECT",
      args: vec![RValue::N(0.5)],
      content: vec![RFragment::Attribute("FOO", vec![RValue::S("test")])],
    };

    assert_eq!(v.get_str_arg(0), None);
    assert_eq!(v.get_num_attr("FOO", 0), None);
  }
}

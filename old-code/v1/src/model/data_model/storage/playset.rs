use super::{ruleset::Ruleset, setting::Setting};

#[derive(Debug, PartialEq, Clone)]
pub struct Playset<'a> {
    ruleset: Option<&'a Ruleset>,
    setting: Option<&'a Setting>,
}

impl Playset<'_> {
    pub fn from_ruleset<'a>(r: &'a Ruleset) -> Playset<'a> {
        Playset { ruleset: Some(r), setting: None }
    }

    pub fn from_setting<'a>(s: &'a Setting) -> Playset<'a> {
        Playset { ruleset: None, setting: Some(s) }
    }

    pub fn new<'a>(r: Option<&'a Ruleset>, s: Option<&'a Setting>) -> Playset<'a> {
        Playset { ruleset: r, setting: s }
    }
}
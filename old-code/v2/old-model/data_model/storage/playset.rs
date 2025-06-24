use super::{ruleset::Ruleset, setting::Setting};

#[derive(Debug, PartialEq, Clone)]
pub struct Playset<'a, 'b> {
    ruleset: Option<&'a Ruleset<'b>>,
    setting: Option<&'a Setting<'b>>,
}

impl Playset<'_, '_> {
    pub fn from_ruleset<'a, 'b>(r: &'a Ruleset<'b>) -> Playset<'a, 'b> {
        Playset { ruleset: Some(r), setting: None }
    }

    pub fn from_setting<'a, 'b>(s: &'a Setting<'b>) -> Playset<'a, 'b> {
        Playset { ruleset: None, setting: Some(s) }
    }

    pub fn new<'a, 'b>(r: Option<&'a Ruleset<'b>>, s: Option<&'a Setting<'b>>) -> Playset<'a, 'b> {
        Playset { ruleset: r, setting: s }
    }
}
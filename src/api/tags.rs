use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use std::ops::Index;

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Clone, Hash)]
pub struct Tag
{
    name: String,
}

impl Tag
{
    /// Parse a string into the according tag.
    /// 
    /// Ex:
    /// Ability.Magic Theory -> Ability.Magic Theory
    /// Ability. Magic Theory . Speciality -> Ability.Magic Theory.Speciality
    /// 
    /// A tag can only contain alpha-numeric characters and
    /// the first sub-tag must contain at least one non-numeric
    /// character. Each subtag must also contain at least one character.
    /// 
    /// Ex:
    /// Ability.0 -> OK
    /// 0.Ability -> NOT OK
    /// Ability.Magic Theory!.Speciality -> NOT OK
    pub fn from_str(s: &str) -> Tag
    {
        // TODO:
        // Properly parse the string to convert it to a tag.
        // Return error accordingly

        // Parsing tag procedure:
        // Check that the string only contains alpha-numeric values or '.'s
        // Split by '.'
        // Ensure first sub-string is not just a number
        // Initialize empty result string
        // Loop through each substring s
        //      Trim the outer white-space
        //      Add to result string
        //      Add '.' (if we are not the last value)
        Tag
        {
            name: s.to_string(),
        }
    }

    pub fn to_str(&self) -> &str
    {
        &self.name
    }

    /// Given a tag, splits the literals into 
    /// the sub-tag array. This is a help method
    /// used by TagContainer to add and remove tags.
    /// Ex:
    /// Ability.Magic Theory -> ["Ability", "Ability.Magic Theory"]
    /// Ability.Magic Theory.Speciality -> ["Ability", "Ability.Magic Theory", "Ability.Magic Theory.Speciality"]
    fn split_to_subtags(&self) -> Vec<String>
    {
        todo!()
    }
}

/// Contains tags, which are string literals delinitated by '.'s
/// 
/// A tag can be made of smaller sub-tags, which are children to the
/// greater tag. For example, Ability.Magic Theory as a tag has
/// Ability as the first sub-tag and Magic Theory as the second.
/// 
/// The presence of Ability.Magic Theory 
#[derive()]
pub struct TagSet
{
    tags: HashMap<String, i32>,
}

impl TagSet
{
    pub fn new() -> TagSet
    {
        TagSet { tags: HashMap::new() }
    }

    pub fn count_tag(&self, t: &Tag) -> i32
    {
        self[&t.to_str()]
    }

    pub fn add_tag_count(&mut self, t: &Tag, c: i32)
    {
        for st in t.split_to_subtags()
        {
            let v = self[&st];
            self.tags.insert(st.clone(), v + c);
        }
    }

    pub fn remove_tag_count(&mut self, t: &Tag, c: i32)
    {
        self.add_tag_count(t, -c);
    }

    pub fn has_tag(&self, t: &Tag) -> bool
    {
        self.count_tag(t) > 0
    }

    pub fn add_tag(&mut self, t: &Tag)
    {
        self.add_tag_count(t, 1);
    }

    pub fn remove_tag(&mut self, t: &Tag)
    {
        self.remove_tag_count(t, 1);
    }
}

impl Index<&str> for TagSet
{
    type Output = i32;
 
    #[inline]
    fn index(&self, index: &str) -> &Self::Output {
        self.tags.get(index).unwrap_or(&0)
    }
}
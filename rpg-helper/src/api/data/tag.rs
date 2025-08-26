use std::{collections::{HashMap, HashSet}, fmt::Display};

use serde::{Deserialize, Serialize};

use std::ops::Index;

use crate::api::data::{error::{ParseError, ParseErrorType, TagParseError, TemplateError}, template::Template};

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Clone, Hash)]
pub struct Tag
{
    name: String,
}

impl Tag
{
    /// Alias for from_str(s)
    pub fn new(s: &str) -> Result<Tag, ParseError>
    {
        Self::from_str(s)
    }

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
    pub fn from_str(s: &str) -> Result<Tag, ParseError>
    {
        // Initial error check to ensure not empty
        if s.is_empty() || s.chars().all(char::is_whitespace)
        {
            return Err(ParseError::new(s.to_string(), s.len(), ParseErrorType::Tag(TagParseError::TagEmpty)));
        }

        // Check that the string only contains alpha-numeric values or '.'s
        if !s.chars().all(|c| Self::is_valid_tag_char(c))
        {
            return Err(ParseError::new(s.to_string(), s.find(|c| !Self::is_valid_tag_char(c)).unwrap(), ParseErrorType::Tag(TagParseError::InvalidCharacter)));
        }

        // Ensure first sub-string is not just a number
        let first_str = if let Some(f) = s.split('.').next()
        {
            f
        }
        else
        {
            s
        };

        if first_str.chars().all(|c| c.is_numeric() || c.is_whitespace())
        {
            return Err(ParseError::new(s.to_string(), s.len() - 1, ParseErrorType::Tag(TagParseError::FirstTagNumeric)));
        }

        // Initialize empty result string
        // Loop through each substring s
        //      Trim the outer white-space
        //      Add to result string
        //      Add '.' (if we are not the last value)
        let mut name = String::new();
        let mut it = s.split('.').peekable();
        while let Some(sub) = it.next()
        {
            if sub.chars().all(char::is_whitespace)
            {
                return Err(ParseError::new(s.to_string(), s.find(sub).unwrap(), ParseErrorType::Tag(TagParseError::SubTagEmpty)))
            }

            name.push_str(sub.trim());
            if it.peek().is_some()
            {
                name.push('.');
            }
        }

        Ok(Tag { name })
    }

    pub fn to_str(&self) -> &str
    {
        &self.name
    }

    /// Removes the prefix of the tag up to the
    /// matching given prefix.
    /// 
    /// Returns None if no match is found
    /// Ex:
    ///     remove_prefix(ability.spell.Name Of Spell, ability.spell)     -> Name Of Spell
    ///     remove_prefix(ability.spell.Name Of Spell.Exp, ability.spell) -> Name Of Spell.Exp
    pub fn remove_prefix(&self, prefix: &Tag) -> Option<Tag>
    {
        if self.name.starts_with(&prefix.name)
        {
            // Calculate where to split: prefix length + potential dot
            let prefix_len = prefix.name.len();
            
            if prefix_len >= self.name.len()
            {
                return None;
            }
            
            // Check if there's a dot after the prefix
            let remaining = if self.name.chars().nth(prefix_len) == Some('.')
            {
                &self.name[prefix_len + 1..]
            }
            else
            {
                return None;
            };
            
            if remaining.is_empty()
            {
                None
            }
            else
            {
                Some(Tag { name: remaining.to_string() })
            }
        }
        else
        {
            None
        }
    }

    pub fn remove_prefix_by_count(&self, count: usize) -> Option<Tag>
    {
        let mut iter = self.name.split('.').peekable();
        for _ in 0..count
        {
            iter.next();
        }

        if iter.peek().is_none()
        {
            return None;
        }
        else
        {
            let mut result_string = String::new();
            while let Some(st) = iter.next()
            {
                result_string.push_str(&st);
                if iter.peek().is_some()
                {
                    result_string.push('.');
                }
            }
            Some(Tag { name: result_string })
        }
    }
    
    /// Prefix a tag with another prefix
    /// Ex:
    ///     add_prefix(Name Of Spell, ability.spell) -> ability.spell.Name Of Spell
    pub fn add_prefix(&self, prefix: &Tag) -> Tag
    {
        let mut final_string = prefix.name.clone();
        final_string.push('.');
        final_string.push_str(&self.name);
        Tag { name: final_string }
    }

    pub fn has_prefix(&self, prefix: &str) -> bool
    {
        if self.name.len() > prefix.len()
        {
            self.name.starts_with(prefix) && self.name.chars().nth(prefix.len()) == Some('.') 
        }
        else
        {
            self.name.starts_with(prefix)
        }
    }

    /// Suffix a tag with another suffix
    /// Ex:
    ///     "Name Of Spell".add_suffix("Burning Hands") -> "Name Of Spell.Burning Hands"
    pub fn add_suffix(&self, suffix: &Tag) -> Tag
    {
        let mut final_string = self.name.clone();
        final_string.push('.');
        final_string.push_str(&suffix.name);
        Tag { name: final_string }
    }

    pub fn has_suffix(&self, suffix: &str) -> bool
    {
        if self.name.len() > suffix.len()
        {
            if let Some(i) = self.name.find(suffix)
            {
                self.name.ends_with(suffix) && self.name.chars().nth(i) == Some('.')
            }
            else
            {
                false
            }
        }
        else
        {
            self.name.ends_with(suffix)
        }
    }

    /// Removes all sub-tags in a tag.
    /// Ex:
    ///     ability.spell.Name Of Spell.Exp -> ability
    ///     Name Of Spell.Exp               -> Name Of Spell
    /// 
    pub fn no_subtags(&self) -> Tag
    {
        if let Some(f) = self.name.split('.').next()
        {
            Tag { name: f.to_string() }
        }
        else
        {
            self.clone()
        }
    }

    /// Returns the number of sub-tags in the tag
    /// Ex:
    ///     ability.spell.Name Of Spell -> 2
    ///     ability                     -> 0
    pub fn count_subtags(&self) -> usize
    {
        self.name.split('.').count() - 1
    }

    pub fn find_all_parse_errors(s: &str) -> Result<(), Vec<ParseError>>
    {
        let mut res = vec![];
        if s.is_empty() || s.chars().all(char::is_whitespace)
        {
            res.push(ParseError::new(s.to_string(), s.len(), ParseErrorType::Tag(TagParseError::TagEmpty)));
            return Err(res);
        }

        let first_str = if s.contains('.')
        {
            s.split('.').next().unwrap()
        }
        else
        {
            s
        };

        if first_str.chars().all(char::is_numeric)
        {
            res.push(ParseError::new(s.to_string(), s.len() - 1, ParseErrorType::Tag(TagParseError::FirstTagNumeric)));
        }

        for (i, c) in s.chars().enumerate()
        {
            if !Self::is_valid_tag_char(c)
            {
                res.push(ParseError::new(s.to_string(), i, ParseErrorType::Tag(TagParseError::InvalidCharacter)));
            }
        }

        for sub in s.split('.')
        {
            if sub.chars().all(char::is_whitespace)
            {
                res.push(ParseError::new(s.to_string(), s.find(sub).unwrap(), ParseErrorType::Tag(TagParseError::SubTagEmpty)));
            }
        }

        if res.is_empty()
        {
            Ok(())
        }
        else
        {
            Err(res)
        }
    }

    fn is_valid_tag_char(c: char) -> bool
    {
        c.is_alphanumeric() || c == '.' || c.is_whitespace()
    }


    /// Given a tag, splits the literals into 
    /// the sub-tag array. This is a help method
    /// used by TagContainer to add and remove tags.
    /// Ex:
    /// Ability.Magic Theory -> ["Ability", "Ability.Magic Theory"]
    /// Ability.Magic Theory.Speciality -> ["Ability", "Ability.Magic Theory", "Ability.Magic Theory.Speciality"]
    fn split_to_subtags(&self) -> Vec<String>
    {
        let mut res = vec![];
        if self.name.contains('.')
        {
            let mut cur = String::new();

            let mut it = self.name.split('.').peekable();
            while let Some(sub) = it.next()
            {
                cur.push_str(sub);
                res.push(cur.to_string());
                if it.peek().is_some()
                {
                    cur.push('.');
                }
            }
        }
        else
        {
            res.push(self.name.to_string());
        }
        res
    }
}

impl Display for Tag
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Contains tags, which are string literals delinitated by '.'s
/// 
/// A tag can be made of smaller sub-tags, which are children to the
/// greater tag. For example, Ability.Magic Theory as a tag has
/// Ability as the first sub-tag and Magic Theory as the second.
/// 
/// The presence of Ability.Magic Theory 
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct TagSet
{
    primary_tags: HashMap<Tag, i32>,
    tags: HashMap<String, i32>,
}

impl TagSet
{
    pub fn new() -> TagSet
    {
        TagSet { primary_tags: HashMap::new(), tags: HashMap::new() }
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
        self.primary_tags.insert(t.clone(), c + self.primary_tags.get(t).unwrap_or(&0));
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

    /// Given a tag prefix, this method returns all
    /// subtags which exist in this tag set (the count of the tag must be > 0)
    /// Example:  get_subtags(ability) -> [ability.Magic Theory, ability.Magic Theory.Exp, ability.Latin, ability.Latin.Exp]
    pub fn get_matching_prefix(&self, prefix: &Tag) -> Vec<Tag>
    {
        self.tags.clone().into_iter().filter_map(|(t, c)| 
        {
            let t = Tag { name: t };
            if c > 0 && t.has_prefix(&prefix.name)
            {
                Some(t)
            }
            else
            {
                None
            }
        }).collect()
    }

    /// Given a tag prefix, this method returns all
    /// immediate tags which exist in this tag set (the count of the tag must be > 0)
    /// Example:  get_subtags(ability) -> [ability.Magic Theory, ability.Latin]
    ///           but does not return [ability.Magic Theory.Exp, ability.Latin.Exp]
    pub fn get_immediate_matching_prefix(&self, prefix: &Tag) -> Vec<Tag>
    {
        let num_subtags = prefix.count_subtags() + 1;
        self.tags.clone().into_iter().filter_map(|(t, c)|
        {
            let t = Tag { name: t };
            if c > 0 && t.count_subtags() == num_subtags && t.has_prefix(&prefix.name) 
            {
                Some(t)
            }
            else
            {
                None
            }
        }
        ).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = Tag> + '_
    {
        self.tags.keys().map(| s| Tag::from_str(s).unwrap()).into_iter()
    }

    pub fn iter_primary_tags(&self) -> impl Iterator<Item = (&Tag, &i32)> + '_
    {
        self.primary_tags.iter()
    }

    /// Current implementation is to just add the tag counts to the resulting
    /// final tag set.
    pub fn layer(&self, other: &Self) -> Self
    {
        let mut res = Self::new();
        for (tag, count) in self.primary_tags.iter().chain(other.primary_tags.iter())
        {
            res.add_tag_count(tag, *count);
        }
        res
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

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Clone, Hash)]
pub struct TagTemplate
{
    decomposed_tag: Vec<TagTemplateSubtag>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Clone, Hash)]
enum TagTemplateSubtag
{
    Literal(String),
    Subtag(String),
}

impl TagTemplate
{
    /// Given a string, constructs a TagTemplate
    /// Expects to find at least one template value, indicated by a subtag surrounded
    /// by "[]". Also expects each template identifier to be unique
    /// 
    /// Ex:
    ///     "tag.test.[template].value"
    pub fn from_str(s: &str) -> Result<TagTemplate, ParseError>
    {
        // Initial error check to ensure not empty
        if s.is_empty() || s.chars().all(char::is_whitespace)
        {
            return Err(ParseError::new(s.to_string(), s.len(), ParseErrorType::Tag(TagParseError::TagEmpty)));
        }

        // Check that the string only contains alpha-numeric values or '.'s
        if !s.chars().all(|c| Self::is_valid_tag_char(c))
        {
            return Err(ParseError::new(s.to_string(), s.find(|c| !Self::is_valid_tag_char(c)).unwrap(), ParseErrorType::Tag(TagParseError::InvalidCharacter)));
        }

        // Ensure first sub-string is not just a number
        let first_str = if let Some(f) = s.split('.').next()
        {
            f
        }
        else
        {
            s
        };

        if first_str.chars().all(|c| c.is_numeric() || c.is_whitespace())
        {
            return Err(ParseError::new(s.to_string(), s.len() - 1, ParseErrorType::Tag(TagParseError::FirstTagNumeric)));
        }

        let mut decomposed_tag = vec![];

        let mut it = s.split('.').peekable();
        while let Some(sub) = it.next()
        {
            if sub.chars().all(char::is_whitespace)
            {
                return Err(ParseError::new(s.to_string(), s.find(sub).unwrap(), ParseErrorType::Tag(TagParseError::SubTagEmpty)));
            }

            let sub = sub.trim();
            if let Some(first_char) = sub.chars().next()
            {
                let is_literal;
                let check;
                if first_char == '['
                {
                    if let Some(last_char) = sub.chars().next_back()
                    {
                        if last_char == ']'
                        {
                            check = &sub[1..sub.len()-1];
                            is_literal = false;
                        }
                        else
                        {
                            return Err(ParseError::new(s.to_string(), s.find(sub).unwrap_or(0), ParseErrorType::Tag(TagParseError::InvalidCharacter)));
                        }
                    }
                    else
                    {
                        return Err(ParseError::new(s.to_string(), s.find(sub).unwrap_or(0), ParseErrorType::Tag(TagParseError::SubTagEmpty)));
                    }
                }
                else
                {
                    check = sub;
                    is_literal = true;
                }

                if check.contains(|c| !Tag::is_valid_tag_char(c))
                {
                    return Err(ParseError::new(s.to_string(), s.find(sub).unwrap_or(0), ParseErrorType::Tag(TagParseError::InvalidCharacter)))
                }

                if is_literal
                {
                    decomposed_tag.push(TagTemplateSubtag::Literal(sub.to_string()));
                }
                else
                {
                    decomposed_tag.push(TagTemplateSubtag::Subtag(check.to_string()));
                }
            }
            else
            {
                return Err(ParseError::new(s.to_string(), s.find(sub).unwrap(), ParseErrorType::Tag(TagParseError::SubTagEmpty)))
            }
        }

        Ok(TagTemplate { decomposed_tag })
    }

    pub fn into_tag(&self) -> Result<Tag, TemplateError>
    {
        self.attempt_complete()
    }

    fn is_valid_tag_char(c: char) -> bool
    {
        Tag::is_valid_tag_char(c) || c == '[' || c == ']'
    }
}

impl Template<Tag> for TagTemplate
{
    fn get_required_inputs(&self) -> HashSet<String>
    {
        self.decomposed_tag.clone().into_iter().filter_map(|st|
            if let TagTemplateSubtag::Subtag(s) = st
            {
                Some(s)
            }
            else
            {
                None
            }
        ).collect()
    }

    /// Inserts a template value into the tag template.
    /// This method does nothing if the provided input is not contained
    /// in the template.
    /// 
    /// If the template value provided was the last required input,
    /// the tag created is returned.
    fn insert_template_value(&mut self, input_name: &str, input_value: &Tag) -> Option<Tag>
    {
        self.decomposed_tag = self.decomposed_tag.clone().into_iter().map(|t|
            match &t
            {
                TagTemplateSubtag::Literal(_) => t,
                TagTemplateSubtag::Subtag(s) =>
                if s == input_name
                {
                    TagTemplateSubtag::Literal(input_value.name.to_string())
                }
                else
                {
                    t
                },
            }
        ).collect();

        match self.attempt_complete()
        {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }

    /// Inserts the mapping of templated subtags to tag values.
    /// Expected only to fail if there is a template subtag missing a tag value.
    fn attempt_complete(&self) -> Result<Tag, TemplateError>
    {
        let full_subtags = self.decomposed_tag.iter().map(|e|
            match e
            {
                TagTemplateSubtag::Literal(l) => Ok(l.to_string()),
                TagTemplateSubtag::Subtag(s) => Err(s.to_string()),
            }
        );

        if full_subtags.clone().any(|st| st.is_err())
        {
            return Err(TemplateError::MissingTemplateValues(full_subtags.filter_map(|e| 
                if let Err(e) = e
                {
                    Some(e)
                }
                else
                {
                    None
                }
            ).collect()));
        }
        else
        {
            let mut full_subtags = full_subtags.filter_map(|st|
            if let Ok(st) = st {
                Some(st)
            }
            else
            {
                None   
            }).peekable();
            let mut result_string = String::new();
            while let Some(st) = full_subtags.next()
            {
                result_string.push_str(&st);
                if full_subtags.peek().is_some()
                {
                    result_string.push('.');
                }
            }
            Ok(Tag { name: result_string })
        }
    }
}

#[cfg(test)]
mod unit_tests 
{
    use super::*;

    /// Tests the creation of a single word tag.
    /// Expected to succeed in creation.
    #[test]
    fn parse_test_1()
    {
        match Tag::from_str("First")
        {
            Ok(t) => assert_eq!(t.name, "First"),
            Err(e) => panic!("Tag failed with error {:?}", e),
        }
    }

    /// Tests the creation  of a multi-word tag with spaces.
    /// Expected to succeed in creation.
    #[test]
    fn parse_test_2()
    {
        match Tag::from_str("First Tag")
        {
            Ok(t) => assert_eq!(t.name, "First Tag"),
            Err(e) => panic!("Tag failed with error {:?}", e),
        }
    }

    /// Tests the creation a single word tag with leading and trailing spaces.
    /// Expected to succeed in creation and match result.
    #[test]
    fn parse_test_3()
    {
        match Tag::from_str(" First ")
        {
            Ok(t) => assert_eq!(t.name, "First"),
            Err(e) => panic!("Tag failed with error {:?}", e),
        }
    }

    /// Tests the creation a multi-word tag with leading and trailing spaces.
    /// Expected to succeed in creation and match result.
    #[test]
    fn parse_test_4()
    {
        match Tag::from_str(" First Tag ")
        {
            Ok(t) => assert_eq!(t.name, "First Tag"),
            Err(e) => panic!("Tag failed with error {:?}", e),
        }
    }

    /// Tests the creation of subtags that are single words.
    /// Expected to succeed in creation and match result.
    #[test]
    fn parse_test_5()
    {
        match Tag::from_str("One.Two")
        {
            Ok(t) => assert_eq!(t.name, "One.Two"),
            Err(e) => panic!("Tag failed with error {:?}", e),
        }
    }

    /// Tests the creation of subtags that are single words with leading and trailing spaces.
    /// Expected to succeed in creation and match result.
    #[test]
    fn parse_test_6()
    {
        match Tag::from_str(" One . Two ")
        {
            Ok(t) => assert_eq!(t.name, "One.Two"),
            Err(e) => panic!("Tag failed with error {:?}", e),
        }
    }

    /// Tests the creation of subtags that are multiple words.
    /// Expected to succeed in creation and match result.
    #[test]
    fn parse_test_7()
    {
        match Tag::from_str("Multi Worded.Tag Test")
        {
            Ok(t) => assert_eq!(t.name, "Multi Worded.Tag Test"),
            Err(e) => panic!("Tag failed with error {:?}", e),
        }
    }

    /// Tests the creation of subtags that are multiple words with leading and trailing whitespace.
    /// Expected to succeed in creation and match result.
    #[test]
    fn parse_test_8()
    {
        match Tag::from_str(" Multi Worded . Tag Test ")
        {
            Ok(t) => assert_eq!(t.name, "Multi Worded.Tag Test"),
            Err(e) => panic!("Tag failed with error {:?}", e),
        }
    }

    /// Tests the parsing of tags with an empty string.
    /// Expected to fail with an "Empty Tag" error
    #[test]
    fn parse_test_9()
    {
        match Tag::from_str("")
        {
            Ok(t) => panic!("Succeeded in creating empty tag: \'{}\'", t.name),
            Err(e) => 
            {
                if let ParseErrorType::Tag(t) = e.error_type
                {
                    assert_eq!(t, TagParseError::TagEmpty);
                }
                else
                {
                    panic!("Parse error is not a tag error: {:?}", e);
                }
            },
        }
    }

    /// Tests the parsing of tags with an only-space string.
    /// Expected to fail with an "Empty Tag" error
    #[test]
    fn parse_test_10()
    {
        match Tag::from_str("  ")
        {
            Ok(t) => panic!("Succeeded in creating empty tag: \'{}\'", t.name),
            Err(e) => 
            {
                if let ParseErrorType::Tag(t) = e.error_type
                {
                    assert_eq!(t, TagParseError::TagEmpty);
                }
                else
                {
                    panic!("Parse error is not a tag error: {:?}", e);
                }
            },
        }
    }

    /// Tests the parsing of tags with an empty subtag 
    /// Expected to fail with an "Empty Sub Tag" error
    #[test]
    fn parse_test_11()
    {
        match Tag::from_str("Empty..Tag")
        {
            Ok(t) => panic!("Succeeded in creating empty sub tag: \'{}\'", t.name),
            Err(e) => 
            {
                if let ParseErrorType::Tag(t) = e.error_type
                {
                    assert_eq!(t, TagParseError::SubTagEmpty);
                }
                else
                {
                    panic!("Parse error is not a tag error: {:?}", e);
                }
            },
        }
    }

    /// Tests the parsing of tags with an empty spaced subtag 
    /// Expected to fail with an "Empty Sub Tag" error
    #[test]
    fn parse_test_12()
    {
        match Tag::from_str("Empty.   .Tag")
        {
            Ok(t) => panic!("Succeeded in creating empty sub tag: \'{}\'", t.name),
            Err(e) => 
            {
                if let ParseErrorType::Tag(t) = e.error_type
                {
                    assert_eq!(t, TagParseError::SubTagEmpty);
                }
                else
                {
                    panic!("Parse error is not a tag error: {:?}", e);
                }
            },
        }
    }

    /// Tests the parsing of tags with a only numeric first tag.
    /// Expected to fail with a "Only Numeric" first tag error
    #[test]
    fn parse_test_13()
    {
        match Tag::from_str("10.tag")
        {
            Ok(t) => panic!("Succeeded in creating numeric first tag: \'{}\'", t.name),
            Err(e) => 
            {
                if let ParseErrorType::Tag(t) = e.error_type
                {
                    assert_eq!(t, TagParseError::FirstTagNumeric);
                }
                else
                {
                    panic!("Parse error is not a tag error: {:?}", e);
                }
            },
        }
    }

    /// Tests the parsing of tags with a only numeric first tag with leading and trailing whitespace.
    /// Expected to fail with a "Only Numeric" first tag error
    #[test]
    fn parse_test_14()
    {
        match Tag::from_str(" 10 .tag")
        {
            Ok(t) => panic!("Succeeded in creating numeric first tag: \'{}\'", t.name),
            Err(e) => 
            {
                if let ParseErrorType::Tag(t) = e.error_type
                {
                    assert_eq!(t, TagParseError::FirstTagNumeric);
                }
                else
                {
                    panic!("Parse error is not a tag error: {:?}", e);
                }
            },
        }
    }

    /// Tests the parsing of tags with invalid character '&'
    /// Expected to fail with invalid character error
    #[test]
    fn parse_test_15()
    {
        match Tag::from_str("Death&Taxes")
        {
            Ok(t) => panic!("Succeeded in creating invalid character tag: \'{}\'", t.name),
            Err(e) => 
            {
                if let ParseErrorType::Tag(t) = e.error_type
                {
                    assert_eq!(t, TagParseError::InvalidCharacter);
                }
                else
                {
                    panic!("Parse error is not a tag error: {:?}", e);
                }
            },
        }
    }

    /// Tests the parsing of tags with invalid character '#'
    /// Expected to fail with invalid character error
    #[test]
    fn parse_test_16()
    {
        match Tag::from_str("#TagStyle")
        {
            Ok(t) => panic!("Succeeded in creating invalid character tag: \'{}\'", t.name),
            Err(e) => 
            {
                if let ParseErrorType::Tag(t) = e.error_type
                {
                    assert_eq!(t, TagParseError::InvalidCharacter);
                }
                else
                {
                    panic!("Parse error is not a tag error: {:?}", e);
                }
            },
        }
    }

    /// Tests the parsing of tags with invalid character '-'
    /// Expected to fail with invalid character error
    #[test]
    fn parse_test_17()
    {
        match Tag::from_str("Tag-Style")
        {
            Ok(t) => panic!("Succeeded in creating invalid character tag: \'{}\'", t.name),
            Err(e) => 
            {
                if let ParseErrorType::Tag(t) = e.error_type
                {
                    assert_eq!(t, TagParseError::InvalidCharacter);
                }
                else
                {
                    panic!("Parse error is not a tag error: {:?}", e);
                }
            },
        }
    }

    /// Tests parsing for all errors.
    /// Expected to not find any errors.
    #[test]
    fn parse_test_18()
    {
        match Tag::find_all_parse_errors(" Is an ok tag. With Many. Words")
        {
            Ok(_) => (),
            Err(e) => 
            {
                panic!("Found parse errors in valid tag: {:?}", e)
            },
        }
    }

    /// Tests parsing for all errors.
    /// Expected to find all invalid characters and leading numeric subtag error.
    #[test]
    fn parse_test_19()
    {
        match Tag::find_all_parse_errors("90.This-Tag.Contains@Many.)()Invalid.%100,.Wrong;:chars[][\\")
        {
            Ok(_) => (),
            Err(e) => 
            {
                assert_eq!(e.len(), 14);
            },
        }
    }

    /// Tests parsing for all errors.
    /// Expected to find all invalid empty string.
    #[test]
    fn parse_test_20()
    {
        match Tag::find_all_parse_errors(" \t  ")
        {
            Ok(_) => (),
            Err(e) => 
            {
                assert_eq!(e.len(), 1);
                assert_eq!(e[0].error_type, ParseErrorType::Tag(TagParseError::TagEmpty));
            },
        }
    }

    /// Tests that a number is not allowed as a tag
    /// Expected to fail
    #[test]
    fn parse_test_21()
    {
        match Tag::find_all_parse_errors("10")
        {
            Ok(_) => (),
            Err(e) => 
            {
                assert_eq!(e.len(), 1);
                assert_eq!(e[0].error_type, ParseErrorType::Tag(TagParseError::FirstTagNumeric));
            },
        }
    }

    #[test]
    fn parse_test_22()
    {
        match Tag::from_str("First.")
        {
            Ok(_) => panic!("Expected error"),
            Err(e) => 
            {
                assert_eq!(e.error_type, ParseErrorType::Tag(TagParseError::SubTagEmpty));
            },
        }
    }

    #[test]
    fn parse_test_23()
    {
        match Tag::from_str("First.[Template]")
        {
            Ok(_) => panic!("Expected error"),
            Err(e) => 
            {
                assert_eq!(e.error_type, ParseErrorType::Tag(TagParseError::InvalidCharacter));
            },
        }
    }

    /// Tests adding a simple tag to a tag set.
    /// Expected to succeed with a single count of the simple tag.
    #[test]
    fn tagset_test_1()
    {
        let mut tag_set = TagSet::new();
        let tag = Tag::from_str("Simple").unwrap();
        tag_set.add_tag(&tag);
        assert!(tag_set.has_tag(&tag));
        assert_eq!(tag_set.count_tag(&tag), 1);
    }

    /// Tests adding a multi-word tag to a tag set.
    /// Expected to succeed with a single count of the multi-word tag and of leading sub-tags.
    #[test]
    fn tagset_test_2()
    {
        let mut tag_set = TagSet::new();
        let tag = Tag::from_str("Simple.Subtag").unwrap();
        tag_set.add_tag(&tag);
        assert!(tag_set.has_tag(&tag));
        assert_eq!(tag_set.count_tag(&tag), 1);

        let leading = Tag::from_str("Simple").unwrap();
        assert!(tag_set.has_tag(&leading));
        assert_eq!(tag_set.count_tag(&leading), 1);
    }

    /// Tests adding multiple different tags
    /// Expected to succeed
    #[test]
    fn tagset_test_3()
    {
        let mut tag_set = TagSet::new();
        let tag = Tag::from_str("Simple").unwrap();
        tag_set.add_tag(&tag);
        assert!(tag_set.has_tag(&tag));
        assert_eq!(tag_set.count_tag(&tag), 1);

        let other = Tag::from_str("Other").unwrap();
        tag_set.add_tag(&other);
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.count_tag(&other), 1);
    }

    /// Tests adding multiple of the same tags
    /// Expected to succeed
    #[test]
    fn tagset_test_4()
    {
        let mut tag_set = TagSet::new();
        let tag = Tag::from_str("Simple").unwrap();
        tag_set.add_tag(&tag);
        assert!(tag_set.has_tag(&tag));
        assert_eq!(tag_set.count_tag(&tag), 1);

        let other = Tag::from_str(" Simple ").unwrap();
        tag_set.add_tag(&other);
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.count_tag(&other), 2);
    }

    /// Tests adding multiple tags
    /// Expected to succeed
    #[test]
    fn tagset_test_5()
    {
        let mut tag_set = TagSet::new();
        let tag = Tag::from_str("Simple").unwrap();
        tag_set.add_tag(&tag);
        assert!(tag_set.has_tag(&tag));
        assert_eq!(tag_set.count_tag(&tag), 1);

        let other = Tag::from_str(" Simple .Subtag").unwrap();
        tag_set.add_tag(&other);
        assert!(tag_set.has_tag(&tag));
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.count_tag(&tag), 2);
        assert_eq!(tag_set.count_tag(&other), 1);
    }

    /// Tests adding and removing multiple tags
    /// Expected to succeed
    #[test]
    fn tagset_test_6()
    {
        let mut tag_set = TagSet::new();
        let tag = Tag::from_str("Simple").unwrap();
        tag_set.add_tag(&tag);
        assert!(tag_set.has_tag(&tag));
        assert_eq!(tag_set.count_tag(&tag), 1);

        let other = Tag::from_str(" Simple .Subtag").unwrap();
        tag_set.add_tag(&other);
        assert!(tag_set.has_tag(&tag));
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.count_tag(&tag), 2);
        assert_eq!(tag_set.count_tag(&other), 1);

        tag_set.remove_tag(&tag);
        assert!(tag_set.has_tag(&tag));
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.count_tag(&tag), 1);
        assert_eq!(tag_set.count_tag(&other), 1);

        tag_set.remove_tag(&other);
        assert!(!tag_set.has_tag(&tag));
        assert!(!tag_set.has_tag(&other));
        assert_eq!(tag_set.count_tag(&tag), 0);
        assert_eq!(tag_set.count_tag(&other), 0);
    }

    /// Tests adding and removing multiple tags
    /// Expected to succeed
    #[test]
    fn tagset_test_7()
    {
        let mut tag_set = TagSet::new();
        let tag = Tag::from_str("Simple").unwrap();
        tag_set.add_tag(&tag);
        assert!(tag_set.has_tag(&tag));
        assert_eq!(tag_set.count_tag(&tag), 1);

        let other = Tag::from_str(" Simple .Subtag").unwrap();
        tag_set.add_tag(&other);
        assert!(tag_set.has_tag(&tag));
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.count_tag(&tag), 2);
        assert_eq!(tag_set.count_tag(&other), 1);

        tag_set.remove_tag(&other);
        assert!(tag_set.has_tag(&tag));
        assert!(!tag_set.has_tag(&other));
        assert_eq!(tag_set.count_tag(&tag), 1);
        assert_eq!(tag_set.count_tag(&other), 0);

        tag_set.remove_tag(&tag);
        assert!(!tag_set.has_tag(&tag));
        assert!(!tag_set.has_tag(&other));
        assert_eq!(tag_set.count_tag(&tag), 0);
        assert_eq!(tag_set.count_tag(&other), 0);
    }
    
    /// Tests the creation of a simple template tag
    #[test]
    fn tag_template_1()
    {
        let temp = TagTemplate::from_str("simple.[template]").unwrap();
        assert_eq!(temp.decomposed_tag.iter().nth(0).unwrap(), &TagTemplateSubtag::Literal("simple".to_string()));
        assert_eq!(temp.decomposed_tag.iter().nth(1).unwrap(), &TagTemplateSubtag::Subtag("template".to_string()));
    }

    /// Tests the creation of a simple template tag with leading template
    #[test]
    fn tag_template_2()
    {
        let temp = TagTemplate::from_str("[simple].template").unwrap();
        assert_eq!(temp.decomposed_tag.iter().nth(0).unwrap(), &TagTemplateSubtag::Subtag("simple".to_string()));
        assert_eq!(temp.decomposed_tag.iter().nth(1).unwrap(), &TagTemplateSubtag::Literal("template".to_string()));
    }

    /// Tests the creation of a template tag full of templates
    #[test]
    fn tag_template_3()
    {
        let temp = TagTemplate::from_str("[simple].[template]").unwrap();
        assert_eq!(temp.decomposed_tag.iter().nth(0).unwrap(), &TagTemplateSubtag::Subtag("simple".to_string()));
        assert_eq!(temp.decomposed_tag.iter().nth(1).unwrap(), &TagTemplateSubtag::Subtag("template".to_string()));
    }

    /// Tests filling in a template tag with values
    #[test]
    fn tag_template_4()
    {
        let mut temp = TagTemplate::from_str("simple.[template]").unwrap();
        temp.insert_template_value("template", &Tag::from_str("inserted").unwrap());
        let tag = temp.into_tag().unwrap();
        assert_eq!(tag, Tag::from_str("simple.inserted").unwrap());
    }

    /// Tests filling in a template tag with values
    #[test]
    fn tag_template_5()
    {
        let mut temp = TagTemplate::from_str("[simple].[template]").unwrap();
        temp.insert_template_value("template", &Tag::from_str("inserted").unwrap());
        temp.insert_template_value("simple", &Tag::from_str("first").unwrap());
        let tag = temp.into_tag().unwrap();
        assert_eq!(tag, Tag::from_str("first.inserted").unwrap());
    }

    /// Tests filling in a template tag with a long tag
    #[test]
    fn tag_template_6()
    {
        let mut temp = TagTemplate::from_str("[simple].[template]").unwrap();
        temp.insert_template_value("template", &Tag::from_str("inserted.other. Long Tag").unwrap());
        temp.insert_template_value("simple", &Tag::from_str("first").unwrap());
        let tag = temp.into_tag().unwrap();
        assert_eq!(tag, Tag::from_str("first.inserted.other.Long Tag").unwrap());
    }

    /// Creating a template that contains no template values.
    /// This is expected to succeed.
    #[test]
    fn tag_template_7()
    {
        let temp = TagTemplate::from_str("no template.tag").unwrap();
        assert!(temp.get_required_inputs().is_empty());
    }

    /// Test no-subtags method
    /// Expects no change to the tag
    #[test]
    fn tag_no_subtags_1()
    {
        let t = Tag::from_str("first").unwrap();
        let r = t.no_subtags();
        assert_eq!(t, r);
    }

    /// Test no-subtags method
    /// Expects only first tag
    #[test]
    fn tag_no_subtags_2()
    {
        let t = Tag::from_str("first.second.third").unwrap();
        let r = t.no_subtags();
        assert_eq!(Tag::from_str("first").unwrap(), r);
    }

    /// Test removing simple prefix
    #[test]
    fn tag_prefix_1()
    {
        let t = Tag::from_str("first.second.third").unwrap();
        assert_eq!(t.remove_prefix(&Tag::from_str("first.second").unwrap()).unwrap(), Tag::from_str("third").unwrap());
    }

    /// Test removing prefix by count
    #[test]
    fn tag_prefix_2()
    {
        let t = Tag::from_str("first.second.third").unwrap();
        assert_eq!(t.remove_prefix_by_count(2).unwrap(), Tag::from_str("third").unwrap());
    }

    /// Test adding and removing prefix
    #[test]
    fn tag_prefix_3()
    {
        let t = Tag::from_str("first.second.third").unwrap();
        let a = Tag::from_str("additional.prefix").unwrap();
        assert_eq!(t.add_prefix(&a), Tag::from_str("additional.prefix.first.second.third").unwrap());
        assert_eq!(t.add_prefix(&a).remove_prefix(&a).unwrap(), t);
    }

    /// Test removing prefix as full tag
    /// Expected to fail
    #[test]
    fn tag_prefix_4()
    {
        let t = Tag::from_str("first.second.third").unwrap();
        assert_eq!(t.remove_prefix(&t), None);
    }

    /// Test removing prefix as full tag by count
    /// Expected to fail
    #[test]
    fn tag_prefix_5()
    {
        let t = Tag::from_str("first.second.third").unwrap();
        assert_eq!(t.remove_prefix_by_count(3), None);
    }

    /// Test removing simple suffix
    #[test]
    fn tag_suffix_1()
    {
        let pre = Tag::from_str("ability.spell.Name Of Spell").unwrap();
        let suf = Tag::from_str("Burning Hands").unwrap();
        assert_eq!(&pre.add_suffix(&suf).name, "ability.spell.Name Of Spell.Burning Hands");
    }

    /// Ensure count subtags method works as expected
    #[test]
    fn count_subtags()
    {
        let t = Tag::from_str("ability").unwrap();
        assert_eq!(t.count_subtags(), 0);
        let t = Tag::from_str("ability.Magic Theory").unwrap();
        assert_eq!(t.count_subtags(), 1);
        let t = Tag::from_str("ability.Magic Theory.Exp").unwrap();
        assert_eq!(t.count_subtags(), 2);
    }


    /// Test getting tags by prefix
    #[test]
    fn tagset_prefix_1()
    {
        let mut ts = TagSet::new();
        ts.add_tag(&Tag::from_str("ability.Magic Theory").unwrap());
        ts.add_tag(&Tag::from_str("ability.Magic Theory.Exp").unwrap());
        ts.add_tag(&Tag::from_str("ability.Latin").unwrap());
        ts.add_tag(&Tag::from_str("ability.Latin.Exp").unwrap());
        ts.add_tag(&Tag::from_str("charateristic.Intelligence").unwrap());
        ts.add_tag(&Tag::from_str("charateristic.Intelligence.Age").unwrap());
        let expected  = vec![
            Tag::from_str("ability").unwrap(),
            Tag::from_str("ability.Magic Theory").unwrap(),
            Tag::from_str("ability.Magic Theory.Exp").unwrap(),
            Tag::from_str("ability.Latin").unwrap(),
            Tag::from_str("ability.Latin.Exp").unwrap(),
        ];
        let result = ts.get_matching_prefix(&Tag::from_str("ability").unwrap());
        assert_eq!(expected.len(), result.len());
        for e in expected
        {
            assert!(result.contains(&e));
        }
    }

    /// Test getting tags by immediate prefix
    #[test]
    fn tagset_prefix_2()
    {
        let mut ts = TagSet::new();
        ts.add_tag(&Tag::from_str("ability.Magic Theory").unwrap());
        ts.add_tag(&Tag::from_str("ability.Magic Theory.Exp").unwrap());
        ts.add_tag(&Tag::from_str("ability.Latin").unwrap());
        ts.add_tag(&Tag::from_str("ability.Latin.Exp").unwrap());
        ts.add_tag(&Tag::from_str("charateristic.Intelligence").unwrap());
        ts.add_tag(&Tag::from_str("charateristic.Intelligence.Age").unwrap());
        let expected = vec![
            Tag::from_str("ability.Magic Theory").unwrap(),
            Tag::from_str("ability.Latin").unwrap(),
        ];
        let result = ts.get_immediate_matching_prefix(&Tag::from_str("ability").unwrap());
        assert_eq!(expected.len(), result.len());
        for e in expected
        {
            assert!(result.contains(&e));
        }
    }

    /// Tests prefix search searching properly by subtags
    #[test]
    fn tagset_prefix_3()
    {
        let mut ts = TagSet::new();
        ts.add_tag(&Tag::from_str("ability.Magic Theory").unwrap());
        ts.add_tag(&Tag::from_str("ability.Magic Theory Extended").unwrap());
        ts.add_tag(&Tag::from_str("ability.Magic Theory Extended.Exp").unwrap());
        ts.add_tag(&Tag::from_str("ability.Magic Theory.Exp").unwrap());
        ts.add_tag(&Tag::from_str("ability.Latin").unwrap());
        ts.add_tag(&Tag::from_str("ability.Latin.Exp").unwrap());
        ts.add_tag(&Tag::from_str("charateristic.Intelligence").unwrap());
        ts.add_tag(&Tag::from_str("charateristic.Intelligence.Age").unwrap());
        let expected = vec![
            Tag::from_str("ability.Magic Theory.Exp").unwrap(),
        ];
        let result = ts.get_immediate_matching_prefix(&Tag::from_str("ability.Magic Theory").unwrap());
        assert_eq!(expected.len(), result.len());
        for e in expected
        {
            assert!(result.contains(&e));
        }
    }
}
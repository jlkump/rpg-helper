use std::{collections::{HashMap, HashSet}, fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};
use string_interner::{backend::StringBackend, symbol::{SymbolU16, SymbolU32}, StringInterner, Symbol};

use std::ops::Index;

use crate::api::data::{error::{ParseError, ParseErrorType, TagParseError, TemplateError}, template::{Template, Templated}};

static TAG_DELIMITER: char = '.';

/// This is where all tags' string value equivalents are stored.
/// In order to create a tag from a string, a registry must be made
/// and used.
/// 
/// Likewise, when a tag needs to be displayed in a string format,
/// a registry is required.
/// 
/// This complicates equation evaluation slightly, as now plain
/// string tags in the equation may or may not be valid tags registered
/// in a registry >:(
/// 
/// On the upside, tags are more compact (as long as the tag was more than 4 characters)
/// and more efficient in comparison and tag operations :D
#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub struct TagRegistry
{
    string_interner: StringInterner<StringBackend<SymbolU32>>,
}

impl TagRegistry
{
    /// Create a new empty tag registry. This is primarily used for testing,
    /// as the rpg layer has a set of reserved subtags.
    /// 
    /// Thus, [`TagRegistry::new_with_reserved`] should be used intead.
    pub fn new() -> TagRegistry
    {
        TagRegistry { string_interner: StringInterner::new() }
    }

    /// Creates a new registry with some set of reserved subtags.
    /// The order of the reserved subtags will be used for the creation
    /// of the subtag ids. This allows a static time assumption of what
    /// certain ids for subtags will be.
    /// 
    /// The reserved subtags are declared in each layer of the api, with the
    /// highest layer's reserved tags being the ones that should be used
    /// when that layer is included. I.E., if you use only the data layer,
    /// then no tags are reserved.
    pub fn new_with_reserved(reserved_subtags: &[&str]) -> TagRegistry
    {
        TagRegistry { string_interner: reserved_subtags.into_iter().collect::<StringInterner<StringBackend<SymbolU32>>>() }
    }

    /// Returns the tag value of a string if the string is formatted correctly as a tag.
    /// Otherwise, returns the parse errors in the string.
    /// 
    /// When called, each subtag (deliminated by '.') is checked to be contained
    /// in the registry. If it is not present, the subtag is registered. All
    /// registered subtags are then combined to create the final tag.
    /// 
    /// If the string contains parse errors, then no new subtags are registered.
    pub fn get_or_register_tag(&mut self, tag_str: &str) -> Result<Tag, ParseError>
    {
        // Initial error check to ensure not empty
        Self::check_parse_error(tag_str, Self::is_valid_tag_char)?;

        // Ensure first sub-string is not just a number
        // let first_str = if let Some(f) = s.split('.').next()
        // {
        //     f
        // }
        // else
        // {
        //     s
        // };

        // if first_str.chars().all(|c| c.is_numeric() || c.is_whitespace())
        // {
        //     return Err(ParseError::new(s.to_string(), s.len() - 1, ParseErrorType::Tag(TagParseError::FirstTagNumeric)));
        // }

        // Loop through each substring s
        //      Trim the outer white-space and make lowercase
        //      Get or register 
        let subtags = tag_str.split(TAG_DELIMITER).map(|substring| Subtag::from(self.string_interner.get_or_intern(substring.trim().to_lowercase()))).collect();
        Ok(Tag { subtags })
    }

    pub fn get_or_register_subtag(&mut self, subtag_str: &str) -> Result<Subtag, ParseError>
    {
        Self::check_parse_error(subtag_str, Self::is_valid_subtag_char)?;

        Ok(self.string_interner.get_or_intern(subtag_str.trim().to_lowercase()).into())
    }

    /// Retrieves the tag handle from a given string if it exists.
    /// If the given string is poorly formatted, then a parse error is returned
    pub fn get_tag(&self, tag_str: &str) -> Result<Option<Tag>, ParseError>
    {
        Self::check_parse_error(tag_str, Self::is_valid_tag_char)?;

        Ok(tag_str.split(TAG_DELIMITER)
            .map(|substring| self.string_interner.get(substring))
            .try_fold(vec![], |mut acc, subtag|
                {
                    match subtag
                    {
                        Some(subtag) =>
                        {
                            acc.push(subtag.into());
                            Some(acc)
                        },
                        None => None,
                    }
                })
            .map(|subtags| Tag { subtags }))
    }

    pub fn get_subtag(&self, subtag_str: &str) -> Result<Option<Subtag>, ParseError>
    {
        Self::check_parse_error(subtag_str, Self::is_valid_subtag_char)?;

        Ok(self.string_interner.get(subtag_str.trim().to_lowercase()).map(|i| i.into()))
    }

    pub fn find_all_parse_errors(s: &str) -> Result<(), Vec<ParseError>>
    {
        let mut res = vec![];
        if s.is_empty() || s.chars().all(char::is_whitespace)
        {
            res.push(ParseError::new(s.to_string(), s.len(), ParseErrorType::Tag(TagParseError::TagEmpty)));
            return Err(res);
        }

        // let first_str = if s.contains('.')
        // {
        //     s.split('.').next().unwrap()
        // }
        // else
        // {
        //     s
        // };

        // if first_str.chars().all(char::is_numeric)
        // {
        //     res.push(ParseError::new(s.to_string(), s.len() - 1, ParseErrorType::Tag(TagParseError::FirstTagNumeric)));
        // }

        for (i, c) in s.chars().enumerate()
        {
            if !Self::is_valid_tag_char(c)
            {
                res.push(ParseError::new(s.to_string(), i, ParseErrorType::Tag(TagParseError::InvalidCharacter)));
            }
        }

        for sub in s.split(TAG_DELIMITER)
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

    // =============== Private Helpers ===================

    fn is_valid_tag_char(c: char) -> bool
    {
        Self::is_valid_subtag_char(c) || c == TAG_DELIMITER
    }
    
    fn is_valid_subtag_char(c: char) -> bool
    {
        c.is_alphanumeric() || c.is_whitespace()
    }

    fn check_parse_error<T: Fn(char) -> bool>(s: &str, valid: T) -> Result<(), ParseError>
    {
        // Initial error check to ensure not empty
        if s.is_empty() || s.chars().all(char::is_whitespace)
        {
            return Err(ParseError::new(s.to_string(), s.len(), ParseErrorType::Tag(TagParseError::TagEmpty)));
        }

        // Check that the string only contains valid characters
        if !s.chars().all(|c| valid(c))
        {
            return Err(ParseError::new(s.to_string(), s.find(|c| !valid(c)).unwrap(), ParseErrorType::Tag(TagParseError::InvalidCharacter)));
        }

        Ok(())
    }
}

/// Subtags make up a larger part of a tag. As singular instances, they can be used as
/// a unique string-based indentifier for a ruleset.
/// 
/// ## Restrictions
/// The string of a subtag may only contain valid alphanumeric values.
/// Unlike tags, subtags can not contain '.' delimiters.
/// Capitalization does not affect the subtag value for comparisons.
/// 
/// **Examples**
/// - `subtag`
/// - `tag`
/// - `a long subtag`
/// - `MixeD CapItalIzaTioN`
/// - `1 Numeric Value`
/// - `50356`
/// 
/// **Counter Examples**
/// - `a.multivalue.tag`
/// - `non alphanumeric! "tag" values`
#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Clone, Copy, Hash)]
pub struct Subtag
{
    intern_id: SymbolU32,
}

impl Subtag
{
    pub(crate) fn reserved_new(id: u32) -> Subtag
    {
        Subtag { intern_id: SymbolU32::try_from_usize(id as usize).unwrap() }
    }
}

impl From<SymbolU32> for Subtag
{
    fn from(value: SymbolU32) -> Self
    {
        Self { intern_id: value }
    }
}

impl From<Subtag> for Tag
{
    fn from(value: Subtag) -> Self
    {
        Tag { subtags: vec![value] }
    }
}

impl From<&Subtag> for Tag
{
    fn from(value: &Subtag) -> Self
    {
        Tag { subtags: vec![*value] }
    }
}

impl Default for Subtag
{
    fn default() -> Self
    {
        Self::reserved_new(0)
    }
}

// TODO: A macro that creates a tag from a variable number of Subtags
// tag_from_subtags!(TIMELINE, DEFAULT)
#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, Clone, Hash)]
pub struct Tag
{
    subtags: Vec<Subtag>,
}

impl Tag
{
    pub fn to_string(&self, registry: &TagRegistry) -> Option<String>
    {
        let mut subtags = self.subtags.iter().map(|subtag| registry.string_interner.resolve(subtag.intern_id)).peekable();
        
        let mut string = String::new();
        while let Some(subtag) = subtags.next()
        {
            match subtag
            {
                Some(subtag) => string.push_str(subtag),
                None => return None,
            }
            if subtags.peek().is_some()
            {
                string.push(TAG_DELIMITER);
            }
        }
        Some(string)
    }

    /// Removes the prefix of the tag up to the matching given prefix.
    /// 
    /// Returns None if no match is found or the entire tag is removed
    /// 
    /// ## Examples
    /// ```
    /// let registry = TagRegistry::new_with_reserved(&vec!["ability", "spell", "name of spell"]);
    /// 
    /// let tag = registry.get_tag("ability.spell.Name of Spell").unwrap();
    /// let ability_spell = registry.get_tag("ability.spell").unwrap();
    /// let ability_spell_name = registry.get_tag("ability.spell.name of spell").unwrap();
    /// 
    /// assert_eq!(registry.get_tag("Name Of Spell"), tag.remove_prefix(ability_spell));
    /// assert_eq!(None, tag.remove_prefix(ability_spell_name));
    /// ```
    pub fn remove_prefix(&self, prefix: &Tag) -> Option<Tag>
    {
        let mut first_diff = None;
        for ((i, lhs), rhs) in self.subtags.iter().enumerate().zip(prefix.subtags.iter())
        {
            if lhs != rhs
            {
                first_diff = Some(i);
                break;
            }
        }

        match first_diff
        {
            // The index `i` will always be in range, as, in order to be found
            // the iterator above must have run and found some subtag for which `lhs` != `rhs`.
            // It is also not possible for the resulting subtag string to be empty, as again,
            // this would only happen if i >= self.subtags.len(), which is not possible.
            Some(i) => Some(Tag { subtags: self.subtags[i..].to_vec() }),
            None => None,
        }
    }

    /// Removes the first `count` elements from the subtags of the tag
    /// If the count is greater than the number of subtags, then this
    /// method will return none.
    pub fn remove_prefix_by_count(&self, count: usize) -> Option<Tag>
    {
        if count >= self.subtags.len()
        {
            None
        }
        else
        {
            Some(Tag { subtags: self.subtags[count..].to_vec() })
        }
    }
    
    /// Prefix a tag with another prefix tag
    /// Ex:
    ///     add_prefix(Name Of Spell, ability.spell) -> ability.spell.Name Of Spell
    pub fn add_prefix(&self, prefix: &Tag) -> Tag
    {
        Tag { subtags: prefix.subtags.iter().map(|s| *s).chain(self.subtags.iter().map(|s| *s)).collect() }
    }

    pub fn has_prefix(&self, prefix: &Tag) -> bool
    {
        if self.subtags.len() >= prefix.subtags.len()
        {
            self.subtags.starts_with(&prefix.subtags)
        }
        else
        {
            false
        }
    }

    /// Removes the suffix of the tag up to the matching given suffix.
    /// 
    /// Returns None if no match is found or the entire tag is removed
    /// 
    /// ## Examples
    /// ```
    /// let registry = TagRegistry::new_with_reserved(&vec!["ability", "spell", "name of spell"]);
    /// 
    /// let tag = registry.get_tag("ability.spell.Name of Spell").unwrap();
    /// let spell_name = registry.get_tag("spell.name of spell").unwrap();
    /// let ability_spell_name = registry.get_tag("ability.spell.name of spell").unwrap();
    /// 
    /// assert_eq!(registry.get_tag("ability"), tag.remove_suffix(spell_name));
    /// assert_eq!(None, tag.remove_suffix(ability_spell_name));
    /// ```
    pub fn remove_suffix(&self, suffix: &Tag) -> Option<Tag>
    {
        let mut first_diff = None;
        for ((i, lhs), rhs) in self.subtags.iter().rev().enumerate().zip(suffix.subtags.iter().rev())
        {
            if lhs != rhs
            {
                first_diff = Some(i);
                break;
            }
        }

        match first_diff
        {
            // The index `i` will always be in range, as, in order to be found
            // the iterator above must have run and found some subtag for which `lhs` != `rhs`.
            // It is also not possible for the resulting subtag string to be empty, as again,
            // this would only happen if i >= self.subtags.len(), which is not possible.
            Some(i) =>
            {
                let r = self.subtags.iter().rev().collect::<Vec<&Subtag>>()[i..].into_iter().rev().map(|s| **s).collect();
                Some(Tag { subtags: r })
            },
            None => None,
        }
    }

    /// Removes the suffix of the tag by a given count of subtags.
    /// Returns None if the entire tag is removed
    /// 
    /// ## Examples
    /// ```
    /// let registry = TagRegistry::new_with_reserved(&vec!["ability", "spell", "name of spell"]);
    /// 
    /// let tag = registry.get_tag("ability.spell.Name of Spell").unwrap();
    /// 
    /// assert_eq!(registry.get_tag("ability.spell"), tag.remove_suffix_by_count(1));
    /// assert_eq!(registry.get_tag("ability"), tag.remove_suffix_by_count(2));
    /// assert_eq!(None, tag.remove_suffix_by_count(3));
    /// ```
    pub fn remove_suffix_by_count(&self, count: usize) -> Option<Tag>
    {
        if self.subtags.len() >= count + 1
        {
            Some(Tag { subtags: self.subtags[..count + 1].to_vec() } )
        }
        else
        {
            None
        }
    }

    /// Suffix a tag with another tag
    /// 
    /// ## Examples
    /// ```
    /// let registry = TagRegistry::new_with_reserved(&vec!["ability", "spell", "name of spell"]);
    /// 
    /// let tag = registry.get_tag("ability.spell").unwrap();
    /// let suffix = registry.get_tag("name of spell").unwrap();
    /// 
    /// assert_eq!(registry.get_tag("ability.spell.name of spell").unwrap(), tag.add_suffix(suffix));
    /// ```
    pub fn add_suffix(&self, suffix: &Tag) -> Tag
    {
        Self::add_prefix(suffix, self)
    }

    /// Check the suffix of a tag given another tag.
    /// If the suffix subtag length is greater than
    /// this tag's length, then the check fails.
    /// 
    /// ## Examples
    /// ```
    /// let registry = TagRegistry::new_with_reserved(&vec!["ability", "spell", "name of spell"]);
    /// 
    /// let tag = registry.get_tag("ability.spell.name of spell").unwrap();
    /// let suffix = registry.get_tag("name of spell").unwrap();
    /// let suffix_two = registry.get_tag("spell.name of spell").unwrap();
    /// 
    /// assert!(tag.has_suffix(suffix));
    /// assert!(tag.has_suffix(suffix_two));
    /// ```
    pub fn has_suffix(&self, suffix: &Tag) -> bool
    {
        if self.subtags.len() >= suffix.subtags.len()
        {
            self.subtags.ends_with(&suffix.subtags)
        }
        else
        {
            false
        }
    }

    /// Returns the subtags in this tag as a slice.
    /// 
    /// You can also directly index into a tag to access specific subtags. 
    /// 
    /// ## Examples
    /// ```
    /// let registry = TagRegistry::new_with_reserved(&vec!["ability", "spell", "name of spell"]);
    /// 
    /// let tag = registry.get_tag("ability.spell.name of spell").unwrap();
    /// let subtags = vec![registry.get_subtag("ability").unwrap(), registry.get_subtag("spell").unwrap()];
    /// assert_eq!(subtags.as_slice(), tag.as_slice()[..3]);
    /// ```
    pub fn as_subtag_slice(&self) -> &[Subtag]
    {
        self.subtags.as_slice()
    }

    /// Counts the number of subtags in this tag.
    /// 
    /// Equivalent to `&[..].len()`
    pub fn count_subtags(&self) -> usize
    {
        self[..].len()
    }

    /// Unlike a subtag slice view of this tag, this view creates successively larger tags from an initial
    /// prefix of just the initial subtag.
    /// 
    /// ## Examples
    /// ```
    /// let registry = TagRegistry::new_with_reserved(&vec!["ability", "spell", "name of spell"]);
    /// 
    /// let tag = registry.get_tag("ability.spell.name of spell").unwrap();
    /// let subtags = vec!["ability", "ability.spell", "ability.spell.name Of Spell"]
    ///     .into_iter().map(|s| registry.get_tag(s).unwrap())
    ///     .collect();
    /// 
    /// assert_eq!(subtags, tag.as_collective_subtags());
    /// ```
    pub fn as_collective_subtags(&self) -> Vec<Tag>
    {
        let (_, res) = self.subtags.iter().fold((vec![], vec![]), 
            |(mut subtags, mut result), subtag|
            {
                subtags.push(*subtag); 
                result.push(Tag { subtags: subtags.clone() } );
                (subtags, result)
            }
        );
        res
    }
}

impl<Idx> std::ops::Index<Idx> for Tag
where
    Idx: std::slice::SliceIndex<[Subtag]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output
    {
        &self.subtags[index]
    }
}

impl Default for Tag
{
    fn default() -> Self
    {
        Subtag::default().into()
    }
}

/// Contains tags registered with a TagRegistry.
/// 
/// It is important to note, a TagSet is not explicitly associated
/// with a TagRegistry. Thus, the tags it contains may not be valid
/// depending on the TagRegistry used. This also means that Tags
/// between TagRegistries can not be mixed and matched!
/// 
/// A tag can be made of smaller sub-tags, which are children to the
/// greater tag. For example, Ability.Magic Theory as a tag has
/// Ability as the first sub-tag and Magic Theory as the second.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct TagSet
{
    primary_tags: HashMap<Tag, i32>,
    tags: HashMap<Tag, i32>,
}

impl TagSet
{
    /// Creates a new empty TagSet
    pub fn new() -> TagSet
    {
        TagSet { primary_tags: HashMap::new(), tags: HashMap::new() }
    }

    pub fn get_tag_count(&self, t: &Tag) -> i32
    {
        self[t]
    }

    pub fn add_tag_count(&mut self, t: &Tag, c: i32)
    {
        for st in t.as_collective_subtags()
        {
            let v = self.tags.get(&st).unwrap_or(&0);
            self.tags.insert(st, v + c);
        }
        self.primary_tags.insert(t.clone(), c + self.primary_tags.get(t).unwrap_or(&0));
    }

    pub fn remove_tag_count(&mut self, t: &Tag, c: i32)
    {
        self.add_tag_count(t, -c);
    }

    pub fn has_tag(&self, t: &Tag) -> bool
    {
        self.get_tag_count(t) > 0
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
    /// 
    /// ## Examples
    /// ```
    /// let registry = TagRegistry::new_with_reserved(&vec!["ability", "Magic Theory", "Exp", "Latin"]);
    /// 
    /// let tags: Vec<Tag> = vec!["ability.Magic Theory", "ability.Magic Theory.Exp", "ability.Latin", "ability.Latin.Exp"].into_iter()
    ///     .map(|s| registry.get_tag(s).unwrap())
    ///     .collect();
    /// 
    /// let mut tag_set = TagSet::new();
    /// for t in tags.iter()
    /// {
    ///     tag_set.add_tag(t);
    /// }
    /// 
    /// let matching = tag_set.get_matching_prefix(registry.get_tag("ability").unwrap());
    /// assert_eq!(matching.sort(), tags.sort());
    /// ```
    pub fn get_matching_prefix(&self, prefix: &Tag) -> Vec<Tag>
    {
        self.tags.iter().filter_map(|(t, c)| 
        {
            if *c > 0 && t.has_prefix(&prefix)
            {
                Some(t.clone())
            }
            else
            {
                None
            }
        }).collect()
    }

    /// Given a tag prefix, this method returns all immediate tags which exist in this tag set (the count of the tag must be > 0)
    /// 
    /// In other words: `get_immediate_matching_prefix(ability) -> [ability.Magic Theory, ability.Latin]`
    /// but does not return `[ability.Magic Theory.Exp, ability.Latin.Exp]` when the tag set
    /// is `ability.Magic Theory, ability.Magic Theory.Exp, ability.Latin, ability.Latin.Exp`
    /// 
    /// ## Examples
    /// ```
    /// let registry = TagRegistry::new_with_reserved(&vec!["ability", "Magic Theory", "Exp", "Latin"]);
    /// 
    /// let tags: Vec<Tag> = vec!["ability.Magic Theory", "ability.Magic Theory.Exp", "ability.Latin", "ability.Latin.Exp"].into_iter()
    ///     .map(|s| registry.get_tag(s).unwrap())
    ///     .collect();
    /// 
    /// let mut tag_set = TagSet::new();
    /// for t in tags.iter()
    /// {
    ///     tag_set.add_tag(t);
    /// }
    /// 
    /// let expected: Vec<Tag> = vec!["ability.Magic Theory", "ability.Latin"].into_iter()
    ///     .map(|s| registry.get_tag(s).unwrap())
    ///     .collect();
    /// let matching = tag_set.get_matching_prefix(registry.get_tag("ability").unwrap());
    /// assert_eq!(matching.sort(), expected.sort());
    /// ```
    pub fn get_immediate_matching_prefix(&self, prefix: &Tag) -> Vec<Tag>
    {
        let num_subtags = prefix.count_subtags() + 1;
        self.tags.iter().filter_map(|(t, c)|
        {
            if *c > 0 && t.count_subtags() == num_subtags && t.has_prefix(&prefix) 
            {
                Some(t.clone())
            }
            else
            {
                None
            }
        }
        ).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tag> + '_
    {
        self.tags.keys()
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

impl Index<&Tag> for TagSet
{
    type Output = i32;
 
    #[inline]
    fn index(&self, index: &Tag) -> &Self::Output
    {
        &self.tags[index]
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

impl TagTemplateSubtag
{
    fn is_literal(&self) -> bool
    {
        match self
        {
            TagTemplateSubtag::Literal(_) => true,
            TagTemplateSubtag::Subtag(_) => false,
        }
    }
}

impl TagTemplate
{
    /// Convention for a template's `new` method
    /// is to return a Templated result of
    /// either the template value or the finished
    /// value if the template is already complete.
    pub fn new(s: &str) -> Result<Templated<TagTemplate, Tag>, ParseError>
    {
        match Self::from_str(s)
        {
            Ok(t) => Ok(Templated::Template(t)),
            Err(e) =>
            {
                if e.error_type == ParseErrorType::Tag(TagParseError::MissingTemplate)
                {
                    if let Ok(t) = Tag::from_str(s)
                    {
                        return Ok(Templated::Complete(t));
                    }
                }
                Err(e)
            },
        }
    }

    /// Given a string, constructs a TagTemplate
    /// Expects to find at least one template value, indicated by a subtag surrounded
    /// by "[]". Also expects each template identifier to be unique
    /// 
    /// Ex:
    ///     "tag.test.[template].value"
    /// 
    /// Unless you know the given string is garanteed to have
    /// a template value, it is better to call `new`,
    /// as it ensures a template tag is in a valid state.
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

        if decomposed_tag.iter().all(|tok| tok.is_literal())
        {
            Err(ParseError::new(s.to_string(), s.len() - 1, ParseErrorType::Tag(TagParseError::MissingTemplate)))
        }
        else
        {
            Ok(TagTemplate { decomposed_tag })
        }
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
    fn fill_template_value(&mut self, input_name: &str, input_value: &Tag) -> Option<Tag>
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

impl Display for TagTemplate
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let result_string = self.decomposed_tag.iter().map(|tok|
        {
            match tok
            {
                TagTemplateSubtag::Literal(l) => l.to_string(),
                TagTemplateSubtag::Subtag(s) => format!("[{}]", s),
            }
        }).fold(String::new(), |mut f, tok|
        {
            f.push_str(&tok);
            f.push('.');
            f
        });
        let result = &result_string[..result_string.len()];

        write!(f, "{}", result)
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
        assert_eq!(tag_set.get_tag_count(&tag), 1);
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
        assert_eq!(tag_set.get_tag_count(&tag), 1);

        let leading = Tag::from_str("Simple").unwrap();
        assert!(tag_set.has_tag(&leading));
        assert_eq!(tag_set.get_tag_count(&leading), 1);
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
        assert_eq!(tag_set.get_tag_count(&tag), 1);

        let other = Tag::from_str("Other").unwrap();
        tag_set.add_tag(&other);
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.get_tag_count(&other), 1);
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
        assert_eq!(tag_set.get_tag_count(&tag), 1);

        let other = Tag::from_str(" Simple ").unwrap();
        tag_set.add_tag(&other);
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.get_tag_count(&other), 2);
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
        assert_eq!(tag_set.get_tag_count(&tag), 1);

        let other = Tag::from_str(" Simple .Subtag").unwrap();
        tag_set.add_tag(&other);
        assert!(tag_set.has_tag(&tag));
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.get_tag_count(&tag), 2);
        assert_eq!(tag_set.get_tag_count(&other), 1);
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
        assert_eq!(tag_set.get_tag_count(&tag), 1);

        let other = Tag::from_str(" Simple .Subtag").unwrap();
        tag_set.add_tag(&other);
        assert!(tag_set.has_tag(&tag));
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.get_tag_count(&tag), 2);
        assert_eq!(tag_set.get_tag_count(&other), 1);

        tag_set.remove_tag(&tag);
        assert!(tag_set.has_tag(&tag));
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.get_tag_count(&tag), 1);
        assert_eq!(tag_set.get_tag_count(&other), 1);

        tag_set.remove_tag(&other);
        assert!(!tag_set.has_tag(&tag));
        assert!(!tag_set.has_tag(&other));
        assert_eq!(tag_set.get_tag_count(&tag), 0);
        assert_eq!(tag_set.get_tag_count(&other), 0);
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
        assert_eq!(tag_set.get_tag_count(&tag), 1);

        let other = Tag::from_str(" Simple .Subtag").unwrap();
        tag_set.add_tag(&other);
        assert!(tag_set.has_tag(&tag));
        assert!(tag_set.has_tag(&other));
        assert_eq!(tag_set.get_tag_count(&tag), 2);
        assert_eq!(tag_set.get_tag_count(&other), 1);

        tag_set.remove_tag(&other);
        assert!(tag_set.has_tag(&tag));
        assert!(!tag_set.has_tag(&other));
        assert_eq!(tag_set.get_tag_count(&tag), 1);
        assert_eq!(tag_set.get_tag_count(&other), 0);

        tag_set.remove_tag(&tag);
        assert!(!tag_set.has_tag(&tag));
        assert!(!tag_set.has_tag(&other));
        assert_eq!(tag_set.get_tag_count(&tag), 0);
        assert_eq!(tag_set.get_tag_count(&other), 0);
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
        temp.fill_template_value("template", &Tag::from_str("inserted").unwrap());
        let tag = temp.into_tag().unwrap();
        assert_eq!(tag, Tag::from_str("simple.inserted").unwrap());
    }

    /// Tests filling in a template tag with values
    #[test]
    fn tag_template_5()
    {
        let mut temp = TagTemplate::from_str("[simple].[template]").unwrap();
        temp.fill_template_value("template", &Tag::from_str("inserted").unwrap());
        temp.fill_template_value("simple", &Tag::from_str("first").unwrap());
        let tag = temp.into_tag().unwrap();
        assert_eq!(tag, Tag::from_str("first.inserted").unwrap());
    }

    /// Tests filling in a template tag with a long tag
    #[test]
    fn tag_template_6()
    {
        let mut temp = TagTemplate::from_str("[simple].[template]").unwrap();
        temp.fill_template_value("template", &Tag::from_str("inserted.other. Long Tag").unwrap());
        temp.fill_template_value("simple", &Tag::from_str("first").unwrap());
        let tag = temp.into_tag().unwrap();
        assert_eq!(tag, Tag::from_str("first.inserted.other.Long Tag").unwrap());
    }

    /// Creating a template that contains no template values.
    /// This is expected to fail.
    #[test]
    fn tag_template_7()
    {
        let temp = TagTemplate::from_str("no template.tag");
        if let Err(e) = temp
        {
            assert_eq!(e.error_type, ParseErrorType::Tag(TagParseError::MissingTemplate));
            assert_eq!(e.string, "no template.tag");
        }
        else
        {
            panic!("Succeeded in creating a template of a non templatable string.")
        }
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

    /// Test to ensure the interner uses usize indexes in order
    #[test]
    fn tag_intern_test()
    {
        use string_interner::{StringInterner, Symbol};

        let mut interner = StringInterner::default();
        let sym0 = interner.get_or_intern("Elephant");
        let sym1 = interner.get_or_intern("Tiger");
        let sym2 = interner.get_or_intern("Horse");
        let sym3 = interner.get_or_intern("Tiger");

        assert_eq!(sym0.to_usize(), 0);
        assert_eq!(sym1.to_usize(), 1);
        assert_eq!(sym2.to_usize(), 2);
        assert_eq!(sym3.to_usize(), 1);
    }

    // Looks like the statics work, so I'll be using this for declaring tags
    // #[test]
    // fn tag_static_test_1()
    // {
    //     assert_eq!(RESERVED.intern_id, 0);
    //     assert_eq!(DIE_ROLL.intern_id, 6);
    // }

    // #[test]
    // fn tag_static_test_2()
    // {
    //     let mut interner = RESERVED_SUBTAG_STRINGS.into_iter().collect::<DefaultStringInterner>();
    //     let sym0 = interner.get_or_intern("reserved");
    //     let sym1 = interner.get_or_intern("die roll");
    //     let sym2 = interner.get_or_intern("Horse");

    //     assert_eq!(sym0.to_usize() as u32, RESERVED.intern_id);
    //     assert_eq!(sym1.to_usize() as u32, DIE_ROLL.intern_id);
    //     assert_eq!(sym2.to_usize(), RESERVED_SUBTAG_COUNT);

    // }
}
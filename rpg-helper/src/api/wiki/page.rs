use serde::{Deserialize, Serialize};

use crate::api::{data::tag::Tag, wiki::{error::WikiError, syntax::Block}};

/// A page is edited as simple markdown text by the user.
/// It displays text, images, ctx values, and can link to other pages.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Page
{
    id: Tag,
    raw_text: String,   // Written in markdown
    cached_syntax: Vec<Block>,
}

impl Page
{
    pub fn new(id: Tag) -> Page
    {
        Page { id, raw_text: String::new(), cached_syntax: vec![] }
    }

    pub fn set_text(&mut self, text: &str) -> Result<String, WikiError>
    {
        let old = self.raw_text.clone();
        self.raw_text = text.to_string();
        if let Err(e) = self.update_syntax_cache()
        {
            self.raw_text = old;
            Err(e)
        }
        else
        {
            Ok(old)
        }
    }

    pub fn iter_syntax(&self) -> impl Iterator<Item = &Block>
    {
        self.cached_syntax.iter()
    }

    pub fn get_referenced_pages(&self) -> Vec<Tag>
    {
        todo!()
    }

    fn update_syntax_cache(&mut self) -> Result<String, WikiError>
    {
        todo!()
    }
}
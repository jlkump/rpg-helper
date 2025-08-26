use serde::{Deserialize, Serialize};

use crate::api::data::tag::Tag;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Block
{
    Header(HeaderLevel, Vec<Inline>),
    Paragraph(Vec<Inline>),
    BlockQuote(Vec<Block>),
    List(ListType, Vec<Inline>),
    CodeBlock(String, Option<String>), // content, language
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Inline
{
    Text(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    Code(String),
    TagReference(TagRef),
    ImageReference(ImageRef),
    PageReference(PageRef),
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone, Copy)]
pub enum HeaderLevel
{
    H1, H2, H3, H4, H5, H6
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone, Copy)]
pub enum ListType
{
    Ordered,
    Unordered,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct TagRef
{

}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ImageRef
{

}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct PageRef
{
    name: String,
    page_id: Tag,
}
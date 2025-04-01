mod atoms;
mod molecules;
mod organisms;
pub mod pages;

pub trait ToHtml {
    fn to_html(&self) -> yew::Html;
}
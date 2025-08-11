use rpg_helper::api::display::icon::Icon;
use yew::prelude::*;

pub trait ToClientHtml
{
    fn to_html(&self) -> Html;
}

impl ToClientHtml for Icon
{
    fn to_html(&self) -> Html
    {
        todo!()
    }
}
use std::ops::Deref;

use stylist::yew::styled_component;
use yew::{hook, html, use_state, Children, ContextProvider, Html, Properties, UseStateHandle};

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub paper: String,
    pub paper_dark: String,
    pub panel_primary: String,
    pub panel_secondary: String,
    pub panel_color_primary: String,
    pub panel_color_secondary: String,
    pub text_default: String,
    pub text_faint: String,
    pub text_invert: String,
    pub text_colored: String,
    pub text_link: String,
    pub text_link_highlight: String,
    pub decorative_scroll_border: String,
    pub decorative_scroll_drop_shadow: String,
    pub button_color: String,
    pub button_color_hover: String,
    pub button_color_press: String,
    pub button_color_disabled: String,
    pub h1: String,
    pub h2: String,
    pub h3: String,
    pub h4: String,
    pub h5: String,
    pub h6: String,
    pub header_line: String,
    pub scroll_bar: String,
    pub scroll_bar_hover: String,
    pub scroll_bar_drag: String,
    pub border_colored: String,
    pub border_light: String,
    pub border_hard: String,
    pub border_tooltip_light: String,
    pub border_tooltip_hard: String,
    pub image_border: String,
    pub hover_dropshadow: String,
    pub navbar_line: String,
    pub logo: String,
    pub hamburger_menu: String
}

#[derive(Debug, Clone)]
pub(crate) struct ThemeContext {
    inner: UseStateHandle<Theme>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<Theme>) -> Self {
        Self { inner }
    }

    pub fn set(&self, theme: Theme) {
        self.inner.set(theme)
    }
}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ThemeProviderProps {
    pub children: Children,
}

#[styled_component]
pub(crate) fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme_kind = use_state(|| get_default_theme());

    let theme_ctx = ThemeContext::new(theme_kind);

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

fn get_default_theme() -> Theme {
    Theme {
        paper: "#ece9e4".to_owned(),
        paper_dark: "#e2ded8".to_owned(),
        panel_primary: "#e4e0d5".to_owned(),
        panel_secondary: "#e1dccf".to_owned(),
        panel_color_primary: "#5a0000".to_owned(),
        panel_color_secondary: "#6f2320".to_string(),
        text_default: "#393636".to_owned(),
        text_faint: "#636e83".to_owned(),
        text_invert: "#e6ebee".to_owned(),
        text_colored: "#7a0002".to_owned(),
        text_link: "#265dc0".to_owned(),
        text_link_highlight: "#3e9bf3".to_owned(),
        decorative_scroll_border: "#7a0002".to_owned(),
        decorative_scroll_drop_shadow: "rgb(163, 122, 93, 0.5)".to_owned(),
        button_color: "#7a0002".to_owned(),
        button_color_hover: "#6f2320".to_owned(),
        button_color_press: "#5a0000".to_owned(),
        button_color_disabled: "#c2a6a3".to_owned(),
        h1: "#c59654".to_owned(),
        h2: "#0a2666".to_owned(),
        h3: "#590000".to_owned(),
        h4: "#a26550".to_owned(),
        h5: "#393636".to_owned(),
        h6: "#c79a5a".to_owned(),
        header_line: "#000000".to_owned(),
        scroll_bar: "#5a0000".to_owned(),
        scroll_bar_hover: "#da9840".to_owned(),
        scroll_bar_drag: "#da9840".to_owned(),
        border_colored: "#7a0002".to_owned(),
        border_light: "#93918e".to_owned(),
        border_hard: "#000000".to_owned(),
        border_tooltip_light: "#bd9f89".to_owned(),
        border_tooltip_hard: "#7a0002".to_owned(),
        image_border: "#bd9f89".to_owned(),
        hover_dropshadow: "rgb(163, 122, 93, 0.5)".to_owned(),
        navbar_line: "#7a0002".to_owned(),
        logo: "#7a0002".to_owned(),
        hamburger_menu: "#7a0002".to_owned()
    }
}

pub(crate) fn read_theme(file_path: &str) -> Theme {
    todo!()
}

#[hook]
pub(crate) fn use_theme() -> ThemeContext {
    use yew::use_context;
    use_context::<ThemeContext>().unwrap()
}
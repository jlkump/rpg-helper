use super::style::StyleSheet;

// TODO: Replace
struct html {

}

pub trait Display {
    fn html_display(s: StyleSheet) -> html;
}
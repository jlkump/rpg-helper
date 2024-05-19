use crate::data::meta_type::MetaType;

#[derive(Clone, Copy)]
struct SOMETHING;

#[derive(Clone, Copy)]
pub struct Style {
    text_style: Option<TextStyle>,
}

#[derive(Clone, Copy)]
pub struct TextStyle {
    pub text_color: SOMETHING,
    pub text_font: SOMETHING,
    pub text_bold: bool,
    pub text_italic: bool,
    pub text_size: usize,
}

impl Style {
    pub fn get_text_style(&self) -> Option<TextStyle> {
        self.text_style // Maybe revert to default on none?
    }

    pub fn get_subtext_color() -> SOMETHING {
        todo!()
    }

    pub fn get_bg_color() -> SOMETHING {
        todo!()
    }
}

pub struct StyleSheet {

}

impl StyleSheet {
    pub fn get_style_for(t: MetaType) -> Style {
        let t = t.get_type_name();
        todo!()
    }
}
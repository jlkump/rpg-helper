use yew::prelude::*;
use stylist::{css, yew::Global};

use crate::app::router::Router;

mod context;
mod gui;
mod router;
mod store;

pub fn run_app()
{
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let theme = rpg_helper::api::display::style::theme::Theme::default_light();
    html!
    {
        <>
            <Global css={css!(
                r#"
                    html, body
                    {

                        --background: ${background};
                        --paper: ${paper};
                        --primary: ${primary};
                        --secondary: ${secondary};
                        --tertiary: ${tertiary};
                        --accent: ${accent};

                        --text-default: ${text_default};
                        --text-primary: ${text_primary};
                        --text-secondary: ${text_secondary};
                        --text-tertiary: ${text_tertiary};
                        --text-accent: ${text_accent};

                        --paper-75: ${paper_75};
                        --paper-50: ${paper_50};
                        --paper-25: ${paper_25};

                        --primary-75: ${primary_75};
                        --primary-50: ${primary_50};
                        --primary-25: ${primary_25};

                        --secondary-75: ${secondary_75};
                        --secondary-50: ${secondary_50};
                        --secondary-25: ${secondary_25};

                        --tertiary-75: ${tertiary_75};
                        --tertiary-50: ${tertiary_50};
                        --tertiary-25: ${tertiary_25};

                        --accent-75: ${accent_75};
                        --accent-50: ${accent_50};
                        --accent-25: ${accent_25};

                        --text-default-75: ${text_default_75};
                        --text-default-50: ${text_default_50};
                        --text-default-25: ${text_default_25};

                        --text-primary-75: ${text_primary_75};
                        --text-primary-50: ${text_primary_50};
                        --text-primary-25: ${text_primary_25};

                        --text-secondary-75: ${text_secondary_75};
                        --text-secondary-50: ${text_secondary_50};
                        --text-secondary-25: ${text_secondary_25};

                        --text-tertiary-75: ${text_tertiary_75};
                        --text-tertiary-50: ${text_tertiary_50};
                        --text-tertiary-25: ${text_tertiary_25};

                        --text-accent-75: ${text_accent_75};
                        --text-accent-50: ${text_accent_50};
                        --text-accent-25: ${text_accent_25};
                    }
                "#, 
                background=theme.background, paper=theme.paper, primary=theme.primary, 
                secondary=theme.secondary, tertiary=theme.tertiary, accent=theme.accent,
                text_default=theme.text_default, text_primary=theme.text_primary, text_secondary=theme.text_secondary,
                text_tertiary=theme.text_tertiary, text_accent=theme.text_accent,

                paper_75=theme.paper_75, paper_50=theme.paper_50, paper_25=theme.paper_25,
                primary_75=theme.primary_75, primary_50=theme.primary_50, primary_25=theme.primary_25,
                secondary_75=theme.secondary_75, secondary_50=theme.secondary_50, secondary_25=theme.secondary_25,
                tertiary_75=theme.tertiary_75, tertiary_50=theme.tertiary_50, tertiary_25=theme.tertiary_25,
                accent_75=theme.accent_75, accent_50=theme.accent_50, accent_25=theme.accent_25,

                text_default_75=theme.text_default_75, text_default_50=theme.text_default_50, text_default_25=theme.text_default_25,
                text_primary_75=theme.text_primary_75, text_primary_50=theme.text_primary_50, text_primary_25=theme.text_primary_25,
                text_secondary_75=theme.text_secondary_75, text_secondary_50=theme.text_secondary_50, text_secondary_25=theme.text_secondary_25,
                text_tertiary_75=theme.text_tertiary_75, text_tertiary_50=theme.text_tertiary_50, text_tertiary_25=theme.text_tertiary_25,
                text_accent_75=theme.text_accent_75, text_accent_50=theme.text_accent_50, text_accent_25=theme.text_accent_25,
            )} />
            <Router/>
        </>
    }
}
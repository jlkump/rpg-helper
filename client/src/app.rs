use yew::prelude::*;
use stylist::{css, yew::Global};

pub mod gui;
pub mod router;
pub mod store;

pub fn run_app()
{
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
                        --text: ${text},
                        --text-accent: ${text_accent};
                        --background: ${background};
                        --primary: ${primary};
                        --secondary: ${secondary};
                        --accent: ${accent};

                        --text-minor-faint: ${text_minor_faint};
                        --text-medium-faint: ${text_medium_faint};
                        --text-major-faint: ${text_major_faint};
                        --text-max-faint: ${text_max_faint};
                    }
                "#, 
                text=theme.text, text_accent=theme.text_accent, background=theme.background,
                primary=theme.primary, secondary=theme.secondary, accent=theme.accent,
                text_minor_faint=theme.text_minor_faint, text_medium_faint=theme.text_medium_faint,
                text_major_faint=theme.text_major_faint, text_max_faint=theme.text_major_faint
            )} />

            <div class="test-container">
                <div class="panel">
                </div>
            </div>
        </>
    }
}
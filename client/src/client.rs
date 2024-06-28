use stylist::{css, yew::Global};
use yew::prelude::*;

use crate::gui::contexts::theme::{use_theme, ThemeProvider};
use crate::router::Router;


pub fn run_app() {
    yew::Renderer::<Root>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let theme = use_theme();
    // TODO: Define text sizes

    html! {
        <>
            <Global css={css!(
                r#"
                    html, body {
                        background-color: ${bg};
                        color: ${text_default};
                        margin: 0px;
                        height: 100%;
                    }

                    h1 {
                        color: ${h1};
                        font-size: 2.5em;
                        margin-top: 10px;
                        margin-bottom: 10px;
                    }

                    h2 {
                        color: ${h2};
                        font-size: 2em;
                        margin-top: 5px;
                        margin-bottom: 5px;
                    }

                    h3 {
                        color: ${h3};
                        font-size: 1.75em;
                        margin-top: 2px;
                        margin-bottom: 2px;
                    }

                    h4 {
                        color: ${h4};
                        font-size: 1.5em;
                        margin-top: 2px;
                        margin-bottom: 2px;
                    }

                    h5 {
                        color: ${h5};
                        font-size: 1.25em;
                        margin-top: 2px;
                        margin-bottom: 2px;
                    }

                    h6 {
                        color: ${h6};
                        font-size: 1em;
                        margin-top: 2px;
                        margin-bottom: 2px;
                    }

                    hr {
                        border-top: 3px solid ${border_light};
                        margin: 0;
                    }
                    
                    a {
                        text-decoration: none;
                        color: ${link};
                    }

                    a:hover {
                        color: ${link_highlight};
                    }

                    button {
                        background: ${button};
                        border: 2px none;
                        border-radius: 4px;
                        padding: 8px;
                        cursor: pointer;
                        color: ${text_invert};
                        font-family: "Times New Roman", Times, serif;
                        font-weight: bold;
                        font-size: 1em;
                        margin: 5px;
                        outline: none;
                    }

                    button:hover {
                        background: ${button_hover};
                    }

                    button:active {
                        background: ${button_pressed};
                    }

                    button:disabled {
                        background: ${button_disabled};
                    }

                    input {
                        background: ${bg};
                        border: 2px solid ${unfocused};
                        border-radius: 2px;
                        font-size: 1em;
                        padding: 4px;
                        outline: none;
                    }

                    input:focus {
                        border: 2px solid ${focus};
                        border-radius: 2px;
                    }

                    input[type=submit] {
                        background: ${button};
                        border: 0px;
                        border-radius: 4px;
                        color: ${text_invert};
                    }

                    input[type=submit]:hover {
                        background: ${button_hover};
                        border: 0px;
                        border-radius: 4px;
                        color: ${text_invert};
                    }

                    input[type=submit]:focus {
                        background: ${button_pressed};
                        border: 0px;
                        border-radius: 4px;
                        color: ${text_invert};
                    }

                    /* width */
                    ::-webkit-scrollbar {
                        width: 10px;
                        height: 10px;
                    }

                    /* Track */
                    ::-webkit-scrollbar-track {
                        background: rgb(0, 0, 0, 0);
                    }

                    /* Handle */
                    ::-webkit-scrollbar-thumb {
                        background: ${scroll_bar};
                    }

                    /* Handle on hover */
                    ::-webkit-scrollbar-thumb:hover {
                        background: ${scroll_hover};
                    }

                    ::-webkit-scrollbar-corner {
                        background: transparent;
                    }

                    -webkit-user-select: none; /* Safari */
                    -ms-user-select: none; /* IE 10 and IE 11 */
                    user-select: none; /* Standard syntax */

                "#, bg = theme.paper, h1 = theme.h1, h2 = theme.h2, 
                h3 = theme.h3, h4 = theme.h4, h5 = theme.h5, h6 = theme.h6,
                text_default = theme.text_default, text_invert = theme.text_invert,
                button = theme.button_color, 
                button_hover = theme.button_color_hover,
                button_pressed = theme.button_color_press,
                button_disabled = theme.button_color_disabled,
                unfocused = theme.border_light,
                focus = theme.h1,
                scroll_bar = theme.scroll_bar, scroll_hover = theme.scroll_bar_hover,
                border_light=theme.border_light, link = theme.text_link, link_highlight = theme.text_link_highlight
            )} />
            <Router/>
        </>
    }
}

#[function_component(Root)]
fn root() -> Html {
    // TODO: Define a UserProvider for providing the user context to each element
    // This will make it much easier to know if a user is logged in or not
    // This will also probably replace the theme provider as the User defines the theme used
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }
}
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::app::{context::focus::{use_focus, FocusProvider}, gui::molecules::navbar::Navbar};

pub mod editor;
pub mod general;

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(BasePage)]
pub fn base_page(props: &Props) -> Html
{
    html!
    {
        <FocusProvider>
            <InnerBasePage children={props.children.clone()} class={props.class.clone()} style={props.style.clone()} />
        </FocusProvider>
    }
}

#[styled_component(InnerBasePage)]
fn inner_base_page(props: &Props) -> Html
{
    let inner = css!(
        r#"
            .page
            {
                position: absolute;
                top: 0;
                left: 0;
                width: 100vw;
                height: 100vh;
                z-index: -1;
            }

            .fullpage-container
            {
                width: 100%; 
                height: max(calc(100vh - 60px), auto);
                padding: 10px;
            }
        "#
    );
    
    let cs = css!(
        r#"
            .palette
            {
                width: 13vw;
                height: 25vh;
                background-color: var(--paper);
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
            }
        "#
    );

    let fctx = use_focus();
    html!
    {
        <div class={inner}>
            <Navbar />
            <div class={classes!("fullpage-container", props.class.clone())} style={props.style.clone()}>
                {props.children.clone()}
            </div>
            <div class={"page"} onclick={Callback::from(move |_| { fctx.clear_focus(); })}></div>
            <div class={cs} style="display: flex;">
                <div style="display: flex; flex-direction: column;">
                    <div class="palette" style="background-color: var(--paper);">
                        <p style="color: var(--text-default)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--paper-75);">
                        <p style="color: var(--text-default)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--paper-50);">
                        <p style="color: var(--text-default)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--paper-25);">
                        <p style="color: var(--text-default)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-default-25)">{"Rpg Helper"}</p>
                    </div>
                </div>
                <div style="display: flex; flex-direction: column;">
                    <div class="palette" style="background-color: var(--primary);">
                        <p style="color: var(--text-primary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--primary-75);">
                        <p style="color: var(--text-primary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--primary-50);">
                        <p style="color: var(--text-primary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--primary-25);">
                        <p style="color: var(--text-primary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-primary-25)">{"Rpg Helper"}</p>
                    </div>
                </div>
                <div style="display: flex; flex-direction: column;">
                    <div class="palette" style="background-color: var(--secondary);">
                        <p style="color: var(--text-secondary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--secondary-75);">
                        <p style="color: var(--text-secondary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--secondary-50);">
                        <p style="color: var(--text-secondary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--secondary-25);">
                        <p style="color: var(--text-secondary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-secondary-25)">{"Rpg Helper"}</p>
                    </div>
                </div>
                <div style="display: flex; flex-direction: column;">
                    <div class="palette" style="background-color: var(--tertiary);">
                        <p style="color: var(--text-tertiary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--tertiary-75);">
                        <p style="color: var(--text-tertiary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--tertiary-50);">
                        <p style="color: var(--text-tertiary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--tertiary-25);">
                        <p style="color: var(--text-tertiary)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-tertiary-25)">{"Rpg Helper"}</p>
                    </div>
                </div>

                <div style="display: flex; flex-direction: column;">
                    <div class="palette" style="background-color: var(--accent);">
                        <p style="color: var(--text-accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-25)">{"Rpg Helper"}</p>
                    </div>
                    <div class="palette" style="background-color: var(--accent-75);">
                        <p style="color: var(--text-accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-25)">{"Rpg Helper"}</p>
                    </div>
                    <div class="palette" style="background-color: var(--accent-50);">
                        <p style="color: var(--text-accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-25)">{"Rpg Helper"}</p>
                    </div>
                    <div class="palette" style="background-color: var(--accent-25);">
                        <p style="color: var(--text-accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--text-accent-25)">{"Rpg Helper"}</p>
                    </div>
                </div>
                <div style="display: flex; flex-direction: column;">
                    <div class="palette" style="background-color: var(--paper);">
                        <p style="color: var(--accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--paper-75);">
                        <p style="color: var(--accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--paper-50);">
                        <p style="color: var(--accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--paper-25);">
                        <p style="color: var(--accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-25)">{"Rpg Helper"}</p>
                    </div>
                </div>

                <div style="display: flex; flex-direction: column;">
                    <div class="palette" style="background-color: var(--primary);">
                        <p style="color: var(--accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--primary-75);">
                        <p style="color: var(--accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--primary-50);">
                        <p style="color: var(--accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-25)">{"Rpg Helper"}</p>
                    </div>

                    <div class="palette" style="background-color: var(--primary-25);">
                        <p style="color: var(--accent)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-75)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-50)">{"Rpg Helper"}</p>
                        <p style="color: var(--accent-25)">{"Rpg Helper"}</p>
                    </div>
                </div>

            </div>

        </div>
    }
}
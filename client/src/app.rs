use yew::prelude::*;

pub mod gui;
pub mod router;

pub fn run_app()
{
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html!
    {
        <div class="test-container">
            <img src="/assets/Dice RPG Icon.svg"/>
            <div style="display: flex; flex-direction: column;">
                <h1>{"RPG Helper"}</h1>
                <h2>{"RPG Helper"}</h2>
                <h3>{"RPG Helper"}</h3>
                <h4>{"RPG Helper"}</h4>
                <h5>{"RPG Helper"}</h5>
                <h6>{"RPG Helper"}</h6>
            </div>
            <div class="panel">
                <p>
                    <a>{"Hyper "}</a>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                </p>
            </div>
        </div>
    }
}
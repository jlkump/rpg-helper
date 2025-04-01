use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub color: AttrValue,
    pub open: bool,
    #[prop_or_default]
    pub style: AttrValue,
}

#[styled_component(HamburgerMenu)]
pub fn hamburger_menu(props: &Props) -> Html {
    // Credit to https://codepen.io/designcouch/pen/ExvwPY for hamburger menu

    let style = css!(
        r#"
            #nav-icon3 {
                width: 40px;
                height: 30px;
                position: relative;
                margin: 0;
                -webkit-transform: rotate(0deg);
                -moz-transform: rotate(0deg);
                -o-transform: rotate(0deg);
                transform: rotate(0deg);
                -webkit-transition: .5s ease-in-out;
                -moz-transition: .5s ease-in-out;
                -o-transition: .5s ease-in-out;
                transition: .5s ease-in-out;
                cursor: pointer;
            }

            #nav-icon3 span {
                display: block;
                position: absolute;
                height: 5px;
                width: 100%;
                background: ${color};
                border-radius: 9px;
                opacity: 1;
                left: 0;
                -webkit-transform: rotate(0deg);
                -moz-transform: rotate(0deg);
                -o-transform: rotate(0deg);
                transform: rotate(0deg);
                -webkit-transition: .25s ease-in-out;
                -moz-transition: .25s ease-in-out;
                -o-transition: .25s ease-in-out;
                transition: .25s ease-in-out;
            }

            #nav-icon3 span:nth-child(1) {
                top: 0px;
            }

            #nav-icon3 span:nth-child(2),#nav-icon3 span:nth-child(3) {
                top: 12px;
            }

            #nav-icon3 span:nth-child(4) {
                top: 24px;
            }

            #nav-icon3.open span:nth-child(1) {
                top: 18px;
                width: 0%;
                left: 50%;
            }

            #nav-icon3.open span:nth-child(2) {
                -webkit-transform: rotate(45deg);
                -moz-transform: rotate(45deg);
                -o-transform: rotate(45deg);
                transform: rotate(45deg);
            }

            #nav-icon3.open span:nth-child(3) {
                -webkit-transform: rotate(-45deg);
                -moz-transform: rotate(-45deg);
                -o-transform: rotate(-45deg);
                transform: rotate(-45deg);
            }

            #nav-icon3.open span:nth-child(4) {
                top: 18px;
                width: 0%;
                left: 50%;
            }
        "#,
        color=props.color
    );
    
    html! {
        <div class={style}>
            <div id="nav-icon3" class={if props.open {"open"} else {""}}>
                <span></span>
                <span></span>
                <span></span>
                <span></span>
            </div>
        </div>
    }
}
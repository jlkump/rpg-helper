use stylist::yew::styled_component;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use stylist::Style;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {

}

#[styled_component(CharacterSelectDropdown)]
pub fn character_select_dropdown(props: &Props) -> Html {
    html! {
        <div class={get_character_select_style()}>
            <h3 style="margin-right: 5px">{"Current Character"}</h3>
            <Icon icon_id={IconId::FeatherChevronDown}/>
            <ul class={get_character_select_dropdown_style()}>
                <li>{"Item 1"}</li>
                <li>{"Item 2"}</li>
                <li>{"Item 3"}</li>
            </ul>
        </div>
    }
}


fn get_character_select_style() -> Style {
    Style::new(
        r#"
            /* background: #ccb897; */
            color: #7a0002;
            border-radius: 20px;
            padding: 10px;

            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;
            justify-content: space-between;
            align-items: center;

            -webkit-user-select: none; /* Safari */
            -ms-user-select: none; /* IE 10 and IE 11 */
            user-select: none; /* Standard syntax */

            position: relative;
            
            cursor: pointer;
        "#
    ).expect("Failed to create logo style")
}

fn get_character_select_dropdown_style() -> Style {
    Style::new(
        r#"
            margin-top: 5px;
            list-style-type: none;
            height: 0px;
            overflow: hidden;
        
            position: absolute;  /* <-- added declarations */
            left: 0; top: 100%;  /*     here               */
            width: 100%;         /*     and here...        */
        	
            -webkit-transition: height 1s ease;
            -moz-transition: height 1s ease;
            -o-transition: height 1s ease;
            -ms-transition: height 1s ease;
            transition: height 1s ease;
        "#
    ).expect("Failed to create character dropdown style")
}
// Will display a given meta type inst in table format, where the given inst is a
// list of some type. If the list contains other insts, then the table
// will display given fields as columns with the columns named by the fields.
// Otherwise, the table will be two columns from name to value

use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub columns: Vec<String>,
    pub column_names: Vec<String>
}

#[styled_component(Table)]
pub fn table(props: &Props) -> Html {
    // TODO: Use actual data
    // let character = use_context::<CharacterContext>().unwrap().character_view;
    let style = css!(
        r#"
            display: grid;
            
        "#
    );
    html! {

    }
}
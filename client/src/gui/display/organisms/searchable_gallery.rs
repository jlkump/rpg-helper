use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use validator::ValidationErrors;
use yew::{platform::spawn_local, prelude::*};
use stylist::yew::styled_component;
use yew_icons::{Icon, IconId};

use crate::{api::user_api::Error, gui::{contexts::theme::use_theme, display::atoms::{form_input::FormInput, loading::SkeletonPane, scroll_div::ScrollDiv, tooltip::Tooltip}}, model::schema::{SearchSchema, SortOptions}};

pub fn test_gallery(create: Option<Callback<()>>) -> Html {
    let default = TestDataFetcher {
        test_data: vec![
            TestData::get_default(),
            TestData::get_default(),
            TestData::get_default(),
            TestData::get_default(),
            TestData::get_default(),
            TestData::get_default(),
            TestData::get_default(),
            TestData::get_default(),
        ]
    };
    html! {
        <Gallery<TestData, TestDataFetcher> data_fetcher={default} {create} />
    }
}

#[derive(PartialEq, Debug, Clone)]
struct TestDataFetcher {
    test_data: Vec<Rc<RefCell<TestData>>>,
}

impl DataFetch<TestData> for TestDataFetcher {
    async fn fetch_data(&self, conditions: SearchSchema) -> Result<Vec<Rc<RefCell<TestData>>>, Error> {
        Ok(self.test_data.clone())
    }
}

#[derive(PartialEq, Debug, Clone)]
struct TestData {
    display_name: String,
    display_img: String,
    description: String,
    description_brief: String,
    tags: Vec<String>,
    version: String,
}

impl TestData {
    fn get_default() -> Rc<RefCell<TestData>> {
        Rc::new(RefCell::new(TestData { 
            display_name: "Ars Magica Core Ruleset".to_string(), 
            display_img: "/img/generic/ars-magica-logo-icon.png".to_string(), 
            description: "A default description with lots of words...".to_string(), 
            description_brief: "A default description brief".to_string(), 
            tags: vec!["default".to_string()], 
            version: "v 1.0".to_string(), 
        }))
    }
}

impl GalleryData for TestData {
    fn get_display_name(&self) -> &str {
        &self.display_name
    }

    fn get_display_img_src(&self) -> &str {
        &self.display_img
    }

    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_brief(&self) -> &str {
        &self.description_brief
    }

    fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    fn get_version(&self) -> Option<&str> {
        Some(&self.version)
    }

    fn get_display_buttons(&self) -> Html {
        html! {
            <div>
                <button>{"Click to edit"}</button>
            </div>
        }
    }

    fn get_last_update_data(&self) -> &str {
        todo!()
    }
}

pub trait GalleryData {
    fn get_display_name(&self) -> &str;
    fn get_display_img_src(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_description_brief(&self) -> &str;
    fn get_tags(&self) -> Vec<String>;
    fn get_version(&self) -> Option<&str>;
    fn get_last_update_data(&self) -> &str;
    fn get_display_buttons(&self) -> Html; // This is where button options are placed, such as "Create Game with Ruleset" and such
}

pub trait DataFetch<T>
where
    T: PartialEq + Clone + GalleryData
{
    async fn fetch_data(&self, conditions: SearchSchema) -> Result<Vec<Rc<RefCell<T>>>, Error>;
}

#[derive(Properties, PartialEq, Clone)]
pub struct GalleryProps<T, D>
where
    T: PartialEq + Clone + GalleryData,
    D: PartialEq + Clone + DataFetch<T>,
{
    data_fetcher: D,
    #[prop_or_default]
    create: Option<Callback<()>>, // If there is an option to create a new thing for the gallery
    #[prop_or_default]
    _phantom: PhantomData<T>, // Marks that GalleryProps uses T but does not directly store T
}

#[styled_component(Gallery)]
pub fn gallery<T, D>(props: &GalleryProps<T, D>) -> Html 
where
    T: PartialEq + Clone + GalleryData + 'static,
    D: PartialEq + Clone + DataFetch<T> + 'static
{
    let data = use_state(|| None);
    let sorting = use_state(|| SortOptions::Alphabetical);
    let selected_gallery_component = use_state(|| None);

    let gallery_brief_onclick = {
        let selected_gallery_component = selected_gallery_component.clone();
        Callback::from(move |e: Rc<RefCell<T>>| selected_gallery_component.set(Some(e.clone()))) 
    };

    use_effect_with((), {
        let data = data.clone();
        let sorting = sorting.clone();
        let data_fetcher = props.data_fetcher.clone();
        move |_| {
            spawn_local(async move {
    
                let res = data_fetcher.fetch_data(SearchSchema { search_string: "".to_string(), sorting: *sorting }).await;
                match res {
                    Ok(d) => data.set(Some(d)),
                    Err(_e) => {
                        todo!()
                    },
                }
            });
        }
    });

    let theme = use_theme();
    let css = css!(
        r#"
            width: 100%;
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;

            .search {
                flex: 50%;
                margin: 10px;
                padding: 8px;
                border: 3px solid ${border};
                background: ${paper_dark};
            }

            .search-bar {
                display: flex; 
                align-items: center;
            }

            .search-bar > * {
                margin-left: 20px;
            }
            
            .search-bar .selected {
                color: ${selected_color};
            }

            .search-icon {
                color: ${color};
                cursor: pointer;
            }

            .search-icon.create {
                margin-left: auto;
                margin-right: 20px;
            }

            .search-icon:hover {
                color: ${hover_color};
            }

            .search-gallery {
                display: flex; 
                flex-direction: row; 
                flex-wrap: wrap; 
                align-items: center;
                justify-content: center;


                overflow-y: scroll;
                min-height: 40vh;
                max-height: 80vh;
            }

            .details {
                margin: 20px; 
                flex: 35%; 
                display: flex; 
                flex-direction: column; 
                align-items: center;
            }

            @media screen and (max-width: 600px) {
                flex-wrap: wrap-reverse;
            }
        "#,
        paper_dark=theme.paper_dark,
        border=theme.border_light,
        color=theme.text_default,
        selected_color=theme.text_colored,
        hover_color=theme.text_colored_highlight,
    );

    html! {
        <div class={css}>
            <div class="search">
                <div class="search-bar">
                    <FormInput<String> 
                        input_type="search" 
                        name="search" 
                        placeholder="Search Rulesets"
                        input_ref={NodeRef::default()}
                        onchange={Callback::from(|_| {})}
                        onblur={Callback::from(|(_, _)| {})}
                        to_type={Callback::from(|_| { String::from("") })}
                        errors={Rc::new(RefCell::new(ValidationErrors::new()))}
                    />
                    {get_sort_options(sorting.clone(), props.create.clone())}
                </div>
                <div class="search-gallery">
                    if let Some(data) = &(*data) {
                        {into_gallery_display(data, gallery_brief_onclick)}
                    } else {
                        <SkeletonPane />
                    }
                </div>
            </div>
            <ScrollDiv class="details">
                if let Some(selected) = &(*selected_gallery_component) {
                    <GalleryDetails<T> data={selected.clone()}/>
                } else {
                    <h3>{"Details"}<hr/></h3>
                    <div>
                    </div>
                }
            </ScrollDiv>
        </div>
    }
}

fn into_gallery_display<T>(data: &Vec<Rc<RefCell<T>>>, onclick: Callback<Rc<RefCell<T>>>) -> Vec<Html> 
where
    T: PartialEq + Clone + GalleryData + 'static,
{
    let mut res = vec![];
    for d in data {
        res.push(
            html! {
                <GalleryBrief<T> data={d.clone()} onclick={onclick.clone()}/>
            }
        );
    }
    res
}

fn get_sort_options(sorting: UseStateHandle<SortOptions>, create_new: Option<Callback<()>>) -> Html {
    let selected = &*sorting;
    let alphabetical_onclick = {
        let sorting = sorting.clone();
        Callback::from(move |_| { sorting.set(SortOptions::Alphabetical) })
    };
    let rev_alphabetical_onclick = {
        let sorting = sorting.clone();
        Callback::from(move |_| { sorting.set(SortOptions::ReverseAlphabetical) })
    };
    let recent_onclick = {
        let sorting = sorting.clone();
        Callback::from(move |_| { sorting.set(SortOptions::LastUpdated) })
    };
    let alpha_classes = if selected == &SortOptions::Alphabetical { "search-icon selected" } else { "search-icon" };
    let rev_alpha_classes = if selected == &SortOptions::ReverseAlphabetical { "search-icon selected" } else { "search-icon" };
    let latest_classes = if selected == &SortOptions::LastUpdated { "search-icon selected" } else { "search-icon" };
    html! {
        <>
            <Icon class={alpha_classes} icon_id={IconId::BootstrapSortAlphaDown} onclick={alphabetical_onclick}/>
            <Icon class={rev_alpha_classes} icon_id={IconId::BootstrapSortAlphaUp} onclick={rev_alphabetical_onclick}/>
            <Icon class={latest_classes} icon_id={IconId::LucideClock4} onclick={recent_onclick}/>
            if let Some(create_new) = create_new {
                <Icon class="create search-icon" icon_id={IconId::FeatherPlusSquare} onclick={Callback::from(move |_| {create_new.emit(())})}/>
            }
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct GalleryBriefProps<T>
where
    T: PartialEq + Clone + GalleryData + 'static
{
    data: Rc<RefCell<T>>,
    #[prop_or_default]
    onclick: Option<Callback<Rc<RefCell<T>>>>,
}

#[styled_component(GalleryBrief)]
pub fn gallery_brief<T>(props: &GalleryBriefProps<T>) -> Html 
where
    T: PartialEq + Clone + GalleryData + 'static
{
    let theme = use_theme();
    let css = css!(
        r#"
            width: 128px; 
            margin: 5px;
            align-self: center;
            text-align: center;
            text-wrap: wrap;

            position: relative;
            border: 3px solid transparent;
            cursor: pointer;
            border: 3px solid ${border};

            &:hover {
                border: 3px solid ${highlight};
            }

            p {
                margin-block-start: 0em;
                margin-block-end: 0.5em;
            }

            .text-header {
                position: absolute;
                bottom: 1px;
                width: 100%;
                background: ${bg};
            }

            img {
                width: 100%;
                height: 100%;
                object-fit: cover;
            }

        "#, 
        bg=theme.paper,
        border=theme.border_tooltip_light,
        highlight=theme.text_colored_highlight,
    );
    let data = props.data.as_ref().borrow();

    let onclick = if let Some(onclick) = &props.onclick {
        let onclick = onclick.clone();
        let data = props.data.clone();
        Some(Callback::from(move |_: MouseEvent| {
            let data = data.clone();
            onclick.emit(data);
        }))
    } else {
        None
    };

    html! {
        <div class={css} {onclick}>
            <Tooltip tooltip_content={html! {
                <div style="min-width: 128px; padding: 15px;">
                    <h5>{"Description"}<hr/></h5>
                    <p>{data.get_description_brief()}</p>
                </div>
            }}>
                <div style="width: 128px; height: 128px;">
                    <img src={data.get_display_img_src().to_string()} />
                    <div class="text-header">
                        <p>{data.get_display_name()}</p>
                    </div>
                </div>
            </Tooltip>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct GalleryDetailsProps<T>
where
    T: PartialEq + Clone + GalleryData + 'static
{
    data: Rc<RefCell<T>>,
}

#[styled_component(GalleryDetails)]
pub fn gallery_brief<T>(props: &GalleryDetailsProps<T>) -> Html 
where
    T: PartialEq + Clone + GalleryData + 'static
{
    let data = props.data.as_ref().borrow();
    html! {
        <div>
            <h3 style="align-self: center;">{data.get_display_name().to_string()}<hr/></h3>
            <div>
                <img src={data.get_display_img_src().to_string()} />
                
            </div>
            <div>
                <h5 style="position: relative;">{"Description"}<hr/><div style="position: absolute; right: 0; top: 0;"><i>{data.get_version()}</i></div></h5>
                {data.get_description()}
                <div>
                    {data.get_tags()}
                </div>
            </div>
        </div>
    }
}
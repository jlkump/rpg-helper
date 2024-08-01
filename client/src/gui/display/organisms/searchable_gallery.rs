use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use yew::{platform::spawn_local, prelude::*};
use stylist::yew::styled_component;

use crate::{api::user_api::Error, gui::{contexts::theme::use_theme, display::atoms::{loading::SkeletonPane, tooltip::Tooltip}}, model::schema::{SearchSchema, SortOptions}};

pub fn test_gallery() -> Html {
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
        <Gallery<TestData, TestDataFetcher> data_fetcher={default} />
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
            display_name: "Display Name".to_string(), 
            display_img: "/img/generic/Birb Wizard Transparent.png".to_string(), 
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
}

pub trait GalleryData {
    fn get_display_name(&self) -> &str;
    fn get_display_img_src(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_description_brief(&self) -> &str;
    fn get_tags(&self) -> Vec<String>;
    fn get_version(&self) -> Option<&str>;
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

    html! {
        <div style="width: 100%; padding: 20px; display: flex; flex-direction: row;">
            <div>
                <div style="display: flex; justify-content: space-evenly;">
                    <div>
                        {"Search"}
                    </div>
                    <div>
                        {"Sort Options"}
                    </div>
                    <div>
                        {"View Private Gallery"}
                    </div>
                </div>
                <div style="display: flex; flex-direction: row; flex-wrap: wrap; align-items: center;">
                    if let Some(data) = &(*data) {
                        {into_gallery_display(data)}
                    } else {
                        <SkeletonPane />
                    }
                </div>
            </div>
            <div>
                <h3>{"Details"}</h3>
                <div>
                </div>
            </div>
        </div>
    }
}

fn into_gallery_display<T>(data: &Vec<Rc<RefCell<T>>>) -> Vec<Html> 
where
    T: PartialEq + Clone + GalleryData + 'static,
{
    let mut res = vec![];
    for d in data {
        res.push(
            html! {
                <GalleryBrief<T> data={d.clone()}/>
            }
        );
    }
    res
}

#[derive(Properties, PartialEq, Clone)]
pub struct GalleryBriefProps<T>
where
    T: PartialEq + Clone + GalleryData
{
    data: Rc<RefCell<T>>,
    #[prop_or_default]
    onclick: Option<Callback<MouseEvent>>,
}

#[styled_component(GalleryBrief)]
pub fn gallery_brief<T>(props: &GalleryBriefProps<T>) -> Html 
where
    T: PartialEq + Clone + GalleryData
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

            .wrapper {
                border: 3px solid transparent;
                cursor: pointer;
                width: 128px; 
                height: 128px;
            }

            .wrapper:hover {
                border: 3px solid ${highlight};
            }

            .wrapper:focus {
                border: 3px solid ${focus_highlight};
            }

            img {
                width: 100%;
                height: 100%;
                object-fit: cover;
            }

        "#, 
        bg=theme.paper_dark,
        focus_highlight=theme.text_colored,
        highlight=theme.text_colored_highlight
    );
    let data = props.data.as_ref().borrow();
    html! {
        <Tooltip tooltip_content={html! {
            <div style="min-width: 128px;">
                <h3>{"Description"}<hr/></h3>
                <p>{data.get_description_brief()}</p>
            </div>
        }}>
            <div class={css}>
                <img src={data.get_display_img_src().to_string()} />
                <div>
                    <p>{data.get_display_name()}</p>
                </div>
            </div>
        </Tooltip>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct GalleryDetailsProps<T>
where
    T: PartialEq + Clone + GalleryData
{
    data: Rc<RefCell<T>>,
    #[prop_or_default]
    click_options: Option<Vec<(String, Callback<MouseEvent>)>>,
}

#[styled_component(GalleryDetails)]
pub fn gallery_brief<T>(props: &GalleryDetailsProps<T>) -> Html 
where
    T: PartialEq + Clone + GalleryData
{
    // TODO: also do tooltip
    html! {
        <div>
            <h1>{"TODO"}</h1>
        </div>
    }
}
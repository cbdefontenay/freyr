use crate::assets::tabs_styles::TABS_STYLES;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct TabsProps {
    pub city_names: Vec<String>,
    pub city_texts: Vec<String>,
    pub custom_titles: Option<Vec<Element>>,
    pub custom_texts: Option<Vec<Element>>,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let mut active_city_idx = use_signal(|| 0);

    let title = match &props.custom_titles {
        Some(custom_titles) => custom_titles.get(active_city_idx()).cloned(),
        None => Some(rsx! {
            h1 { class: "city-title", "{props.city_names[active_city_idx()]}" }
        }),
    };

    let text = match &props.custom_texts {
        Some(custom_texts) => custom_texts.get(active_city_idx()).cloned(),
        None => Some(rsx! {
            p { class: "city-text", "{props.city_texts[active_city_idx()]}" }
        }),
    };

    let style_tag = rsx! {
        style { "{TABS_STYLES}" }
    };

    rsx! {
        div {
            {style_tag}
            div { id: "tabs", class: "tabs-container",

                div { class: "tabs-navigation",
                    for (idx, city_name) in props.city_names.iter().enumerate() {
                        div {
                            class: "tab-item",
                            onclick: move |_| active_city_idx.set(idx),
                            "{city_name}"
                        }
                    }
                }

                div { class: "city-content",
                    {title}
                    {text}
                }
            }
        }
    }
}

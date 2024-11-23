use crate::assets::tabs_styles::TABS_STYLES;
use crate::enums::tabs_enums::TabsColor;
use crate::enums::tabs_enums::TabsProps;
use dioxus::prelude::*;

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let mut active_tab_idx = use_signal(|| 0);

    let title = match &props.custom_titles {
        Some(custom_titles) => custom_titles.get(active_tab_idx()).cloned(),
        None => Some(rsx! {
            h1 { class: "tab-title", "{props.tabs_names[active_tab_idx()]}" }
        }),
    };

    let text = match &props.custom_texts {
        Some(custom_texts) => custom_texts.get(active_tab_idx()).cloned(),
        None => Some(rsx! {
            p { class: "tab-text", "{props.tabs_texts[active_tab_idx()]}" }
        }),
    };

    let style_tag = rsx! {
        style { "{TABS_STYLES}" }
    };

    rsx! {
        div {
            {style_tag}
            div {
                id: "tabs",
                class: "tabs-container",

                // Tabs navigation
                div { class: "tabs-navigation",
                    for (idx, tab_name) in props.tabs_names.iter().enumerate() {
                        div {
                            class: match &props.custom_color {
                                Some(color) => format!("tab-item {}", color.to_css_class()),
                                None => String::from("tab-item"),
                            },
                            onclick: move |_| active_tab_idx.set(idx),
                            "{tab_name}"
                        }
                    }
                }

                // Tabs content
                div { class: "tab-content",
                    {title}
                    {text}
                }
            }
        }
    }
}

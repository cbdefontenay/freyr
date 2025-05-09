use crate::assets::tabs_styles::{TABS_SECONDARY_STYLES, TABS_STYLES};
use crate::enums::tabs_enums::{TabsColor, TabsProps};
use crate::TabsSecondaryProps;
use dioxus::prelude::*;

/// Tabs usage example:
/// ```rust
/// const IMAGE: Asset = asset!("/assets/one.jpg");
/// #[component]
/// pub fn Home() -> Element {
///
///     let tabs_names = vec![
///         String::from("Puffin"),
///         String::from("Otto I"),
///         String::from("Barbarossa"),
///     ];
///
///     let custom_text = vec![
///         rsx! {
///             div {
///                 class: "flex flex-col items-center w-96",
///                 h1 { class: "text-2xl font-bold text-gray-800 my-4", "Icelandic Bird" }
///                 img { src: IMAGE, class: "w-64 h-64 object-cover", alt: "Icelandic Bird" }
///                 p { class: "mt-4 text-gray-600 text-center",
///                     "The puffin, often called the 'clown of the sea,' is an iconic Icelandic bird. Known for its colorful beak and charismatic presence, the puffin thrives along Iceland's coastal cliffs, where it nests in burrows and feeds on small fish."
///                 }
///             }
///         },
///         rsx! {
///             div {
///                 class: "flex flex-col items-start w-96",
///                 p { class: "mt-4 text-gray-600",
///                     "Otto I (912–973), known as Otto the Great, was the first Holy Roman Emperor. He was a key figure in uniting Germany and Italy, strengthening the church, and establishing the Ottonian dynasty, laying the groundwork for the medieval Holy Roman Empire."
///                 }
///             }
///         },
///         rsx! {
///             div {
///                 class: "flex flex-col items-start w-96",
///                 p { class: "mt-4 text-gray-600",
///                     "Frederick Barbarossa (1122–1190) was one of the most famous Holy Roman Emperors. Renowned for his ambition and charisma, he sought to restore the glory of the Roman Empire, leading campaigns in Italy and participating in the Third Crusade."
///                 }
///             }
///         },
///     ];
///
///     rsx! {
///         div { class: "flex flex-col items-center justify-center mt-20 w-full",
///             div { class:"mt-20 flex flex-col items-center justify-center",
///                 TabsPrimary {
///                     tabs_names: tabs_names,
///                     custom_texts: Some(custom_text),
///                     custom_color: TabsColor::Freyr
///                 }
///             }
///         }
///     }
/// }
/// ```
/// The use of tailwind is not required, but as you see, you have the option.
#[component]
pub fn TabsPrimary(props: TabsProps) -> Element {
    let mut active_tab_idx = use_signal(|| 0);

    let text = match &props.custom_texts {
        Some(custom_texts) => custom_texts.get(active_tab_idx()).cloned(),
        None => Some(rsx! {}),
    };

    let custom_styles = if let Some(TabsColor::Custom(color)) = &props.custom_color {
        format!(
            r#"
            .custom-tab-item {{
                color: {color};
                border-bottom-color: transparent;
            }}
            .custom-tab-item:hover {{
                color: {color};
                border-bottom-color: {color};
            }}
            .custom-tab-item.active-tab {{
                color: {color};
                border-bottom-color: {color};
            }}
            "#
        )
    } else {
        String::new()
    };

    let style_tag = rsx! {
        style {
            "{TABS_STYLES}"
            "{custom_styles}"
        }
    };

    rsx! {
        div {
            {style_tag}
            div { id: "tabs", class: "tabs-container",
                div { class: "tabs-navigation",
                    for (idx , tab_name) in props.tabs_names.iter().enumerate() {
                        div {
                            class: format!(
                                "tab-item {} {} {}",
                                if matches!(&props.custom_color, Some(TabsColor::Custom(_))) {
                                    "custom-tab-item"
                                } else {
                                    ""
                                },
                                props.custom_color.as_ref().map_or("", |color| color.to_css_class()),
                                if active_tab_idx() == idx { "active-tab" } else { "" },
                            ),
                            onclick: move |_| active_tab_idx.set(idx),
                            "{tab_name}"
                        }
                    }
                }

                div { class: "tab-content", {text} }
            }
        }
    }
}

/// Code implementation:
/// ```rust
///     let tabs_names = vec![
///         String::from("Puffin"),
///         String::from("Otto I"),
///         String::from("Barbarossa"),
///     ];
///
/// rsx! {
///     div { class: "flex flex-col items-center justify-center mt-10 w-full",
///         // Note that you have to implement colors, width and css properties 
///         // like you would do in a normal CSS project.
///         TabsSecondary {
///             tabs_names,
///             custom_texts: Some(custom_text),
///             tab_max_width: Some(String::from("500px")),
///             tab_bottom_radius: Some(String::from("0em 0em 1em 1em")),
///             tab_header_hover: Some(String::from("#624b6b")),
///             header_bg_color: Some("#bf7ed9".to_string()),
///             header_text_color: Some("#37064a".to_string()),
///             active_bg_color: Some("#d5b8e0".to_string()),
///             active_text_color: Some("#2a0738".to_string()),
///             tab_shadow: None,
///         }
///     }
/// }
/// ```
#[component]
pub fn TabsSecondary(props: TabsSecondaryProps) -> Element {
    let custom_style = format!(
        r#"
    :root {{
        --tab-max-width: {};
        --tab-header-hover: {};
        --tab-header-bg: {};
        --tab-header-text: {};
        --tab-active-bg: {};
        --tab-active-text: {};
        --tab-radius: {};
        --tab-shadow: {};
    }}
    "#,
        props
            .tab_max_width
            .clone()
            .unwrap_or_else(|| "700px".to_string()),
        props
            .tab_header_hover
            .clone()
            .unwrap_or_else(|| "#e5e5e5".to_string()),
        props
            .header_bg_color
            .clone()
            .unwrap_or_else(|| "#e5e5e5".to_string()),
        props
            .header_text_color
            .clone()
            .unwrap_or_else(|| "#7f7f7f".to_string()),
        props
            .active_bg_color
            .clone()
            .unwrap_or_else(|| "#ffffff".to_string()),
        props
            .active_text_color
            .clone()
            .unwrap_or_else(|| "#000000".to_string()),
        props
            .tab_radius
            .clone()
            .unwrap_or_else(|| "1em 1em 1em 1em)".to_string()),
        props.tab_shadow.clone().unwrap_or_else(|| "".to_string()),
    );

    let style_tag = rsx! {
        style { "{TABS_SECONDARY_STYLES}" }
        style { "{custom_style}" }
    };

    let texts = props.custom_texts.clone().unwrap_or_default();

    rsx! {
        div {
            {style_tag}
            div { class: "tabs",
                for (idx , name) in props.tabs_names.iter().enumerate() {
                    input {
                        r#type: "radio",
                        class: "input",
                        id: format!("tab-{idx}"),
                        name: "tabs",
                        checked: idx == 0,
                    }
                    label {
                        class: "label label-default",
                        r#for: format!("tab-{idx}"),
                        "{name}"
                    }
                    div { class: "panel",
                        {texts.get(idx).cloned().unwrap_or(rsx! {
                            div {}
                        })}
                    }
                }
            }
        }
    }
}

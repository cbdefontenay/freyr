use crate::assets::tabs_styles::TABS_STYLES;
use crate::enums::tabs_enums::{TabsColor, TabsProps};
use dioxus::prelude::*;

/// Tabs usage example:
/// ```rust
/// const IMAGE: Asset = asset!("/assets/one.jpg");
/// #[component]
/// pub fn Home() -> Element {
///     let title_one = String::from("First Accordion");
///     let first_text = String::from("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.");
///
///     let tabs_names = vec![
///         String::from("C#"),
///         String::from("Rust"),
///         String::from("TypeScript"),
///     ];
///
///     let custom_titles = vec![
///         rsx! { h1{class:"text-red-800 text-2xl mt-4 flex w-full justify-center items-center", "My First title"}},
///         rsx! { h1{class:"text-blue-800 text-2xl mt-4", "My Second title"}},
///         rsx! { h1{class:"text-red-800 text-2xl mt-4", "My Third title"}},
///     ];
///
///     let custom_text = vec![
///         rsx! {
///             div {
///                 class: "flex flex-col items-center",
///                 img { src: IMAGE, class: "w-64 h-64 object-cover", alt: "Icelandic Bird" }
///                 p { class: "mt-4 text-gray-600 text-center",
///                     "The puffin, often called the 'clown of the sea,' is an iconic Icelandic bird. Known for its colorful beak and charismatic presence, the puffin thrives along Iceland's coastal cliffs, where it nests in burrows and feeds on small fish."
///                 }
///             }
///         },
///         rsx! {
///             div {
///                 class: "flex flex-col items-start",
///                 p { class: "mt-4 text-gray-600",
///                     "Otto I (912–973), known as Otto the Great, was the first Holy Roman Emperor. He was a key figure in uniting Germany and Italy, strengthening the church, and establishing the Ottonian dynasty, laying the groundwork for the medieval Holy Roman Empire."
///                 }
///             }
///         },
///         rsx! {
///             div {
///                 class: "flex flex-col items-start",
///                 p { class: "mt-4 text-gray-600",
///                     "Frederick Barbarossa (1122–1190) was one of the most famous Holy Roman Emperors. Renowned for his ambition and charisma, he sought to restore the glory of the Roman Empire, leading campaigns in Italy and participating in the Third Crusade."
///                 }
///             }
///         },
///     ];
///
///     rsx! {
///         div { class: "flex flex-col items-center justify-center mt-20",
///             div { class:"mt-20 flex flex-col items-center justify-center",
///                 Tabs {
///                     tabs_names: tabs_names,
///                     custom_titles: Some(custom_titles),
///                     custom_texts: Some(custom_text),
///                     custom_color: TabsColor::Custom("#ad1fb8")
///                 }
///             }
///         }
///     }
/// }
/// ```
/// The use of tailwind is not required, but as you see, you have the option.
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
            div {
                id: "tabs",
                class: "tabs-container",
                div { class: "tabs-navigation",
                    for (idx, tab_name) in props.tabs_names.iter().enumerate() {
                        div {
                            class: format!(
                                "tab-item {} {}",
                                if matches!(&props.custom_color, Some(TabsColor::Custom(_))) {
                                    "custom-tab-item"
                                } else {
                                    ""
                                },
                                if active_tab_idx() == idx {
                                    "active-tab"
                                } else {
                                    ""
                                }
                            ),
                            onclick: move |_| active_tab_idx.set(idx),
                            "{tab_name}"
                        }
                    }
                }

                div { class: "tab-content",
                    {title}
                    {text}
                }
            }
        }
    }
}
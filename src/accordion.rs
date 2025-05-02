use crate::assets::accordion_styles::{ACCORDION_NEW_STYLES, ACCORDION_STYLES};
use crate::enums::accordion_enums::{AccordionCustomProps, AccordionProps};
use crate::AccordionLeftProps;
use dioxus::prelude::*;

/// Standard accordion usage example:
/// ```rust
///            Accordion {
///                 title: title_one.clone(),
///                 accordion_text: first_text.clone(),
///                 optional_text: Some(String::from("Optional text displayed here. ignore me if you want!")),
///                 icon_color: AccordionIconColor::Dark,
///                 optional_text: Some("Optional text here".to_string()),
///                 class: Some(String::from("w-96")),
///                 title_class: Some(String::from("text-xl font-bold text-center")),
///                 accordion_wrapper: Some(String::from("bg-orange-200 rounded-t-lg")),
///                 accordion_content: Some(
///                     String::from(
///                         "rounded-b-lg bg-orange-100 w-full h-full text-orange-800 text-justify",
///                     ),
///                 ),
///             }
/// ```
/// Of course every rust Option can be simply ignored if preferred.
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let mut menu_open = use_signal(|| false);

    let plus_svg = rsx! {
        svg {
            class: "icon",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: "{props.icon_color.as_css_class()}",
            width: "24",
            height: "24",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 4v16m8-8H4",
            }
        }
    };

    let minus_svg = rsx! {
        svg {
            class: "icon",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: "{props.icon_color.as_css_class()}",
            width: "24",
            height: "24",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M4 12h16",
            }
        }
    };

    let accordion_class = props
        .class
        .clone()
        .unwrap_or_else(|| String::from("accordion"));
    let accordion_title_class = props
        .title_class
        .clone()
        .unwrap_or_else(|| String::from("title-wrapper-default"));
    let accordion_wrapper = props
        .accordion_wrapper
        .clone()
        .unwrap_or_else(|| String::from("accordion-wrapper-default"));
    let accordion_content = props
        .accordion_content
        .clone()
        .unwrap_or_else(|| String::from("accordion-content-default open-default"));

    rsx! {
        div {
            style { "{ACCORDION_STYLES}" }
            div { class: "{accordion_class}",
                div { class: "{accordion_wrapper} accordion-wrapper",
                    div { class: "{accordion_title_class}",
                        h1 { class: "", "{props.title}" }
                    }
                    button {
                        class: "accordion-button",
                        onclick: move |_| menu_open.set(!menu_open()),
                        match menu_open() {
                            true => rsx! {
                                div { class: "icon-wrapper", {minus_svg} }
                            },
                            false => rsx! {
                                div { class: "icon-wrapper", {plus_svg} }
                            },
                        }
                    }
                }
                match menu_open() {
                    true => rsx! {
                        div { class: "{accordion_content} accordion-content open",
                            p { {props.accordion_text} }
                            p { {props.optional_text} }
                        }
                    },
                    false => rsx! {},
                }
            }
        }
    }
}

/// Accordion Custom content example:
/// ```rust
/// AccordionCustom {
///                 title: title_two,
///                 accordion_text: rsx! {
///                     h1 { "Here is a custom text" }
///                     img {
///                         class: "w-1/2 h-1/2 rounded-lg shadow-lg m-4",
///                         src: "{IMAGE}",
///                         alt: "image",
///                     }
///                 },
///                 icon_color: AccordionIconColor::Light,
///                 class: Some(String::from("w-96")),
///                 title_class: Some(String::from("text-slate-100 text-xl font-bold text-center")),
///                 accordion_wrapper: Some(String::from("bg-orange-200 rounded-t-lg")),
///                 accordion_content: Some(
///                     String::from(
///                         "rounded-b-lg bg-orange-100 w-full h-full text-orange-800 text-justify",
///                     ),
///                 ),
///             }
/// ```
#[component]
pub fn AccordionCustom(props: AccordionCustomProps) -> Element {
    let mut menu_open = use_signal(|| false);
    let accordion_class = props
        .class
        .clone()
        .unwrap_or_else(|| String::from("accordion"));
    let accordion_title_class = props
        .title_class
        .clone()
        .unwrap_or_else(|| String::from("title-wrapper-default"));
    let accordion_wrapper = props
        .accordion_wrapper
        .clone()
        .unwrap_or_else(|| String::from("accordion-wrapper-default"));
    let accordion_content = props
        .accordion_content
        .clone()
        .unwrap_or_else(|| String::from("accordion-content-default open-default"));

    let plus_svg = rsx! {
        svg {
            class: "icon",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: props.icon_color.as_css_class(),
            width: "24",
            height: "24",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 4v16m8-8H4",
            }
        }
    };

    let minus_svg = rsx! {
        svg {
            class: "icon",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: props.icon_color.as_css_class(),
            width: "24",
            height: "24",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M4 12h16",
            }
        }
    };

    rsx! {
        div {
            style { "{ACCORDION_STYLES}" }
            div { class: "{accordion_class}",
                div { class: "{accordion_wrapper} accordion-wrapper",
                    div { class: "{accordion_title_class}",
                        h1 { "{props.title}" }
                    }
                    button {
                        class: "accordion-button",
                        onclick: move |_| menu_open.set(!menu_open()),
                        match menu_open() {
                            true => rsx! {
                                div { class: "icon-wrapper", {minus_svg} }
                            },
                            false => rsx! {
                                div { class: "icon-wrapper", {plus_svg} }
                            },
                        }
                    }
                }
                match menu_open() {
                    true => rsx! {
                        div { class: "{accordion_content} accordion-content open", {props.accordion_text} }
                    },
                    false => rsx! {},
                }
            }
        }
    }
}

#[component]
pub fn AccordionLeft(props: AccordionLeftProps) -> Element {
    let mut is_open = use_signal(|| false);

    let custom_style = format!(
        r#"
            :root {{
                --accordion-width: {};
                --header-background-color: {};
                --accordion-open-max-height: {};
                --box-shadow: {};
                --border-radius: {};
                --title-color: {};
                --title-font-size: {};
                --title-font-weight: {};
                --content-padding: {};
                --icon-color: {};
            }}
    "#,
        props.accordion_width.clone().unwrap_or("700px".into()),
        props
            .accordion_header_bg_color
            .clone()
            .unwrap_or("#f0f9ff".into()),
        props
            .accordion_open_max_height
            .clone()
            .unwrap_or("500px".into()),
        props.box_shadow.clone().unwrap_or("".into()),
        props.border_radius.clone().unwrap_or("10px".into()),
        props.title_color.clone().unwrap_or("#000000".into()),
        props.title_font_size.clone().unwrap_or("1.25rem".into()),
        props.title_font_weight.clone().unwrap_or("600".into()),
        props.content_padding.clone().unwrap_or("1rem".into()),
        props.icon_color.as_css_class(),
    );

    let chevron_svg = rsx! {
         svg {
             class: format_args!("accordion-left-icon {}", if is_open() { "open" } else { "" }),
             xmlns: "http://www.w3.org/2000/svg",
             fill: "none",
             view_box: "0 0 24 24",
             stroke_width: "2",
             stroke: "var(--icon-color)",
             width: "20",
             height: "20",
             path {
                 stroke_linecap: "round",
                 stroke_linejoin: "round",
                 d: "M9 5l7 7-7 7",
             }
         }
    };

    let style_tag = rsx! {
        style { "{ACCORDION_NEW_STYLES}" }
        style { "{custom_style}" }
    };

    rsx! {
        div {
            {style_tag}
            div { class: "accordion-left",
                div {
                    class: "accordion-left-header",
                    onclick: move |_| is_open.set(!is_open()),
                    {chevron_svg}
                    h2 { class: "accordion-left-title", "{props.title}" }
                }
                div { class: format_args!("accordion-left-content {}", if is_open() { "open" } else { "" }),
                    {props.children}
                }
            }
        }
    }
}

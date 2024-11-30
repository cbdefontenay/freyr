use crate::assets::accordion_styles::ACCORDION_STYLES;
use crate::enums::accordion_enums::{AccordionProps, AccordionCustomProps};
use dioxus::prelude::*;

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
            stroke: "#000",
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
            stroke: "#000",
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
        .class
        .clone()
        .unwrap_or_else(|| String::from("accordion"));

    rsx! {
        div { class: "{accordion_class}",
            div { class: "accordion-wrapper",
                div { class: "title-wrapper",
                    h1 { class: "title", "{props.title}" }
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
                    div { class: "accordion-content open",
                        p { "{props.accordion_text}" }
                    }
                },
                false => rsx! {},
            }
        }
    }
}

#[component]
pub fn AccordionCustom(props: AccordionCustomProps) -> Element {
    let mut menu_open = use_signal(|| false);

    let plus_svg = rsx! {
        svg {
            class: "icon",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2",
            stroke: "#000",
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
            stroke: "#000",
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
        .class
        .clone()
        .unwrap_or_else(|| String::from("accordion"));

    rsx! {
        div { class: "{accordion_class}",
            div { class: "accordion-wrapper",
                div { class: "title-wrapper",
                    h1 { class: "title", "{props.title}" }
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
                    div { class: "accordion-content open", {props.text} }
                },
                false => rsx! {},
            }
        }
    }
}

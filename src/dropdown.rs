use dioxus::prelude::*;
use crate::assets::dropdown_styles::DROPDOWN_STYLES;
use crate::DropdownButtonConfig;
use crate::enums::dropdown_enums::{
    DropdownConfig, DropdownItem, DropdownColorScheme, DropdownTitleColor, DropdownLabelsColor, DropdownHoverColor,
};

/// `DropdownMenu` is a customizable dropdown menu component.
/// You can pass a `DropdownConfig` to customize the list of items,
/// background color, and text color.
///
/// # Props:
/// - `config`: A `DropdownConfig` that specifies the items and styles for the dropdown menu.
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use freyr::prelude::*;
///
/// #[component]
/// fn Home() -> Element {
///     let dropdown_items = vec![
///         DropdownItem { label: "Home".to_string(), url: Some("/".to_string()) },
///         DropdownItem { label: "About".to_string(), url: Some("/about".to_string()) },
///         // without routing
///         DropdownItem::without_url("A Label without route"),
///         // Or like that
///         DropdownItem { label: "Contact".to_string(), url: None },
///     ];
///
///     let config_dropdown = DropdownConfig {
///         title: "My dropdown".to_string(),
///         label: dropdown_items,
///         background_color: DropdownColorScheme::Freyr,
///         title_color: DropdownTitleColor::Light,
///         labels_color: DropdownLabelsColor::Light,
///         hover_color: DropdownHoverColor::Custom("#03346E"),
///     };
///
///     rsx! {
///         DropdownMenu { config_dropdown }
///     }
/// }
/// ```
///
/// **NOTE:** The name **_"config_dropdown"_** is mandatory.
///

#[component]
pub fn DropdownMenu(config_dropdown: DropdownConfig) -> Element {
    let mut is_open = use_signal(|| false);

    let style_tag = rsx! {
        style { "{DROPDOWN_STYLES}" }
    };

    let arrow_down_svg = rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: "feather feather-chevron-down",
            path { d: "M6 9l6 6 6-6" }
        }
    };

    let arrow_up_svg = rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: "feather feather-chevron-up",
            path { d: "M18 15l-6-6-6 6" }
        }
    };

    rsx! {
        div {
            {style_tag}

            div { class: "dropdown",

                button {
                    class: "dropdown-toggle",
                    style: "background-color: {config_dropdown.background_color.as_css_class()}; color: {config_dropdown.title_color.as_css_class()};",
                    onclick: move |_| is_open.set(!is_open()),
                    "{config_dropdown.title}"

                    match is_open() {
                        true => arrow_up_svg,
                        false => arrow_down_svg,
                    }
                }

                div {
                    match is_open() {
                        true => {
                            rsx! {
                                div {
                                    class: "dropdown-content",
                                    style: "background-color: {config_dropdown.background_color.as_css_class()}; color: {config_dropdown.labels_color.as_css_class()};",
                                    for item in config_dropdown.label {
                                        if let Some(url) = &item.url {
                                            Link {
                                                class: "link",
                                                to: url.clone(),
                                                style: "color: {config_dropdown.labels_color.as_css_class()}; --custom_color: {config_dropdown.hover_color.as_css_class()};",
                                                "{item.label}"
                                            }
                                        } else {
                                            span {
                                                class: "link",
                                                style: "color: {config_dropdown.labels_color.as_css_class()};",
                                                "{item.label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        false => {
                            rsx! {}
                        }
                    }
                }
            }
        }
    }
}

/// Usage example of a dropdown that passes onclick events to its items like a dummy counter:
/// ```rust
///let mut counter = use_signal(|| 0);
///
///     let dropdown_items = vec!["Increment".to_string(), "Decrement".to_string()];
///
///     let increment = move |_| counter += 1;
///     let decrement = move |_| counter -= 1;
///
///     let onclick_handlers: Vec<EventHandler<MouseEvent>> =
///         vec![EventHandler::new(increment), EventHandler::new(decrement)];
///
///     let config_dropdown = DropdownButtonConfig {
///         title: "Counter".to_string(),
///         labels: dropdown_items,
///         onclick: onclick_handlers,
///         background_color: DropdownColorScheme::Dark,
///         title_color: DropdownTitleColor::Light,
///         labels_color: DropdownLabelsColor::Light,
///         hover_color: DropdownHoverColor::Custom("#03346E"),
///     };
///
///     rsx! {
///         div {
///             class: "w-full flex justify-center items-center mt-20",
///             DropdownMenuButton { config_dropdown }
///             p { "Counter: {counter}"}
///         }
///     }
/// ```
#[component]
pub fn DropdownMenuButton(config_dropdown: DropdownButtonConfig) -> Element {
    let mut is_open = use_signal(|| false);

    let style_tag = rsx! {
        style { "{DROPDOWN_STYLES}" }
    };

    let arrow_down_svg = rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: "feather feather-chevron-down",
            path { d: "M6 9l6 6 6-6" }
        }
    };

    let arrow_up_svg = rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: "feather feather-chevron-up",
            path { d: "M18 15l-6-6-6 6" }
        }
    };

    rsx! {
        div {
            {style_tag}

            div { class: "dropdown",
                button {
                    class: "dropdown-toggle",
                    style: "background-color: {config_dropdown.background_color.as_css_class()}; color: {config_dropdown.title_color.as_css_class()};",
                    onclick: move |_| is_open.set(!is_open()),
                    "{config_dropdown.title}"

                    match is_open() {
                        true => arrow_up_svg,
                        false => arrow_down_svg,
                    }
                }

                div {
                    match is_open() {
                        true => {
                            rsx! {
                                div {
                                    class: "dropdown-content",
                                    style: "background-color: {config_dropdown.background_color.as_css_class()}; color: {config_dropdown.labels_color.as_css_class()};",
                                    for (label, onclick_handler) in config_dropdown.labels.iter().zip(config_dropdown.onclick.iter()) {
                                        button {
                                            onclick: onclick_handler.clone(),
                                            class: "button-config",
                                            style: "color: {config_dropdown.labels_color.as_css_class()}; --custom_color: {config_dropdown.hover_color.as_css_class()};",
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                        false => rsx! {},
                    }
                }
            }
        }
    }
}


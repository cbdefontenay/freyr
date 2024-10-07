use dioxus::prelude::*;
use crate::assets::dropdown_styles::DROPDOWN_STYLES;
use crate::enums::dropdown_enums::{
    DropdownConfig, DropdownItem, DropdownColorScheme, DropdownTitleColor, DropdownLabelsColor,
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
/// use freyr::{DropdownMenu, DropdownConfig, DropdownItem, DropdownColorScheme, DropdownLabelsColor, DropdownTitleColor};
///
/// #[component]
/// fn Home() -> Element {
///     let dropdown_items = vec![
///         DropdownItem { label: "Home".to_string(), url: "/".to_string() },
///         DropdownItem { label: "About".to_string(), url: "/about".to_string() },
///         DropdownItem { label: "Contact".to_string(), url: "/contact".to_string() },
///     ];
///
///     let config_dropdown = DropdownConfig {
///         title: "My dropdown".to_string(),
///         label: dropdown_items,
///         background_color: DropdownColorScheme::Dark,
///         title_color: DropdownTitleColor::Custom("#F9E400"),
///         labels_color: DropdownLabelsColor::Custom("#C5705D"),
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
/// This example demonstrates how to create a dropdown menu with three items: "Home", "About", and "Contact".
/// The background color is set to black, and the text color is customized.
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

            div {
                class: "dropdown",

                button {
                    class: "dropdown-toggle",
                    style: "background-color: {config_dropdown.background_color.as_css_class()}; color: {config_dropdown.title_color.as_css_class()};",
                    onclick: move |_| is_open.set(!is_open()),
                    "{config_dropdown.title}",

                    match is_open() {
                        true => {arrow_up_svg},
                        false => {arrow_down_svg},
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
                                        Link {
                                            class: "link",
                                            to: item.url.clone(),
                                             style: "color: {config_dropdown.labels_color.as_css_class()};",
                                            "{item.label}"
                                        }
                                    }
                                }
                            }
                        },
                        false => {
                            rsx! {}
                        },
                    },
                }
            }
        }
    }
}

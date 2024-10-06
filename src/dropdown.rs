use dioxus::prelude::*;
use crate::assets::dropdown_styles::DROPDOWN_STYLES;
use crate::enums::dropdown_enums::{DropdownConfig, DropdownColorScheme, DropdownLabelsColor};

/// `DropdownMenu` is a customizable dropdown menu component.
/// You can pass a `DropdownConfig` to customize the list of items,
/// background color, and text color.
///
/// # Props:
/// - `config`: A `DropdownConfig` that specifies the items and styles for the dropdown menu.
///
/// # Example
/// ```rust
/// use freyr::{DropdownMenu, DropdownConfig, DropdownItem, DropdownColorScheme, DropdownLabelsColor};
/// use dioxus::prelude::*;
///
/// #[component]
/// fn Home() -> Element {
///     let dropdown_items = vec![
///         DropdownItem { label: "Home".to_string(), url: "/".to_string() },
///         DropdownItem { label: "About".to_string(), url: "/about".to_string() },
///         DropdownItem { label: "Contact".to_string(), url: "/contact".to_string() },
///     ];
///
///     let config = DropdownConfig {
///         title: "My dropdown".to_string(),
///         label: dropdown_items,
///         background_color: DropdownColorScheme::Custom("#ffffff"),
///         labels_color: DropdownLabelsColor::Custom("#000000"),
///     };
///
///     rsx! {
///         DropdownMenu { config: config.clone() }
///     }
/// }
/// ```
///
/// This example demonstrates how to create a dropdown menu with three items: "Home", "About", and "Contact".
/// The background color is set to white, and the text color is black.
#[component]
pub fn DropdownMenu(config: DropdownConfig) -> Element {
    let mut is_open = use_signal(|| false);

    let style_tag = rsx! {
        style { "{DROPDOWN_STYLES}" }
    };

    rsx! {
        div {
            {style_tag}

            div {
                class: "dropdown",

                button {
                style: "background-color: {config.background_color.as_css_class()};",
                    class: "dropdown-toggle",
                    onclick: move |_| is_open.set(!is_open()),
                    "{config.title}"
                }

                div {
                    match is_open() {
                        true => {
                            rsx! {
                                div {
                                    class: "dropdown-content",
                                    style: "background-color: {config.background_color.as_css_class()}; color: {config.labels_color.as_css_class()};",

                                    for item in config.label {
                                            Link {
                                                class: "link",
                                                to: item.url.clone(),
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

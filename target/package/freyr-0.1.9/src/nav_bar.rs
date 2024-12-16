use dioxus::prelude::*;
use crate::assets::navbar_style::NAVBAR_STYLES;
use crate::{DropdownColorScheme, DropdownHoverColor, DropdownItem, DropdownLabelsColor, DropdownMenu, DropdownTitleColor};
use crate::assets::navbar_dropdown_styles::NAVBAR_DROPDOWN_STYLES;
use crate::{ColorScheme, DropdownConfig, DropdownConfigNavBar, IconColor, NavItemsColor};
use crate::enums::navbar_enums::{NavbarConfig, NavbarDropdownConfig};

/// You can configure background color, navigation items, and icon colors.
///
/// # Examples
///
/// ```rust
/// let navbar_config = NavbarConfig {
///     background_color: ColorScheme::Freyr,
///     nav_header: "Freyr".to_string(),
///     nav_items: vec!["Home".to_string(), "About".to_string(), "Contact".to_string()],
///     nav_links: vec!["/".to_string(), "/about".to_string(), "/contact".to_string()],
///     nav_item_color: NavItemsColor::Light,
///     icon_color: IconColor::White,
/// };
///
/// rsx! {
///     Navbar { navbar_config }
/// };
/// ```
#[component]
pub fn Navbar(navbar_config: NavbarConfig) -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div {
            style { "{NAVBAR_STYLES}" }

            nav {
                class: "navbar",
                style: "background-color: {navbar_config.background_color.as_css_class()};",

                div { class: "nav-div",

                    div { class: "nav-header-wrapper", "{navbar_config.nav_header}" }

                    button {
                        class: "hamburger",
                        onclick: move |_| menu_open.set(!menu_open()),

                        match menu_open() {
                            true => {
                                rsx! {
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "32",
                                        height: "32",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "{navbar_config.icon_color.as_css_class()}",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M18 6L6 18M6 6L18 18" }
                                    }
                                }
                            }
                            false => {
                                rsx! {
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "32",
                                        height: "32",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "{navbar_config.icon_color.as_css_class()}",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M4 6h16M4 12h16M4 18h16" }
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    class: match menu_open() {
                        true => "menu open",
                        false => "menu",
                    },
                    style: "background-color: {navbar_config.background_color.as_css_class()};",

                    div { class: "menu-items",

                        for (item , link) in navbar_config.nav_items.iter().zip(navbar_config.nav_links.iter()) {
                            Link {
                                class: "menu-item",
                                to: "{link}",
                                style: "color: {navbar_config.nav_item_color.as_css_class()};",
                                onclick: move |_| {
                                    menu_open.set(false);
                                },
                                "{item}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Navbar with a Dropdown menu implementation example:
/// ```rust
/// #[component]
/// pub fn Navigation() -> Element {
///     let navbar_config = NavbarConfig {
///         background_color: ColorScheme::Freyr,
///         nav_header: "Freyr".to_string(),
///         nav_items: vec![
///             "Home".to_string(),
///             "About".to_string(),
///             "Contact".to_string(),
///         ],
///         nav_links: vec![
///             "/".to_string(),
///             "/about".to_string(),
///             "/contact".to_string(),
///         ],
///         nav_item_color: NavItemsColor::Light,
///         icon_color: IconColor::White,
///     };
///
///     let dropdown_items = vec![
///         DropdownItem {
///             label: "Home".to_string(),
///             url: Some("/".to_string()),
///         },
///         DropdownItem {
///             label: "About".to_string(),
///             url: Some("/about".to_string()),
///         },
///         DropdownItem::without_url("A Label without route"),
///         DropdownItem {
///             label: "Contact".to_string(),
///             url: None,
///         },
///     ];
///
///     let config_dropdown = DropdownConfig {
///         title: "My dropdown".to_string(),
///         label: dropdown_items,
///         background_color: DropdownColorScheme::Dark,
///         title_color: DropdownTitleColor::Light,
///         labels_color: DropdownLabelsColor::Light,
///         hover_color: DropdownHoverColor::Custom("#47453e"),
///     };
///
///     rsx! {
///         NavbarDropdown { navbar_config, config_dropdown },
///         Outlet::<Route> {}
///     }
/// }
/// ```

#[component]
pub fn NavbarDropdown(
    navbar_config: NavbarConfig,
    config_dropdown: DropdownConfig,
) -> Element {
    let mut menu_open = use_signal(|| false);
    let dropdown_open = use_signal(|| None::<usize>);

    rsx! {
        div {
            style { "{NAVBAR_DROPDOWN_STYLES}" }

            nav {
                class: "navbar",
                style: "background-color: {navbar_config.background_color.as_css_class()};",

                div { class: "nav-div",

                    div { class: "nav-header-wrapper", "{navbar_config.nav_header}" }

                    button {
                        class: "hamburger",
                        onclick: move |_| menu_open.set(!menu_open()),

                        match menu_open() {
                            true => {
                                rsx! {
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "32",
                                        height: "32",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "{navbar_config.icon_color.as_css_class()}",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M18 6L6 18M6 6L18 18" }
                                    }
                                }
                            }
                            false => {
                                rsx! {
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "32",
                                        height: "32",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "{navbar_config.icon_color.as_css_class()}",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M4 6h16M4 12h16M4 18h16" }
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    class: match menu_open() {
                        true => "menu open",
                        false => "menu",
                    },
                    style: "background-color: {navbar_config.background_color.as_css_class()};",

                    div { class: "menu-items",

                        for (item , link) in navbar_config.nav_items.iter().zip(navbar_config.nav_links.iter()) {
                            Link {
                                class: "menu-item",
                                to: "{link}",
                                style: "color: {navbar_config.nav_item_color.as_css_class()};",
                                onclick: move |_| menu_open.set(false),
                                "{item}"
                            }
                        }
                        div {
                            class:"dropdown-navbar",
                            DropdownMenu { config_dropdown }
                        }
                    }
                }
            }
        }
    }
}
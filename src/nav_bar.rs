use dioxus::prelude::*;
use crate::assets::navbar_style::NAVBAR_STYLES;
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

#[component]
pub fn NavbarDropdown(navbar_config: NavbarDropdownConfig) -> Element {
    let mut menu_open = use_signal(|| false);
    let mut dropdown_open = use_signal(|| None::<usize>);

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
                                onclick: move |_| menu_open.set(false),
                                "{item}"
                            }
                        }

                        for (index, dropdown) in navbar_config.dropdowns.iter().enumerate() {
                            div {
                                class: "menu-dropdown",
                                onclick: move |_| {
                                    if dropdown_open() == Some(index) {
                                        dropdown_open.set(None);
                                    } else {
                                        dropdown_open.set(Some(index));
                                    }
                                },
                                "{dropdown.label}",

                                match dropdown_open() {
                                    Some(open_index) if open_index == index => {
                                        rsx! {
                                            div {
                                                class: "dropdown-items",
                                                for (item_label, item_link) in dropdown.items.iter() {
                                                    Link {
                                                        class: "dropdown-item",
                                                        to: "{item_link}",
                                                        style: "color: {navbar_config.nav_item_color.as_css_class()};",
                                                        onclick: move |_| {
                                                            dropdown_open.set(None);
                                                            menu_open.set(false);
                                                        },
                                                        "{item_label}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => rsx! {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
use crate::assets::navbar_dropdown_styles::NAVBAR_DROPDOWN_STYLES;
use crate::assets::navbar_style::NAVBAR_STYLES;
use crate::enums::navbar_enums::NavbarConfig;
use crate::DropdownConfig;
use crate::{DropdownButtonConfig, DropdownMenu, DropdownMenuButton, Orientation};
use dioxus::prelude::*;

/// You can configure background color, navigation items, and icon colors.
///
/// # Examples
///
/// ```rust
/// let navbar_config = NavbarConfig {
///     background_color: ColorScheme::Freyr,
///     nav_header: Some(String::from("Freyr")),
///     orientation: Some(Orientation::Center),
///     header_color: HeaderColor::Light,
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
    let orientation_class = match navbar_config
        .orientation
        .clone()
        .unwrap_or(Orientation::Right)
    {
        Orientation::Left => "menu-items left",
        Orientation::Center => "menu-items center",
        Orientation::Right => "menu-items right",
    };

    rsx! {
        div {
            style { "{NAVBAR_STYLES}" }

            nav {
                class: "navbar",
                style: "background-color: {navbar_config.background_color.as_css_class()};",

                div { class: "nav-div",

                    div { class: "",
                        match &navbar_config.nav_header {
                            Some(header) => rsx! {
                                div {
                                    class: "nav-header-wrapper",
                                    style: "color: {navbar_config.header_color.as_css_class()};",
                                    Link { to: "/", "{header}" }
                                }
                            },
                            None => rsx! {
                                div { class: "no-nav-header",
                                    span { "" }
                                }
                            },
                        }
                    }

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

                    div { class: "{orientation_class}",
                        for (item , link) in navbar_config.nav_items.iter().zip(navbar_config.nav_links.iter()) {
                            Link {
                                class: "menu-item",
                                to: "{link}",
                                style: "color: {navbar_config.nav_item_color.as_css_class()};",
                                onclick: move |_| menu_open.set(false),
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
///         nav_header: Some(String::from("Freyr")),
///         orientation: Some(Orientation::Center),
///         header_color: HeaderColor::Light,
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
pub fn NavbarDropdown(navbar_config: NavbarConfig, config_dropdown: DropdownConfig) -> Element {
    let mut menu_open = use_signal(|| false);
    let _dropdown_open = use_signal(|| None::<usize>);
    let orientation_class = match navbar_config
        .orientation
        .clone()
        .unwrap_or(Orientation::Right)
    {
        Orientation::Left => "menu-items left",
        Orientation::Center => "menu-items center",
        Orientation::Right => "menu-items right",
    };

    let has_header = navbar_config.nav_header.is_some();
    let navbar_class = if has_header {
        "navbar"
    } else {
        "navbar no-header"
    };

    rsx! {
        div {
            style { "{NAVBAR_DROPDOWN_STYLES}" }

            nav {
                class: "{navbar_class}",
                style: "background-color: {navbar_config.background_color.as_css_class()};",
                div { class: "nav-div",

                    div { class: "",
                        match &navbar_config.nav_header {
                            Some(header) => rsx! {
                                div {
                                    class: "nav-header-wrapper",
                                    style: "color: {navbar_config.header_color.as_css_class()};",
                                    Link { to: "/", "{header}" }
                                }
                            },
                            None => rsx! {
                                div { class: "no-nav-header",
                                    span { "" }
                                }
                            },
                        }
                    }

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

                    div { class: "{orientation_class}",
                        for (item , link) in navbar_config.nav_items.iter().zip(navbar_config.nav_links.iter()) {
                            Link {
                                class: "menu-item",
                                to: "{link}",
                                style: "color: {navbar_config.nav_item_color.as_css_class()};",
                                onclick: move |_| menu_open.set(false),
                                "{item}"
                            }
                        }
                        div { class: "dropdown-navbar",
                            DropdownMenu { config_dropdown: config_dropdown.clone() }
                        }
                    }
                }
            }
        }
    }
}

/// Example use of the navbar with a **dropdown menu** that allows you to pass onclick events.
/// This example introduces the use of _dioxus-i18n_ for internationalization.
/// ```rust
/// #[component]
/// pub fn Navigation() -> Element {
///     let mut i18n = i18n();
///
///     let change_to_english = move |_| i18n.set_language(langid!("en-US"));
///     let change_to_french = move |_| i18n.set_language(langid!("fr-FR"));
///
///     let dropdown_items = vec!["English".to_string(), "Français".to_string()];
///
///     let onclick_handlers: Vec<EventHandler<MouseEvent>> = vec![
///         EventHandler::new(change_to_english),
///         EventHandler::new(change_to_french),
///     ];
///
///     let config_dropdown = DropdownButtonConfig {
///         title: t!("languages"),
///         labels: dropdown_items,
///         onclick: onclick_handlers,
///         background_color: DropdownColorScheme::Dark,
///         title_color: DropdownTitleColor::Light,
///         labels_color: DropdownLabelsColor::Light,
///         hover_color: DropdownHoverColor::Custom("#03346E"),
///     };
///
///     let navbar_config = NavbarConfig {
///         background_color: ColorScheme::Freyr,
///         nav_header: Some(String::from("Freyr")),
///         orientation: Some(Orientation::Center),
///         header_color: HeaderColor::Light,
///         nav_items: vec![
///             "Home".to_string(),
///             t!("about"),
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
///
///     rsx! {
///         NavbarDropdownButtons { navbar_config, config_dropdown }
///         Outlet::<Route> {}
///     }
/// }
/// ```
#[component]
pub fn NavbarDropdownButtons(
    navbar_config: NavbarConfig,
    config_dropdown: DropdownButtonConfig,
) -> Element {
    let mut menu_open = use_signal(|| false);
    let _dropdown_open = use_signal(|| None::<usize>);
    let orientation_class = match navbar_config
        .orientation
        .clone()
        .unwrap_or(Orientation::Right)
    {
        Orientation::Left => "menu-items left",
        Orientation::Center => "menu-items center",
        Orientation::Right => "menu-items right",
    };

    rsx! {
        div {
            style { "{NAVBAR_DROPDOWN_STYLES}" }

            nav {
                class: "navbar",
                style: "background-color: {navbar_config.background_color.as_css_class()};",

                div { class: "nav-div",
                    div { class: "",
                        match &navbar_config.nav_header {
                            Some(header) => rsx! {
                                div {
                                    class: "nav-header-wrapper",
                                    style: "color: {navbar_config.header_color.as_css_class()};",
                                    Link { to: "/", "{header}" }
                                }
                            },
                            None => rsx! {
                                div { class: "no-nav-header",
                                    span { "" }
                                }
                            },
                        }
                    }
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

                    div { class: "{orientation_class}",
                        for (item , link) in navbar_config.nav_items.iter().zip(navbar_config.nav_links.iter()) {
                            Link {
                                class: "menu-item",
                                to: "{link}",
                                style: "color: {navbar_config.nav_item_color.as_css_class()};",
                                onclick: move |_| menu_open.set(false),
                                "{item}"
                            }
                        }
                        div { class: "dropdown-navbar",
                            DropdownMenuButton { config_dropdown: config_dropdown.clone() }
                        }
                    }
                }
            }
        }
    }
}

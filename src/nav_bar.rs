use dioxus::prelude::*;
use crate::assets::navbar_style::NAVBAR_STYLES;

/// Represents different background color schemes for the navbar.
///
/// # Examples
///
/// Using a dark background:
/// ```rust
/// let config = NavbarConfig {
///     background_color: ColorScheme::Dark,
///     ..Default::default()
/// };
/// ```
///
/// Using a custom background color:
/// ```rust
/// let config = NavbarConfig {
///     background_color: ColorScheme::Custom("#ff5733"),
///     ..Default::default()
/// };
/// ```
#[derive(PartialEq, Clone)]
pub enum ColorScheme {
    Dark,
    Light,
    Custom(&'static str),
}

impl ColorScheme {
    /// Returns the CSS class or custom color for the background.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            ColorScheme::Dark => "#222",
            ColorScheme::Light => "#fff",
            ColorScheme::Custom(color) => color,
        }
    }
}

/// Represents different color options for the navigation items.
/// There are three available colors: Dark, Light, Custom (for example "#ff66a3").
///
/// # Examples
///
/// Using a light color for nav items:
/// ```rust
/// let config = NavbarConfig {
///     nav_item_color: NavItemsColor::Light,
/// };
/// ```
///
/// Using a custom color for nav items:
/// ```rust
/// let config = NavbarConfig {
///     nav_item_color: NavItemsColor::Custom("#ff66a3"),
/// };
/// ```
#[derive(PartialEq, Clone)]
pub enum NavItemsColor {
    Dark,
    Light,
    Custom(&'static str),
}

impl NavItemsColor {
    /// Returns the CSS class or custom color for the navigation items.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            NavItemsColor::Dark => "#000",
            NavItemsColor::Light => "#fff",
            NavItemsColor::Custom(color) => color,
        }
    }
}

/// Represents different color options for the menu icons (hamburger and cross).
///
/// # Examples
///
/// Using a white icon (_hamburger or cross SVG_):
/// ```rust
/// let config = NavbarConfig {
///     icon_color: IconColor::White,
/// };
/// ```
///
/// Using a custom color for the icons:
/// ```rust
/// let config = NavbarConfig {
///     icon_color: IconColor::Custom("#ffcc00"),
/// };
/// ```
#[derive(PartialEq, Clone)]
pub enum IconColor {
    White,
    Black,
    Custom(&'static str),
}

impl IconColor {
    /// Returns the CSS class or custom color for the icons.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            IconColor::White => "#fff",
            IconColor::Black => "#000",
            IconColor::Custom(color) => color,
        }
    }
}

/// Configuration for the Navbar component.
///
/// # Examples
///
/// Creating a navbar with a dark background and light navigation items:
/// ```rust
/// let config = NavbarConfig {
///     background_color: ColorScheme::Dark,
///     nav_items: vec!["Home".to_string(), "About".to_string(), "Contact".to_string()],
///     nav_links: vec!["/".to_string(), "/about".to_string(), "/contact".to_string()],
///     nav_item_color: NavItemsColor::Light,
///     icon_color: IconColor::White,
/// };
/// ```
#[derive(PartialEq, Clone)]
pub struct NavbarConfig {
    pub background_color: ColorScheme,
    pub nav_items: Vec<String>,
    pub nav_links: Vec<String>,
    pub nav_item_color: NavItemsColor,
    pub icon_color: IconColor,
}

#[component]
pub fn Navbar(config: NavbarConfig) -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div {
            style { "{NAVBAR_STYLES}" }

            nav {
                class: "navbar",
                style: "background-color: {config.background_color.as_css_class()};",

                div {
                    class: "nav-div",

                    div {
                        class: "nav-header-wrapper",
                        "Logo"
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
                                        stroke: "{config.icon_color.as_css_class()}",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path {
                                            d: "M18 6L6 18M6 6L18 18",
                                        }
                                    }
                                }
                            },
                            false => {
                                rsx! {
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "32",
                                        height: "32",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "{config.icon_color.as_css_class()}",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path {
                                            d: "M4 6h16M4 12h16M4 18h16",
                                        }
                                    }
                                }
                            },
                        }
                    }
                }

                div {
                    class: match menu_open() {
                        true => "menu open",
                        false => "menu",
                    },
                    style: "background-color: {config.background_color.as_css_class()};",

                    div {
                        class: "menu-items",

                        for (item, link) in config.nav_items.iter().zip(config.nav_links.iter()) {
                            Link {
                                class: "menu-item",
                                to: "{link}",
                                style: "color: {config.nav_item_color.as_css_class()};",
                                onclick: move |_| {
                                    menu_open.set(false);  // Close the menu after being clicked
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


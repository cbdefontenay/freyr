use dioxus::prelude::*;
use crate::assets::navbar_style::NAVBAR_STYLES;

#[derive(PartialEq, Clone)]
pub enum ColorScheme {
    Dark,
    Light,
    Custom(&'static str),
}

#[derive(PartialEq, Clone)]
pub enum NavItemsColor {
    Dark,
    Light,
    Custom(&'static str),
}

#[derive(PartialEq, Clone)]
pub enum IconColor {
    White,
    Black,
    Custom(&'static str),
}

#[derive(PartialEq, Clone)]
pub struct NavbarConfig {
    pub background_color: ColorScheme,
    pub nav_items: Vec<String>,
    pub nav_item_color: NavItemsColor,
    pub icon_color: IconColor,
}

impl ColorScheme {
    pub fn as_css_class(&self) -> &'static str {
        match self {
            ColorScheme::Dark => "#222",
            ColorScheme::Light => "#fff",
            ColorScheme::Custom(color) => color,
        }
    }
}

impl NavItemsColor {
    pub fn as_css_class(&self) -> &'static str {
        match self {
            NavItemsColor::Dark => "#000",
            NavItemsColor::Light => "#fff",
            NavItemsColor::Custom(color) => color,
        }
    }
}

impl IconColor {
    pub fn as_css_class(&self) -> &'static str {
        match self {
            IconColor::White => "#fff",
            IconColor::Black => "#000",
            IconColor::Custom(color) => color,
        }
    }
}

#[component]
pub fn Navbar(config: NavbarConfig) -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div {
            style { "{NAVBAR_STYLES}" } // Load general styles

            nav {
                class: "navbar", // General navbar class for layout

                // Apply user-defined background color via inline style
                style: "background-color: {config.background_color.as_css_class()};",

                div {
                    class: "nav-div", // Flex container for logo and hamburger

                    div {
                        class: "nav-header-wrapper",
                        "Logo" // User can modify this later as needed
                    }

                    button {
                        class: "hamburger",
                        onclick: move |_| menu_open.set(!menu_open()),

                        match menu_open() {
                            true => {
                                // Customizable Cross Icon
                                rsx! {
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "32",
                                        height: "32",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "{config.icon_color.as_css_class()}", // Custom icon color
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
                                // Customizable Hamburger Icon
                                rsx! {
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "32",
                                        height: "32",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "{config.icon_color.as_css_class()}", // Custom icon color
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

                    // Apply user-defined background color to the opened menu
                    style: "background-color: {config.background_color.as_css_class()};",

                    div {
                        class: "menu-items",


                        // TODO: Custom navigation links
                        for item in &config.nav_items {
                            Link {
                                class: "menu-item",
                                to: "/",
                                style: "color: {config.nav_item_color.as_css_class()};",
                                "{item}"
                            }
                        }
                    }
                }
            }
        }
    }
}

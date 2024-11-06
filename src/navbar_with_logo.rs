use dioxus::prelude::*;
use crate::assets::navbar_style::NAVBAR_STYLES;
use crate::enums::navbar_enums::NavbarWithLogoConfig;

/// This navbar works the same as the normal **navbar** component, but instead of taking a logo as _String_ it takes an _image_.
///
/// # Examples
///
/// ```rust
///    #[component]
///    fn Navigation() -> Element {
///     const LOGO: Asset = asset!("./assets/logo.png");
///
///     let navbar_logo_config = NavbarWithLogoConfig {
///         background_color: ColorScheme::Freyr,
///         nav_items: vec!["Home".to_string(), "About".to_string(), "Contact".to_string()],
///         nav_links: vec!["/".to_string(), "/about".to_string(), "/contact".to_string()],
///         nav_item_color: NavItemsColor::Light,
///         icon_color: IconColor::White,
///         logo_url: String::from("/"),
///         logo_src: LOGO,
///         logo_alt: String::from("logo"),
///     };
///
///     rsx! {
///         NavbarWithLogo { navbar_logo_config }
///         Outlet::<Route> {}
///     }
///    }
/// ```
#[component]
pub fn NavbarWithLogo(navbar_logo_config: NavbarWithLogoConfig) -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div {
            style { "{NAVBAR_STYLES}" }

            nav {
                class: "navbar",
                style: "background-color: {navbar_logo_config.background_color.as_css_class()};",

                div {
                    class: "nav-div",

                    Link {
                        to: "{navbar_logo_config.logo_url}",
                            img {
                            class: "nav-logo",
                            src: "{navbar_logo_config.logo_src}",
                            alt: "{navbar_logo_config.logo_alt}"
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
                                        stroke: "{navbar_logo_config.icon_color.as_css_class()}",
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
                                        stroke: "{navbar_logo_config.icon_color.as_css_class()}",
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
                    style: "background-color: {navbar_logo_config.background_color.as_css_class()};",

                    div {
                        class: "menu-items",

                        for (item, link) in navbar_logo_config.nav_items.iter().zip(navbar_logo_config.nav_links.iter()) {
                            Link {
                                class: "menu-item",
                                to: "{link}",
                                style: "color: {navbar_logo_config.nav_item_color.as_css_class()};",
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
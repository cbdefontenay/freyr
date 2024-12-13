//! # Freyr UI Components Library for Dioxus.
//!
//! This crate provides a set of customizable UI components for use in Dioxus projects.
//! It allows developers to easily integrate and style components such as navbars and buttons,
//! with flexible configuration options for color schemes, layouts, and responsiveness.
//!
//! ## Why the name Freyr?
//!
//! Freyr is the norse god of fertility and prosperity and is known for his beauty. Your Dioxus app should also look good.
//!
//! > ### **Warning:**
//! > **_This component is in the early stage of development. Right now there are only five components available: the navbar, the dropdown, the two carousel and the buttons._**
//! > **_New components will be added, and some features that already exist may change._**
//! ## Components
//!
//! - **BasicButton**: A customizable button with color options, and an arbitrary hover effect.
//! - **Navbar**: A fully customizable navigation bar with custom colors, and responsive layouts.
//! - **Dropdown**: A dropdown menu with customizable background colors and labels colors.
//! - **Carousel**: There are two carousel at the moment, the CarouselSimple and the CarouselWithTimer.
//!
//! ## Key Features
//! - Full customization of colors, sizes, and layouts using configuration structs and enums.
//! - Responsive designs that adapt seamlessly to mobile and desktop layouts.
//! - Easy integration into Dioxus web projects.
//!
//! ## Example Usage
//!
//! ```rust
//! #![allow(non_snake_case)]
//! use dioxus::prelude::*;
//! use freyr::prelude::*;
//!
//! #[derive(Clone, Routable, Debug, PartialEq)]
//! enum Route {
//!     #[layout(Navigation)]
//!     #[route("/")]
//!     Home {},
//! }
//!
//!
//! #[component]
//! fn Navigation() -> Element {
//!     let navbar_config = NavbarConfig {
//!         background_color: ColorScheme::Dark,
//!         nav_header: "Freyr".to_string(),
//!         nav_items: vec!["Home".to_string(), "About".to_string(), "Contact".to_string()],
//!         nav_links: vec!["/".to_string(), "/about".to_string(), "/contact".to_string()],
//!         nav_item_color: NavItemsColor::Custom("#990000"),
//!         icon_color: IconColor::Custom("#99cc00"), // Sets the color for both the hamburger SVG and the cross SVG.
//!     };
//!
//!     rsx! {
//!         Navbar { navbar_config }
//!         Outlet::<Route> {}
//!     }
//! }
//!
//! #[component]
//! fn Home() -> Element {
//!     rsx! {
//!         div {
//!             BasicButton { color: ButtonColor::Freyr, label: "Go Home", link: ButtonUrl { url: "/".to_string() } }
//!             BasicButton { color: ButtonColor::Primary, label: "Go to About", link: ButtonUrl { url: "/about".to_string() } }
//!             // Here the routing is made optional
//!             BasicButton { color: ButtonColor::Freyr, label: "Hello" }
//!
//!         }
//!     }
//! }
//!
//! #[component]
//! fn Dropdown() -> Element {
//!     let dropdown_items = vec![
//!        DropdownItem { label: String::from("Freyr"), url: String::from("/") },
//!        DropdownItem { label: "See freyr's components".to_string(), url: "/components".to_string() },
//!        DropdownItem { label: "Learn about dioxus".to_string(), url: "/learn-dioxus".to_string() },
//!    ];
//!
//!    let config_dropdown = DropdownConfig {
//!        title: String::from("Menu"),
//!        label: dropdown_items,
//!        background_color: DropdownColorScheme::Freyr,
//!        title_color: DropdownTitleColor::Light,
//!        labels_color: DropdownLabelsColor::Dark,
//!        hover_color: DropdownHoverColor::Custom("#03346E"),
//!    };
//!     rsx! {
//!         div {
//!             DropdownMenu { config_dropdown }
//!         }
//!     }
//! }
//! ```
//!
//! ## Features in Development
//! - Additional components (e.g., carousel, dropdowns)
//! - Extended customization options for more complex layouts
//!
//! ## Contribution
//! Contributions are welcome! Feel free to open an issue or submit a pull request on [GitHub](https://github.com/cbdefontenay/freyr).

mod accordion;
mod assets;
mod basic_button;
mod carousel;
mod dropdown;
mod enums;
mod nav_bar;
mod navbar_with_logo;
pub mod prelude;
mod scripts;
mod tabs;

pub use crate::accordion::{Accordion, AccordionCustom};
pub use crate::basic_button::BasicButton;
pub use crate::carousel::CarouselSimple;
pub use crate::carousel::CarouselWithNumbers;
pub use crate::carousel::CarouselWithTimer;
pub use crate::dropdown::DropdownMenu;
pub use crate::enums::accordion_enums::{AccordionColor, AccordionCustomProps, AccordionProps, AccordionIconColor};
pub use crate::enums::basic_button_enums::{ButtonColor, ButtonUrl};
pub use crate::enums::carousel_simple_enums::CarouselItem;
pub use crate::enums::dropdown_enums::{
    DropdownColorScheme, DropdownConfig, DropdownHoverColor, DropdownItem, DropdownLabelsColor,
    DropdownTitleColor,
};
pub use crate::enums::navbar_enums::{
    ColorScheme, IconColor, NavItemsColor, NavbarConfig, NavbarWithLogoConfig, NavbarDropdownConfig, DropdownConfigNavBar
};
pub use crate::enums::tabs_enums::TabsColor;
pub use crate::enums::tabs_enums::TabsProps;
pub use crate::nav_bar::Navbar;
pub use crate::nav_bar::NavbarDropdown;
pub use crate::navbar_with_logo::NavbarWithLogo;
pub use crate::tabs::Tabs;

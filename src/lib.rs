//! # Freyr UI Components Library for Dioxus.
//!
//! This crate provides a set of customizable UI components for use in Dioxus projects.
//! It allows developers to easily integrate and style components such as navbars, carousels and buttons,
//! with flexible configuration options for color schemes and layouts. Just make it your own!
//!
//! ## Why the name Freyr?
//!
//! Freyr is the norse god of fertility and prosperity and is known for his beauty. Your Dioxus web app should also look good.
//!
//! > ### **Warning:**
//! > **_This component is in the early stage of development. Right now there are only five components available: the navbar, the dropdown, the two carousel and the buttons._**
//! > **_New components will be added, and some features that already exist may change._**
//! > **_Also keep in mind that this crate is not meant to be THE alternative to existing professional UI component libraries for Dioxus. I just made it for fun, and shared it in the hope it also could help other developers._**
//!
//! ### **Features**
//! - [x] Buttons
//! - [x] Tabs
//! - [x] Navbar
//! - [x] Accordion
//! - [x] Carousel
//! - [x] Dropdown
//! - [x] Dialog
//! - [x] Spinner
//!
//! All those components have more features than you think. For more information about them please chack them [here](https://docs.rs/freyr/latest/freyr/#functions).
//! ## Key Features
//! - Full customization of colors, sizes, and layouts using configuration structs and enums.
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
//!         nav_header: Some(String::from("Freyr")),
//!         orientation: Some(Orientation::Center),
//!         header_color: HeaderColor::Light,
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
//!             BasicButton { color: ButtonColor::Freyr, label: String::from("Go Home"), link: ButtonUrl { url: "/".to_string() } }
//!             BasicButton { color: ButtonColor::Primary, label: String::from("Go to About"), link: ButtonUrl { url: "/about".to_string() } }
//!             // Here the routing is made optional
//!             BasicButton { color: ButtonColor::Freyr, label: String::from("Hello") }
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
//! For more documentation about the actual components, please go to the [functions](https://docs.rs/freyr/latest/freyr/#functions).
mod accordion;
mod assets;
mod basic_button;
mod carousel;
mod dialog;
mod dropdown;
mod enums;
mod nav_bar;
mod navbar_with_logo;
pub mod prelude;
mod scripts;
mod spinner;
mod tabs;

pub use crate::accordion::*;
pub use crate::basic_button::*;
pub use crate::basic_button::*;
pub use crate::carousel::*;
pub use crate::carousel::*;
pub use crate::carousel::*;
pub use crate::dialog::*;
pub use crate::dropdown::*;
pub use crate::enums::accordion_enums::*;
pub use crate::enums::basic_button_enums::*;
pub use crate::enums::carousel_simple_enums::*;
pub use crate::enums::dialog_enums::*;
pub use crate::enums::dropdown_enums::*;
pub use crate::enums::navbar_enums::*;
pub use crate::enums::tabs_enums::*;
pub use crate::enums::tabs_enums::*;
pub use crate::nav_bar::*;
pub use crate::spinner::*;
pub use crate::navbar_with_logo::*;
pub use crate::tabs::Tabs;

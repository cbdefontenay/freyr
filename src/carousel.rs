use dioxus::document::eval;
#[allow(non_snake_case)]

use dioxus::prelude::*;
use serde_json::json;
use crate::enums::carousel_simple_enums::{CarouselItem};
use crate::assets::carousel_simple_styles::CAROUSEL_STYLES;
use crate::scripts::carousel_script::CAROUSEL_SCRIPT;

//! A simple carousel component for Dioxus framework.
//!
//! The `CarouselSimple` component provides a minimalistic image carousel for navigating through a collection of images.
//! And the `CarouselWithTimer` one does the same, but the image changes all 5 secs by default.
//!
//! # Features
//! - Displays a single image at a time, with navigation dots for switching between images.
//! - Supports custom CSS classes for styling via the `class` prop.
//! - Dynamically updates the displayed image when a navigation dot is clicked.
//!
//! # Props
//! - `items: Vec<CarouselItem>`: A list of items to display in the carousel, where each item contains an image URL.
//! - `class: Option<String>`: An optional CSS class to customize the styling of the carousel.
//! - `alt: Vec<CarouselItem>`: A list of items containing alternate text for each image.
//!
//! # Usage
//! ```rust
//! use freyr::prelude::*;
//!
//!        const BIRD: Asset = asset!("./assets/one.jpg");
//!        const FOX: Asset = asset!("./assets/two.jpg");
//!        const DOG: Asset = asset!("./assets/three.jpg");
//!
//!        let items = vec![
//!            CarouselItem::new(BIRD, String::from("Image 1")),
//!            CarouselItem::new(FOX, String::from("Image 2")),
//!            CarouselItem::new(DOG, String::from("Image 3")),
//!        ];
//!
//!     let alt = items.clone(); // Alternate text can be reused if identical.
//!
//! rsx! {
//!     CarouselSimple {
//!         items: items,
//!         alt: alt,
//!         class: Some(String::from("w-full h-full flex justify-center rounded-lg")),
//!     }
//! }
//! ```
//! This component is suitable for straightforward carousel implementations requiring minimal external dependencies.

#[component]
pub fn CarouselSimple(
    items: Vec<CarouselItem>,
    class: Option<String>,
    alt: Vec<CarouselItem>,
) -> Element {
    let mut current_index = use_signal(|| 0);

    let carousel_class = if let Some(custom_class) = class {
        custom_class
    } else {
        "carousel-container".to_string()
    };

    rsx! {
        style { "{CAROUSEL_STYLES}" }

        div { class: "carousel-container {carousel_class}",

            img {
                src: "{items[current_index()].image_url}",
                alt: "{alt[current_index()].image_alt}",
                class: "carousel-image",
            }

            div { class: "carousel-dots",
                for index in 0..items.len() {
                    div {
                        class: if (*current_index)() == index { "carousel-dot active" } else { "carousel-dot" },
                        onclick: move |_| current_index.set(index),
                    }
                }
            }
        }
    }
}

#[component]
pub fn CarouselWithTimer(
    items: Vec<CarouselItem>,
    class: Option<String>,
    alt: Vec<CarouselItem>,
) -> Element {
    let mut current_index = use_signal(|| 0);

    let carousel_class = if let Some(custom_class) = class {
        custom_class
    } else {
        "carousel-container".to_string()
    };

    let items_cloned = items.clone();

    let items_data = items_cloned.iter().map(|item| {
        json!({
            "image_url": item.image_url.to_string(),
            "image_alt": item.image_alt,
        })
    }).collect::<Vec<_>>();

    use_effect(move || {
        let items_json = serde_json::to_string(&items_data).unwrap();
        let script = CAROUSEL_SCRIPT.replace("{items_len}", &items_cloned.len().to_string())
            .replace("{items_data}", &items_json);
        eval(&script);
    });

    rsx! {
        style { "{CAROUSEL_STYLES}" }

        div { class: "carousel-container {carousel_class}",

            img {
                id: "carousel-image",
                src: "{items[current_index()].image_url}",
                alt: "{alt[current_index()].image_alt}",
                class: "carousel-image",
            }

            div { class: "carousel-dots",
                for index in 0..items.len() {
                    div {
                        class: if (*current_index)() == index { "carousel-dot active" } else { "carousel-dot" },
                        onclick: move |_| current_index.set(index),
                    }
                }
            }
        }
    }
}

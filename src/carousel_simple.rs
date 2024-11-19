#[allow(non_snake_case)]

use dioxus::prelude::*;
use crate::enums::carousel_simple_enums::{CarouselItem};
use crate::assets::carousel_simple_styles::CAROUSEL_STYLES;

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

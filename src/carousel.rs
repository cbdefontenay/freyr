#[allow(non_snake_case)]
use crate::assets::carousel_simple_styles::CAROUSEL_STYLES;
use crate::enums::carousel_simple_enums::CarouselItem;
use crate::scripts::carousel_script::CAROUSEL_SCRIPT;
use dioxus::document::eval;
use dioxus::prelude::*;
use serde_json::json;

/// A simple carousel component for Dioxus.
///
/// The `CarouselSimple` component provides a minimalistic image carousel for navigating through a collection of images.
/// And the `CarouselWithTimer` one does the same, but the image changes all 5 secs by default.
///
/// # Features
/// - Displays a single image at a time, with navigation dots for switching between images.
/// - Supports custom CSS classes for styling via the `class` prop.
/// - Dynamically updates the displayed image when a navigation dot is clicked.
///
/// # Props
/// - `items: Vec<CarouselItem>`: A list of items to display in the carousel, where each item contains an image URL.
/// - `class: Option<String>`: An optional CSS class to customize the styling of the carousel.
/// - `alt: Vec<CarouselItem>`: A list of items containing alternate text for each image.
///
/// # Usage
/// ```rust
/// use freyr::prelude::*;
///
///        const BIRD: Asset = asset!("./assets/one.jpg");
///        const FOX: Asset = asset!("./assets/two.jpg");
///        const DOG: Asset = asset!("./assets/three.jpg");
///
///        let items = vec![
///            CarouselItem::new(BIRD, String::from("Image 1")),
///            CarouselItem::new(FOX, String::from("Image 2")),
///            CarouselItem::new(DOG, String::from("Image 3")),
///        ];
///
///     let alt = items.clone(); // Alternate text can be reused if identical.
///
/// rsx! {
///     CarouselSimple {
///         items: items,
///         alt: alt,
///         class: Some(String::from("w-full h-full flex justify-center rounded-lg")),
///     }
/// }
/// ```
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
        "carousel-container-default".to_string()
    };

    rsx! {
        style { "{CAROUSEL_STYLES}" }

        div { class: "carousel-container {carousel_class}",
            div { class: "carousel-content-wrapper",
                img {
                    src: "{items[current_index()].image_url}",
                    alt: "{alt[current_index()].image_alt}",
                    class: "carousel-image",
                }
                div { class: "carousel-dots",
                    for index in 0..items.len() {
                        div {
                            class: if current_index() == index { "carousel-dot active" } else { "carousel-dot" },
                            onclick: move |_| current_index.set(index),
                        }
                    }
                }
            }
        }
    }
}

/// Use it like the CarouselSimple component, just add the ```timer_seconds``` prop like that: ```timer_seconds: 5```
#[component]
pub fn CarouselWithTimer(
    items: Vec<CarouselItem>,
    class: Option<String>,
    alt: Vec<CarouselItem>,
    timer_seconds: u64,
) -> Element {
    let mut current_index = use_signal(|| 0);

    let carousel_class = if let Some(custom_class) = class {
        custom_class
    } else {
        "carousel-container-default".to_string()
    };

    let items_cloned = items.clone();

    let items_data = items_cloned
        .iter()
        .map(|item| {
            json!({
                "image_url": item.image_url.to_string(),
                "image_alt": item.image_alt,
            })
        })
        .collect::<Vec<_>>();

    let timer_ms = timer_seconds * 1000;

    use_effect(move || {
        let items_json = serde_json::to_string(&items_data).unwrap();
        let script = CAROUSEL_SCRIPT
            .replace("{initial_index}", &current_index().to_string())
            .replace("{items_len}", &items_cloned.len().to_string())
            .replace("{items_data}", &items_json)
            .replace("{timer_ms}", &timer_ms.to_string());
        eval(&script);
    });

    rsx! {
        style { "{CAROUSEL_STYLES}" }

        div { class: "carousel-container {carousel_class}",
            div { class: "carousel-content-wrapper",
                img {
                    id: "carousel-image",
                    src: "{items[current_index()].image_url}",
                    alt: "{alt[current_index()].image_alt}",
                    class: "carousel-image",
                }

                div { class: "carousel-dots",
                    for index in 0..items.len() {
                        div {
                            class: if current_index() == index { "carousel-dot active" } else { "carousel-dot" },
                            onclick: move |_| {
                                current_index.set(index);
                                eval(&format!("window.setCarouselIndex({})", index));
                            },
                        }
                    }
                }
            }
        }
    }
}

/// To be used like the CarouselSimple component.
#[component]
pub fn CarouselWithNumbers(
    items: Vec<CarouselItem>,
    class: Option<String>,
    alt: Vec<CarouselItem>,
) -> Element {
    let mut current_index = use_signal(|| 0);

    let carousel_class = if let Some(custom_class) = class {
        custom_class
    } else {
        "carousel-container-default".to_string()
    };

    rsx! {
        style { "{CAROUSEL_STYLES}" }

        div { class: "carousel-container {carousel_class}",
            div { class: "carousel-content-wrapper",
                img {
                    src: "{items[current_index()].image_url}",
                    alt: "{alt[current_index()].image_alt}",
                    class: "carousel-image",
                }

                div { class: "carousel-numbers",
                    for index in 0..items.len() {
                        div {
                            class: if (*current_index)() == index { "carousel-number active" } else { "carousel-number" },
                            onclick: move |_| current_index.set(index),
                            "{index + 1}"
                        }
                    }
                }
            }
        }
    }
}

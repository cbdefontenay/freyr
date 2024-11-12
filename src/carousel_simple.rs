use dioxus::prelude::*;
use crate::enums::carousel_simple_enums::{CarouselItem, CarouselSize};
use crate::assets::carousel_simple_styles::CAROUSEL_STYLES;

/// A simple carousel component that displays a set of images with navigation dots.
///
/// This component is used to create a carousel of images. It displays a set of `CarouselItem`
/// images and allows the user to navigate between them using navigation dots. The size of the
/// carousel can be adjusted using the `size` prop, and the current index of the carousel is
/// tracked internally.
///
/// # Props
/// - `items`: A vector of `CarouselItem` objects. Each item represents an image and its optional caption.
/// - `size`: A `CarouselSize` variant that determines the size of the carousel (e.g., Small, Medium, or Large).
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use freyr::prelude::*;
///
/// #[component]
/// fn Home() -> Element {
///     const BIRD: Asset = asset!("./assets/bird.jpg");
///     const FOX: Asset = asset!("./assets/fox.jpg");
///     const DOG: Asset = asset!("./assets/dog.jpg");
///
///     let items = vec![
///         CarouselItem::new(BIRD, Some("Caption 1".into())),
///         CarouselItem::new(FOX, Some("Caption 2".into())),
///         CarouselItem::new(DOG, Some("Caption 3".into())),
///     ];
///
///     let size = CarouselSize::Medium;  // Set the carousel size to Medium
///
///     rsx! {
///         div {
///             style: "padding: 60px;",
///             CarouselSimple { items: items, size: size }
///         }
///     }
/// }
/// ```
///
/// In this example, we create a vector of `CarouselItem` objects, each containing an image URL
/// and an optional caption. The `CarouselSimple` component is then rendered with the provided
/// `items` and `size` properties.
#[component]
pub fn CarouselSimple(items: Vec<CarouselItem>, size: CarouselSize) -> Element {
    let mut current_index = use_signal(|| 0);

    rsx! {
        style { "{CAROUSEL_STYLES}" }

        div {
            class: "carousel-container",
            style: "{size.to_css_size()}",

            // Display the current image
            img {
                src: "{items[current_index()].image_url}",
                alt: "Carousel Image",
                class: "carousel-image",
            }

            // Render navigation dots
            div {
                class: "carousel-dots",
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

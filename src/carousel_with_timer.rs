#[allow(non_snake_case)]

use dioxus::prelude::*;
use crate::enums::carousel_simple_enums::{CarouselItem};
use crate::assets::carousel_simple_styles::CAROUSEL_STYLES;
use dioxus::document::eval;
use serde_json::json;

const CAROUSEL_SCRIPT: &str = r#"
    let index = 0;
    const itemLength = {items.len};
    const items = {items_data};

    const currentIndexElement = document.getElementById("carousel-image");
    const dots = document.querySelectorAll(".carousel-dot");

    setInterval(() => {
        index = (index + 1) % itemLength;
        currentIndexElement.src = items[index].image_url;
        currentIndexElement.alt = items[index].image_alt;

        dots.forEach(dot => dot.classList.remove('active'));
        dots[index].classList.add('active');
    }, 5000);
"#;

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
        // Clone the data needed for the async block to ensure it has a `'static` lifetime
        let items_data = items_data.clone();
        let items_len = items_cloned.len().to_string();

        spawn(async move {
            // Serialize the items data for use in the script
            let items_json = serde_json::to_string(&items_data).unwrap();

            // Replace placeholders in the script with actual values
            let script = CAROUSEL_SCRIPT
                .replace("{items_len}", &items_len)
                .replace("{items_data}", &items_json);

            // Execute the script in the browser
            let _ = eval(&script).await;
        });

        ()
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


// TODO: wasm is not working, try fix it
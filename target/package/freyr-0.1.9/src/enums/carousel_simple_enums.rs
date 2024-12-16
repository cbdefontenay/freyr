use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct CarouselItem {
    pub image_url: Asset,
    pub image_alt: String,
}

impl CarouselItem {
    pub fn new(image_url: impl Into<Asset>, image_alt: String) -> Self {
        Self {
            image_url: image_url.into(),
            image_alt: image_alt.into(),
        }
    }
}

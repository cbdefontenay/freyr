use dioxus::prelude::*;

/// Enum for different carousel sizes.
#[derive(PartialEq, Clone)]
pub enum CarouselSize {
    Small,
    Medium,
    Large,
}

impl CarouselSize {
    /// Converts the `CarouselSize` variant to its corresponding CSS size string.
    ///
    /// # Arguments
    ///
    /// * `self` - The variant of `CarouselSize` to convert.
    ///
    /// # Returns
    ///
    /// A string representing the CSS size for the carousel.
    pub fn to_css_size(&self) -> &'static str {
        match self {
            CarouselSize::Small => "width: 300px; height: 200px;",
            CarouselSize::Medium => "width: 600px; height: 400px;",
            CarouselSize::Large => "width: 900px; height: 600px;",
        }
    }
}

/// Struct representing each item in the carousel.
#[derive(PartialEq, Clone)]
pub struct CarouselItem {
    pub image_url: Asset,  // Image URL as Asset type
    pub caption: Option<String>,
}

impl CarouselItem {
    /// Creates a new `CarouselItem` with the provided image URL and an optional caption.
    ///
    /// # Arguments
    ///
    /// * `image_url` - The URL of the image to display in the carousel.
    /// * `caption` - An optional caption for the carousel item.
    ///
    /// # Returns
    ///
    /// A new `CarouselItem` instance.
    pub fn new(image_url: impl Into<Asset>, caption: Option<String>) -> Self {
        Self {
            image_url: image_url.into(),
            caption,
        }
    }
}

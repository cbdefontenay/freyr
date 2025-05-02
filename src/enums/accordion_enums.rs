use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum AccordionColor {
    Freyr,
    Black,
}

impl AccordionColor {
    pub fn to_css_class(&self) -> &'static str {
        match self {
            AccordionColor::Freyr => "accordion-freyr",
            AccordionColor::Black => "accordion-black",
        }
    }
}

/// Defines different color options for navigation items.
#[derive(PartialEq, Clone)]
pub enum AccordionIconColor {
    Freyr,
    Dark,
    Light,
    Custom(&'static str),
}

impl AccordionIconColor {
    /// Returns the CSS class or custom color for the navigation items.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            AccordionIconColor::Freyr => "#3795BD",
            AccordionIconColor::Dark => "#000",
            AccordionIconColor::Light => "#fff",
            AccordionIconColor::Custom(color) => color,
        }
    }
}

/// Props are: ```title```, ```accordion_text```, ```optional_text```, ```icon_color```, ```class```, ```title_class```, ```accordion_wrapper```, ```accordion_content```
#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    pub title: String,
    pub accordion_text: String,
    pub optional_text: Option<String>,
    pub icon_color: AccordionIconColor,
    pub class: Option<String>,
    pub title_class: Option<String>,
    pub accordion_wrapper: Option<String>,
    pub accordion_content: Option<String>,
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionCustomProps {
    pub title: String,
    pub accordion_text: Element,
    pub icon_color: AccordionIconColor,
    pub class: Option<String>,
    pub title_class: Option<String>,
    pub accordion_wrapper: Option<String>,
    pub accordion_content: Option<String>,
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionLeftProps {
    pub accordion_width: Option<String>,
    pub accordion_header_bg_color: Option<String>,
    pub title: String,
    pub children: Element,
    pub accordion_open_max_height: Option<String>,
    pub box_shadow: Option<String>,
    pub border_radius: Option<String>,
    pub title_color: Option<String>,
    pub title_font_size: Option<String>,
    pub title_font_weight: Option<String>,
    pub content_padding: Option<String>,

    #[props(default = AccordionIconColor::Freyr)]
    pub icon_color: AccordionIconColor,
}
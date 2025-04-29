use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum TabsColor {
    Freyr,
    Black,
    Light,
    Custom(&'static str),
}

impl TabsColor {
    pub fn to_css_class(&self) -> &'static str {
        match self {
            TabsColor::Freyr => "tabs-freyr",
            TabsColor::Black => "tabs-black",
            TabsColor::Light => "tabs-light",
            TabsColor::Custom(_) => "",
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct TabsProps {
    pub tabs_names: Vec<String>,
    pub custom_texts: Option<Vec<Element>>,
    pub custom_color: Option<TabsColor>,
}

#[derive(PartialEq, Clone, Props)]
pub struct TabsSecondaryProps {
    pub tabs_names: Vec<String>,
    pub custom_texts: Option<Vec<Element>>,
    pub tab_max_width: Option<String>,
    pub tab_header_hover: Option<String>,
    pub tabs_header_class: Option<String>,
    pub header_bg_color: Option<String>,
    pub header_text_color: Option<String>,
    pub active_bg_color: Option<String>,
    pub active_text_color: Option<String>,
    pub tab_radius: Option<String>,
    pub tab_shadow: Option<String>,
}
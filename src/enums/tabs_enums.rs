use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum TabsColor {
    Freyr,
    Black,
}

impl TabsColor {
    pub fn to_css_class(&self) -> &'static str {
        match self {
            TabsColor::Freyr => "tabs-freyr",
            TabsColor::Black => "tabs-black",
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct TabsProps {
    pub tabs_names: Vec<String>,
    pub tabs_texts: Vec<String>,
    pub custom_titles: Option<Vec<Element>>,
    pub custom_texts: Option<Vec<Element>>,
    pub custom_color: Option<TabsColor>,
}
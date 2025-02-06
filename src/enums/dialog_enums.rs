use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DialogProps {
    pub label: String,
    pub dialog_content: Option<Element>,
    pub dialog_button_class: Option<String>,
    pub wrap_class: String,
    pub button_class: String,
    pub close_button_class: Option<String>,
    pub close_button_label: Option<String>,
    pub cross_svg_class: Option<String>,
}
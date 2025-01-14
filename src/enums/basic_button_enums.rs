use dioxus::events::MouseEvent;
use dioxus::prelude::*;

/// Enum that defines which color you want to use for the button.
#[derive(PartialEq, Clone)]
pub enum ButtonColor {
    Freyr,
    Primary,
    Success,
    Danger,
    Black,
    Transparent,
}

impl ButtonColor {
    pub fn to_css_class(&self) -> &'static str {
        match self {
            ButtonColor::Freyr => "btn-freyr",
            ButtonColor::Primary => "btn-primary",
            ButtonColor::Success => "btn-success",
            ButtonColor::Danger => "btn-danger",
            ButtonColor::Black => "btn-black",
            ButtonColor::Transparent => "btn-transparent",
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct ButtonUrl {
    pub url: String,
}

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    pub color: ButtonColor,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}
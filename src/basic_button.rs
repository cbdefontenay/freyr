use dioxus::prelude::*;
use crate::assets::button_style::BUTTON_STYLES;

#[derive(PartialEq, Clone)]
pub enum ButtonColor {
    Default,
    Primary,
    Success,
    Danger,
    Black,
    Transparent,
}

impl ButtonColor {
    pub fn to_css_class(&self) -> &'static str {
        match self {
            ButtonColor::Default => "btn-default",
            ButtonColor::Primary => "btn-primary",
            ButtonColor::Success => "btn-success",
            ButtonColor::Danger => "btn-danger",
            ButtonColor::Black => "btn-black",
            ButtonColor::Transparent => "btn-transparent",
        }
    }
}

#[component]
pub fn BasicButton(color: ButtonColor, label: &'static str) -> Element {
    let style_tag = rsx! {
        style { "{BUTTON_STYLES}" }
    };

    rsx! {
        div {
            {style_tag}
            button {
                class: "{color.to_css_class()}",
                "{label}"
            }
        }
    }
}

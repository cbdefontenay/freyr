use dioxus::prelude::*;
mod assets;
use assets::button_style::BUTTON_STYLES;

#[derive(PartialEq, Clone)]
pub enum ButtonColor {
    Default,
    Primary,
    Success,
    Danger,
}

impl ButtonColor {
    fn to_css_class(&self) -> &'static str {
        match self {
            ButtonColor::Default => "btn-default",
            ButtonColor::Primary => "btn-primary",
            ButtonColor::Success => "btn-success",
            ButtonColor::Danger => "btn-danger",
        }
    }
}

#[component]
pub fn CustomButton(color: ButtonColor, label: &'static str) -> Element {
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

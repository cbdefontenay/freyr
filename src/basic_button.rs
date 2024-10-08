use dioxus::prelude::*;
use crate::assets::button_style::BUTTON_STYLES;

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

/// Defines which kind of button you want to use: Freyr, Primary, Success, Danger, Black or Transparent.
///
/// # Examples
///
/// Using a "Freyr", and a "Transparent" button:
///
/// ```rust
/// BasicButton { color: ButtonColor::Freyr, label: "Freyr!" }
/// BasicButton { color: ButtonColor::Transparent, label: "I am transparent" }
/// ```
///
/// Those buttons have arbitrary colors, that you may not customize to you wishes.
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

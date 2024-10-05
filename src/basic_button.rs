use dioxus::prelude::*;
use crate::assets::button_style::BUTTON_STYLES;

/// Enum that defines which color you want to use for the button.
///
///  # Colors available:
/// Default (gray), Primary (blue), Success (green), Danger (red), Black (black), Transparent (transparent).
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

/// Defines which kind of button you want to use: Default, Primary, Success, Danger, Black or Transparent.
///
/// # Examples
///
/// Using a "Success", and a "Transparent" button:
///
/// ```rust
/// BasicButton { color: ButtonColor::Success, label: "Success!" }
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

use dioxus::prelude::*;
use crate::assets::button_style::BUTTON_STYLES;
use crate::enums::basic_button_enums::{ButtonColor, ButtonUrl};

/// Defines which kind of button you want to use: Freyr, Primary, Success, Danger, Black or Transparent.
/// You may also add a route.
///
/// # Examples
///
/// Using a "Freyr", and a "Transparent" button:
///
/// ```rust
/// BasicButton { color: ButtonColor::Primary, label: "Go Home", link: ButtonUrl { url: "/".to_string() } }
/// BasicButton { color: ButtonColor::Transparent, label: "Go to About", link: ButtonUrl { url: "/about".to_string() } }
/// ```
///
/// Those buttons have arbitrary colors, that you may not customize to you wishes.
#[component]
pub fn BasicButton(color: ButtonColor, label: &'static str, link: ButtonUrl) -> Element {
    let style_tag = rsx! {
        style { "{BUTTON_STYLES}" }
    };

    rsx! {
        div {
            {style_tag}
            Link {
                to: "{link.url}",   // Use `link.url` to set the navigation URL
                button {
                    class: "{color.to_css_class()}",
                    "{label}"
                }
            }
        }
    }
}
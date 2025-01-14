use crate::assets::button_style::BUTTON_STYLES;
use crate::enums::basic_button_enums::{ButtonColor, ButtonProps, ButtonUrl};
use dioxus::prelude::*;

/// Defines which kind of button you want to use: Freyr, Primary, Success, Danger, Black or Transparent.
/// You may also add a route.
///
/// # Examples
///
/// Using a "Freyr", and a "Transparent" button:
///
/// ```rust
/// BasicButton { color: ButtonColor::Primary, label: "Go Home", link: ButtonUrl { url: "/".to_string() } }
///
/// // Here the routing is made optional
/// BasicButton { color: ButtonColor::Freyr, label: "Hello" }
/// ```
///
/// Those buttons have arbitrary colors, that you may not customize to you wishes.
#[component]
pub fn BasicButton(color: ButtonColor, label: &'static str, link: Option<ButtonUrl>) -> Element {
    let style_tag = rsx! {
        style { "{BUTTON_STYLES}" }
    };

    rsx! {
        div {
            {style_tag}
            if let Some(link) = link {
                Link { to: "{link.url}",
                    button { class: "{color.to_css_class()}", "{label}" }
                }
            } else {
                button { class: "{color.to_css_class()}", "{label}" }
            }
        }
    }
}

/// Usage of the button with an event attached to it:
///
/// ```rust
/// #[component]
/// pub fn Hero() -> Element {
///     let mut count = use_signal(|| 0);
///     let increment = move |_| count += 1;
///
///     rsx! {
///         div { class: "mt-20 w-full flex items-center justify-center",
///             EventButton {
///                 color: ButtonColor::Freyr,
///                 onclick: increment,
///                 label: "Increment".to_string(),
///             }
///             div { class: "ml-4", "{count}" }
///         }
///     }
/// }
/// ```
#[component]
pub fn EventButton(props: ButtonProps) -> Element {
    let style_tag = rsx! {
        style { "{BUTTON_STYLES}" }
    };

    rsx! {
        div {
            {style_tag}
            button {
                class: "{props.color.to_css_class()}",
                onclick: move |evt| (props.onclick)(evt),
                {props.label}
            }
        }
    }
}
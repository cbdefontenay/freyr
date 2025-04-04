use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SpinnerProps {
    pub height: Option<String>,
    pub width: Option<String>,
    pub spinner_color: Option<String>,
    pub spinner_bg_color: Option<String>,
}
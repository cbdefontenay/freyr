use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum AccordionColor {
    Freyr,
    Black,
}

impl AccordionColor {
    pub fn to_css_class(&self) -> &'static str {
        match self {
            AccordionColor::Freyr => "accordion-freyr",
            AccordionColor::Black => "accordion-black",
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    pub title: String,
    pub accordion_text: String,
    pub class: Option<String>,
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionCustomProps {
    pub title: String,
    pub text: Element,
    pub class: Option<String>,
}

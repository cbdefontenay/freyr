use dioxus::core_macro::Props;

#[derive(PartialEq, Props, Clone)]
pub struct AccordionProps {
    title: String,
    accordion_text: String,
}
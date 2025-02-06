#[allow(non_snake_case)]
use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("src/output.css");

#[derive(Props, PartialEq, Clone)]
pub struct DialogProps {
    pub label: String,
    pub dialog_content: Option<Element>,
    pub dialog_button_class: Option<String>,
    pub wrap_class: String,
    pub button_class: String,
    pub close_button_class: Option<String>,
    pub close_button_label: Option<String>,
    pub cross_svg_class: Option<String>,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let mut show_modal = use_signal(|| false);

    rsx! {
        div {
            button {
                class: if let Some(class) = &props.dialog_button_class { {class.to_string()} } else { "bg-blue-600 text-white font-bold py-2 px-4 rounded-lg hover:bg-blue-700 transition" },
                onclick: move |_| show_modal.set(!show_modal()),
                {props.label}
            }

            if show_modal() {
                div { class: "fixed inset-0 backdrop-blur-md flex items-center justify-center z-50",
                    div { class: "{props.wrap_class} relative",

                        button {
                            class: "absolute top-2 right-2 p-2 rounded-full hover:bg-gray-200 transition",
                            onclick: move |_| show_modal.set(false),
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                class: if let Some(class) = &props.cross_svg_class {
                                    { class.to_string() }
                                } else {
                                    "w-6 h-6 text-gray-500 hover:text-gray-700".to_string()
                                },
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M6 18L18 6M6 6l12 12"
                                }
                            }
                        }

                        if let Some(content) = &props.dialog_content {
                            div { {content} }
                        } else {
                            p { class: "text-gray-700 mb-4", "You may add some text here." }
                        }

                        div { class: "flex justify-end space-x-2 mt-4",
                            button {
                                class: if let Some(class) = &props.close_button_class { {class.to_string()} } else { "bg-red-600 text-white px-4 py-2 rounded-lg hover:bg-red-700 transition" },
                                onclick: move |_| show_modal.set(false),
                                if let Some(label) = &props.close_button_label {
                                    {label.clone()}
                                } else {
                                    "Close"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


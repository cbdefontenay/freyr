use crate::assets::dialog_styles::DIALOG_STYLES;
use crate::DialogWithoutButtonProps;
#[allow(non_snake_case)]
use dioxus::prelude::*;

/// Usage with Tailwind (but Tailwind is an option):
/// ```rust
/// use dioxus::prelude::Element;
///
/// #[component]
/// fn MyDialog() -> Element {
///  let mut show_dialog = use_signal(|| false);
///
///     let dialog_props = DialogWithoutButtonProps {
///         show_modal: show_dialog,
///         dialog_content: Some(rsx! {
///         h2 { class: "text-xl font-semibold mb-4", "Add a new To-Do" }
///         input {
///             r#type: "text",
///             class: "border rounded p-2 w-full",
///             placeholder: "Enter task..."
///         }
///     }),
///         wrap_class: "bg-white rounded-lg shadow-lg w-full max-w-md p-6 z-50 relative".to_string(),
///         close_button_label: Some("Close".to_string()),
///         close_button_class: Some(
///             "mt-4 bg-purple-800 text-white px-4 py-2 rounded-lg hover:bg-purple-900 transition".to_string()
///         ),
///         cross_svg_class: Some("w-6 h-6 absolute top-4 right-4 text-gray-500 hover:text-black cursor-pointer".to_string()),
///     };
///
///
///     rsx! {
///         div { class: "min-h-screen bg-gray-100 flex items-center justify-center",
///             button {
///                 class: "bg-blue-800 text-white font-bold py-2 px-4 rounded-lg hover:bg-blue-900 transition",
///                 onclick: move |_| show_dialog.set(true),
///                 "Show Dialog"
///             }
///
///             DialogWithoutButton { ..dialog_props }
///         }
///     }
/// ```
#[component]
pub fn Dialog(mut props: DialogWithoutButtonProps) -> Element {
    rsx! {
        style { "{DIALOG_STYLES}" }
        div {
            if *props.show_modal.read() {
                div { class: "dialog-overlay",
                    div {
                        class: if props.wrap_class.is_empty() {
                            "dialog-wrap".to_string()
                        } else {
                            props.wrap_class.clone()
                        },

                        button {
                            class: "dialog-close-icon",
                            onclick: move |_| props.show_modal.set(false),
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                class: if let Some(class) = &props.cross_svg_class {
                                    class.to_string()
                                } else {
                                    "dialog-cross-svg".to_string()
                                },
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M6 18L18 6M6 6l12 12",
                                }
                            }
                        }

                        if let Some(content) = &props.dialog_content {
                            div { class: "dialog-body", {content} }
                        } else {
                            p { class: "dialog-placeholder", "You may add some text here." }
                        }

                        div { class: "dialog-actions",
                            button {
                                class: if let Some(class) = &props.close_button_class {
                                    class.to_string()
                                } else {
                                    "dialog-close-button"
                                },
                                onclick: move |_| props.show_modal.set(false),
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
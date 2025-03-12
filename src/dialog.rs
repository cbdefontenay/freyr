use crate::enums::dialog_enums::DialogProps;
#[allow(non_snake_case)]
use dioxus::prelude::*;
use crate::DialogWithoutButtonProps;

const TAILWIND_CSS: Asset = asset!("/src/output.css");

/// Usage example of the ```dialog``` component:
/// ```rust
/// #[component]
/// pub fn DialogComponent() -> Element {
///     let mut todos: Signal<Vec<String>> = use_signal(|| vec![]);
///     let mut new_todo: Signal<String> = use_signal(|| "".to_string());
///
///     let dialog_props = DialogProps {
///         label: "Add todo".to_string(),
///         dialog_button_class: Some(
///             "bg-green-800 text-white font-bold py-2 px-4 rounded-lg hover:bg-green-700 transition"
///                 .to_string(),
///         ),
///         dialog_content: Some(rsx! {
///             h2 { class: "text-xl font-semibold mb-4", "Add a new To-Do" }
///             input {
///                 class: "border border-gray-300 rounded-md p-2 w-full",
///                 r#type: "text",
///                 value: "{new_todo()}",
///                 autofocus: true,
///                 oninput: move |e| new_todo.set(e.value().clone())
///             }
///             button {
///                 class: "bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition mt-4",
///                 onclick: move |_| {
///                     if !new_todo().is_empty() {
///                         todos.write().push(new_todo().clone());
///                         new_todo.set("".to_string());
///                     }
///                 },
///                 "Add Todo"
///             }
///         }),
///         wrap_class: "bg-white rounded-lg shadow-lg w-full max-w-md p-6".to_string(),
///         close_button_label: Some("Close".to_string()),
///         close_button_class: Some(
///             "bg-red-600 text-white px-4 py-2 rounded-lg hover:bg-red-700 transition".to_string(),
///         ),
///         cross_svg_class: None,
///     };
///
///     rsx! {
///         div { class: "mt-10 ml-20",
///             Dialog {
///                 label: dialog_props.label,
///                 dialog_button_class: dialog_props.dialog_button_class,
///                 dialog_content: dialog_props.dialog_content,
///                 wrap_class: dialog_props.wrap_class,
///                 close_button_label: dialog_props.close_button_label,
///                 close_button_class: dialog_props.close_button_class,
///                 cross_svg_class: dialog_props.cross_svg_class,
///             }
///             ul { class: "mt-6",
///                 for todo in todos().iter() {
///                     li { class: "p-2 border-b border-gray-300", "{todo}" }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let mut show_modal = use_signal(|| false);

    rsx! {
        div {
            button {
                class: if let Some(class) = &props.dialog_button_class { class.to_string() } else { "bg-blue-600 text-white font-bold py-2 px-4 rounded-lg hover:bg-blue-700 transition" },
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
                                class: if let Some(class) = &props.cross_svg_class { class.to_string() } else { "w-6 h-6 text-gray-500 hover:text-gray-700".to_string() },
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M6 18L18 6M6 6l12 12",
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
                                class: if let Some(class) = &props.close_button_class { class.to_string() } else { "bg-red-600 text-white px-4 py-2 rounded-lg hover:bg-red-700 transition" },
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

/// This dialog is similar to the ```Dialog``` component but does not get a starting button to display it.
/// Instead, you have to create your own button or whatever you want to activate it.
///
/// Usage:
/// ```rust
/// use dioxus::prelude::Element;
///
/// #[component]
/// fn MyDialog() -> Element {
///  let mut show_dialog = use_signal(|| false);
///
/// let dialog_props = DialogWithoutButtonProps {
///     show_modal: show_dialog,
///     dialog_content: Some(rsx! {
///         h2 { class: "text-xl font-semibold mb-4", "Add a new To-Do" }
///     }),
///     wrap_class: "bg-white rounded-lg shadow-lg w-full max-w-md p-6".to_string(),
///     close_button_label: Some("Close".to_string()),
///     close_button_class: Some(
///         "bg-purple-800 text-white px-4 py-2 rounded-lg hover:bg-purple-900 transition".to_string(),
///     ),
///     cross_svg_class: None,
/// };
///
/// rsx!{
///     button {
///         class: "bg-blue-800 text-white font-bold py-2 px-4 rounded-lg hover:bg-blue-900 transition",
///         onclick: move |_| show_dialog.set(!show_dialog()),
///         "Show Dialog"
///     }
///         DialogWithoutButton { ..dialog_props }
///     }
/// }
/// ```
#[component]
pub fn DialogWithoutButton(mut props: DialogWithoutButtonProps) -> Element {
    rsx! {
        div {
            if *props.show_modal.read() {
                div { class: "fixed inset-0 backdrop-blur-md flex items-center justify-center z-50",
                    div { class: "{props.wrap_class} relative",

                        button {
                            class: "absolute top-2 right-2 p-2 rounded-full hover:bg-gray-200 transition",
                            onclick: move |_| props.show_modal.set(false),
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                class: if let Some(class) = &props.cross_svg_class { class.to_string() } else { "w-6 h-6 text-gray-500 hover:text-gray-700".to_string() },
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M6 18L18 6M6 6l12 12",
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
                                class: if let Some(class) = &props.close_button_class { class.to_string() } else { "bg-red-600 text-white px-4 py-2 rounded-lg hover:bg-red-700 transition" },
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
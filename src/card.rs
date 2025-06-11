use crate::assets::card_styles::CARD_STYLES;
use dioxus::prelude::*;

/// Code example of the Card component:
/// ```rust
/// rsx! {
///     div { class: "mt-20 flex w-full px-4 flex-col md:flex-row gap-6",
///         FreyrCard {
///             has_shadow: true,
///             class: Some("w-full md:w-[700px]".to_string()),
///             children: rsx! {
///                 div { class: "flex flex-col bg-[#eb4034] text-slate-100 rounded-xl overflow-hidden",
///                     // Header
///                     div { class: "px-6 pt-8 pb-4",
///                         h1 { class: "text-3xl font-bold mb-2", "Welcome to FreyrCard" }
///                         p { class: "text-slate-200 text-sm", "This is a test card with styled content." }
///                     }
///                
///                     // Divider
///                     div { class: "border-t border-red-300 mx-6" }
///                
///                     // Body
///                     div { class: "px-6 py-6 space-y-3 text-base",
///                         p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
///                         p { "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
///                         p { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris." }
///                         p { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore." }
///                     }
///                
///                     // Footer
///                     div { class: "bg-[#d6372e] px-6 py-4 text-right text-sm italic",
///                         "This card was styled using Dioxus + Tailwind."
///                     }
///                 }
///             },
///         }
///     }
/// }
/// ```
#[component]
pub fn FreyrCard(
    children: Element,
    has_shadow: bool,
    class: Option<String>,
) -> Element {
    let base_class = if has_shadow {
        "card-wrapper card-shadow"
    } else {
        "card-wrapper"
    };

    let class_attr = match class {
        Some(extra) => format!("{base_class} {extra}"),
        None => base_class.to_string(),
    };

    rsx! {
        style { "{CARD_STYLES}" }
        div {
            class: "{class_attr}",
            {children}
        }
    }
}
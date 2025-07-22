# Freyr UI web Components Library for Dioxus.

## Why the name Freyr?

Freyr is the norse god of fertility and prosperity and is known for his beauty. Your Dioxus app should also be as beautiful as the god Freyr is.

## Documentation:

#### See the documentation at this link to see how to implement all components in your web app: [freyr doc](https://docs.rs/freyr/latest/freyr/)
#### If you wanna see a showcase of the UI components, please go to that website: [freyr showcase](https://freyr-doc.onrender.com/). 
Find the GitHub repo here: [freyr doc repo](https://github.com/cbdefontenay/freyr-doc).
> _Please note that the showcase website is still under development and may improve over time._

## **First**:
```bash
cargo add freyr
```

This crate provides a set of customizable UI components for use in Dioxus projects.
It allows developers to easily integrate and style components such as navbars and buttons,
with flexible configuration options for color schemes, layouts, and responsiveness.

> ### **Warning:**
> Some components may change a bit, so keep that in mind, but overall they work just fine.
> **Freyr was created out of fun to help me reuse the components I had already used in other project, while I was learning _Rust_ and Dioxus. So I also hope it's going to help you too.**

### **Features**
- [x] Buttons
- [x] Tabs
- [x] Navbar
- [x] Carousel
- [x] Accordion
- [x] Dropdown
- [x] Dialog
- [x] Card
- [x] Spinner

All those components have more features than you think. For more information about them please chack them [here](https://docs.rs/freyr/latest/freyr/#functions). 

### Example of the use of the navbar with a dropdown menu that uses **dioxus-i18n**:

 ```rust
use dioxus::prelude::*;
use freyr::prelude::*;

#[component]
pub fn Navigation() -> Element {
    let mut i18n = i18n();

    let change_to_english = move |_| i18n.set_language(langid!("en-US"));
    let change_to_french = move |_| i18n.set_language(langid!("fr-FR"));

    let dropdown_items = vec!["English".to_string(), "Français".to_string()];

    let onclick_handlers: Vec<EventHandler<MouseEvent>> = vec![
        EventHandler::new(change_to_english),
        EventHandler::new(change_to_french),
    ];

    let config_dropdown = DropdownButtonConfig {
        title: t!("languages"),
        labels: dropdown_items,
        onclick: onclick_handlers,
        background_color: DropdownColorScheme::Dark,
        title_color: DropdownTitleColor::Light,
        labels_color: DropdownLabelsColor::Light,
        hover_color: DropdownHoverColor::Custom("#03346E"),
    };

    let navbar_config = NavbarConfig {
        background_color: ColorScheme::Freyr,
        nav_header: Some(String::from("Freyr")),
        orientation: Some(Orientation::Center),
        header_color: HeaderColor::Light,
        nav_items: vec![
            "Home".to_string(),
            t!("about"),
            "Contact".to_string(),
        ],
        nav_links: vec![
            "/".to_string(),
            "/about".to_string(),
            "/contact".to_string(),
        ],
        nav_item_color: NavItemsColor::Light,
        icon_color: IconColor::White,
    };


    rsx! {
        NavbarDropdownButtons { navbar_config, config_dropdown }
        Outlet::<Route> {}
    }
}
 ```

### Other example of a **card component** (You do not need ```Tailwind```, this is just an example):
```rust
rsx! {
    div { class: "mt-20 flex w-full px-4 flex-col md:flex-row gap-6",
        FreyrCard {
            has_shadow: true,
            class: Some("w-full md:w-[700px]".to_string()),
            children: rsx! {
                div { class: "flex flex-col bg-[#eb4034] text-slate-100 rounded-xl overflow-hidden",
                    // Header
                    div { class: "px-6 pt-8 pb-4",
                        h1 { class: "text-3xl font-bold mb-2", "Welcome to FreyrCard" }
                        p { class: "text-slate-200 text-sm", "This is a test card with styled content." }
                    }
               
                    // Divider
                    div { class: "border-t border-red-300 mx-6" }
               
                    // Body
                    div { class: "px-6 py-6 space-y-3 text-base",
                        p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
                        p { "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
                        p { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris." }
                        p { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore." }
                    }
               
                    // Footer
                    div { class: "bg-[#d6372e] px-6 py-4 text-right text-sm italic",
                        "This card was styled using Dioxus + Tailwind."
                    }
                }
            },
        }
    }
}
```

For more information and documentation about freyr, please go to [docs.rs](https://docs.rs/freyr/latest/freyr/). There you may have a look at the different options that each component may provide you.

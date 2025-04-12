# Freyr UI web Components Library for Dioxus.

## Why the name Freyr?

Freyr is the norse god of fertility and prosperity and is known for his beauty. Your Dioxus app should also look good,
even beautiful.

## Documentation:

#### See the documentation at this link to see how to implement all components in your web app: [freyr doc](https://docs.rs/freyr/latest/freyr/)
#### If you wanna see a showcase of the UI components, please go to that website: [freyr showcase](https://freyr-doc.onrender.com/)
> _Please note that the showcase website is still under development and may improve over time._

## **First**:
```bash
cargo add freyr
```

This crate provides a set of customizable UI components for use in Dioxus projects.
It allows developers to easily integrate and style components such as navbars and buttons,
with flexible configuration options for color schemes, layouts, and responsiveness.

> ### **Warning:**
> **This component is in the early stage of development. Components may change. Be aware of that.**
> **Also keep in mind that this crate is not meant to be THE alternative to existing professional UI component libraries for Dioxus. I just made it for fun, and shared it in the hope it also could help other developers**
### **Features**
- [x] Buttons
- [x] Tabs
- [x] Navbar
- [x] Carousel
- [x] Accordion
- [x] Dropdown
- [x] Dialog
- [x] Spinner

All those components have more features than you think. For more information about them please chack them [here](https://docs.rs/freyr/latest/freyr/#functions). 

## Example of the use of the navbar with a dropdown menu that uses **dioxus-i18n** (_using the version 0.6.3 of Dioxus_):

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
        nav_header: String::from("Freyr"),
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

For more information and documentation about freyr, please go to [docs.rs](https://docs.rs/freyr/latest/freyr/). There you may have a look at the different options that each component may provide you.

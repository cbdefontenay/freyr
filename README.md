# Freyr UI web Components Library for Dioxus.

## Why the name Freyr?

Freyr is the norse god of fertility and prosperity and is known for his beauty. Your Dioxus app should also look good,
even beautiful.

## **First**:
```bash
cargo add freyr
```

This crate provides a set of customizable UI components for use in Dioxus projects.
It allows developers to easily integrate and style components such as navbars and buttons,
with flexible configuration options for color schemes, layouts, and responsiveness.

> ### **Warning:**
> **This component is in the early stage of development. Components may change.
> **_If you are using Dioxus version ```0.5```, please download the ```version 0.1.4```, else use the highest one._**

## Components

- **BasicButton**: A customizable `button` with color options, hover effects, and more.
- **Navbar**: A fully customizable `navigation bar`, custom colors, and responsive layouts.
- **NavbarWithLogo**: A fully customizable `navigation bar`, custom colors, and responsive layouts and with an image logo.
- **NavbarDropdown**: A fully customizable `navigation bar`, custom colors, and responsive layouts and with a dropdown menu included.
- **NavbarDropdownButtons**: The same as the navbar with dropdown menu, but this time you may pass onclick events to the drop items.
- **Dropdown**: A `dropdown` menu with customizable background colors and labels colors, and one that passes onclick events instead of routes.
- **Carousel**: There are three `carousel` at the moment, the CarouselSimple, the CarouselWithTimer, and the CarouselWithNumbers (meaning numbers instead of dots).
- **Tabs**: You may use a standard `Tabs` component with a single text or a personalized one.
- **Accordion**: Make your own or use a more standard one.

## Example of the use of the navbar with a dropdown menu that uses **dioxus-i18n** (_using the version 0.6.0 of Dioxus_):

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

For more information and documentation about freyr, please go to [docs.rs](https://docs.rs/freyr/latest/freyr/).
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
> **This component is in the early stage of development. Right now there are only four components available: two navbar
components, a dropdown and the buttons.**
>
> **_If you are using Dioxus version ```0.5```, please download the ```version 0.1.4```, else use the highest one._**

## Components

- **BasicButton**: A customizable `button` with color options, hover effects, and more.
- **Navbar**: A fully customizable `navigation bar`, custom colors, and responsive layouts.
- **NavbarWithLogo**: A fully customizable `navigation bar`, custom colors, and responsive layouts and with an image logo.
- **NavbarDropdown**: A fully customizable `navigation bar`, custom colors, and responsive layouts and with a dropdown menu included.
- **Dropdown**: A `dropdown` menu with customizable background colors and labels colors.
- **Carousel**: There are three `carousel` at the moment, the CarouselSimple, the CarouselWithTimer, and the CarouselWithNumbers (meaning numbers instead of dots).

## Example Usage (_using the version 0.6.0 of Dioxus_)

 ```rust
 #![allow(non_snake_case)]
use freyr::prelude::*;
use dioxus::prelude::*;

#[component]
fn HomePage() -> Element {
    let navbar_config = NavbarConfig {
        background_color: ColorScheme::Freyr,
        nav_header: "Freyr".to_string(),
        nav_items: vec!["Home".to_string(), "About".to_string(), "Contact".to_string()],
        nav_links: vec!["/".to_string(), "/about".to_string(), "/contact".to_string()],
        nav_item_color: NavItemsColor::Custom("#990000"),
        icon_color: IconColor::Custom("#99cc00"),
    };

    rsx! {
         Navbar { navbar_config }

         div {
             style: "margin-top: 5em;",
             BasicButton { color: ButtonColor::Freyr, label: "Go Home", link: ButtonUrl { url: "/".to_string() } }
         }
     }
}
 ```

For more information and documentation about freyr, please go to [docs.rs](https://docs.rs/freyr/latest/freyr/).
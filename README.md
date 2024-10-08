 # Freyr UI Components Library for Dioxus.

 ## Why the name Freyr?

Freyr is the norse god of fertility and prosperity and is known for his beauty. Your Dioxus app should also look good.

```bash
cargo add freyr
```

 This crate provides a set of customizable UI components for use in Dioxus projects.
 It allows developers to easily integrate and style components such as navbars and buttons,
 with flexible configuration options for color schemes, layouts, and responsiveness.

 > ### **Warning:**
 > **This component is in the early stage of development. Right now there are only two components available: the navbar and the buttons.**

 ## Components

 - **BasicButton**: A customizable button with color options, hover effects, and more.
 - **Navbar**: A fully customizable navigation bar with support for dark/light modes, custom colors, and responsive layouts.

 ## Key Features
 - Full customization of colors, sizes, and layouts using configuration structs and enums.
 - Responsive designs that adapt seamlessly to mobile and desktop layouts.
 - Easy integration into Dioxus projects, with CSS styles scoped to each component.

 ## Example Usage

 ```rust
 #![allow(non_snake_case)]
 use freyr::prelude::*;
 use dioxus::prelude::*;

 #[component]
 fn HomePage() -> Element {
     let navbar_config = NavbarConfig {
         background_color: ColorScheme::Dark, 
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
             BasicButton { color: ButtonColor::Success, label: "Success" }
         }
     }
 }
 ```

 ## Contribution
 Contributions are welcome! Feel free to open an issue or submit a pull request on [GitHub](https://github.com/cbdefontenay/freyr).
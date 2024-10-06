/// Configuration options for the **`Navbar`** component, including color schemes and menu settings.

/// Represents different background color schemes for the navbar.
#[derive(PartialEq, Clone)]
pub enum ColorScheme {
    Dark,
    Light,
    Custom(&'static str),
}

impl ColorScheme {
    /// Returns the CSS class or custom color for the background.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            ColorScheme::Dark => "#222",
            ColorScheme::Light => "#fff",
            ColorScheme::Custom(color) => color,
        }
    }
}

/// Defines different color options for navigation items.
#[derive(PartialEq, Clone)]
pub enum NavItemsColor {
    Dark,
    Light,
    Custom(&'static str),
}

impl NavItemsColor {
    /// Returns the CSS class or custom color for the navigation items.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            NavItemsColor::Dark => "#000",
            NavItemsColor::Light => "#fff",
            NavItemsColor::Custom(color) => color,
        }
    }
}

/// Specifies color options for the menu icons (hamburger and cross).
#[derive(PartialEq, Clone)]
pub enum IconColor {
    White,
    Black,
    Custom(&'static str),
}

impl IconColor {
    /// Returns the CSS class or custom color for the icons.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            IconColor::White => "#fff",
            IconColor::Black => "#000",
            IconColor::Custom(color) => color,
        }
    }
}

/// Configuration struct for the **`Navbar`** component.
#[derive(PartialEq, Clone)]
pub struct NavbarConfig {
    pub background_color: ColorScheme,
    pub nav_header: String,
    pub nav_items: Vec<String>,
    pub nav_links: Vec<String>,
    pub nav_item_color: NavItemsColor,
    pub icon_color: IconColor,
}

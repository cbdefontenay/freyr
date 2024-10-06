/// Represents different background color schemes for the dropdown.
#[derive(PartialEq, Clone)]
pub enum DropdownColorScheme {
    Dark,
    Light,
    Custom(&'static str),
}

impl DropdownColorScheme {
    /// Returns the CSS class or custom color for the background.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            DropdownColorScheme::Dark => "#222",
            DropdownColorScheme::Light => "#fff",
            DropdownColorScheme::Custom(color) => color,
        }
    }
}

/// Defines different color options for dropdown labels.
#[derive(PartialEq, Clone)]
pub enum DropdownLabelsColor {
    Dark,
    Light,
    Custom(&'static str),
}

impl DropdownLabelsColor {
    /// Returns the CSS class or custom color for the dropdown labels.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            DropdownLabelsColor::Dark => "#000",
            DropdownLabelsColor::Light => "#fff",
            DropdownLabelsColor::Custom(color) => color,
        }
    }
}

/// `DropdownItem` represents an individual item in the dropdown menu,
/// containing a label and a URL for navigation.
#[derive(PartialEq, Clone)]
pub struct DropdownItem {
    pub label: String,
    pub url: String,
}

/// `DropdownConfig` holds the configuration for the dropdown menu, including
/// the list of dropdown items, background color, and text color.
#[derive(PartialEq, Clone)]
pub struct DropdownConfig {
    pub title: String,
    pub label: Vec<DropdownItem>,
    pub background_color: DropdownColorScheme,
    pub labels_color: DropdownLabelsColor,
}

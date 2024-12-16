/// Represents different background color schemes for the dropdown.
#[derive(PartialEq, Clone)]
pub enum DropdownColorScheme {
    Freyr,
    Dark,
    Light,
    Custom(&'static str),
}

impl DropdownColorScheme {
    /// Returns the CSS class or custom color for the background.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            DropdownColorScheme::Freyr => "#3795BD",
            DropdownColorScheme::Dark => "#1E201E",
            DropdownColorScheme::Light => "#F7F7F8",
            DropdownColorScheme::Custom(color) => color,
        }
    }
}

/// Defines different color options for the title of the dropdown.
#[derive(PartialEq, Clone)]
pub enum DropdownTitleColor {
    Freyr,
    Dark,
    Light,
    Custom(&'static str),
}

impl DropdownTitleColor {
    /// Returns the CSS class or custom color for the dropdown title.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            DropdownTitleColor::Freyr => "#3795BD",
            DropdownTitleColor::Dark => "#1E201E",
            DropdownTitleColor::Light => "#F7F7F8",
            DropdownTitleColor::Custom(color) => color,
        }
    }
}

/// Defines different color options for dropdown labels.
#[derive(PartialEq, Clone)]
pub enum DropdownLabelsColor {
    Freyr,
    Dark,
    Light,
    Custom(&'static str),
}

impl DropdownLabelsColor {
    /// Returns the CSS class or custom color for the dropdown labels.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            DropdownLabelsColor::Freyr => "#3795BD",
            DropdownLabelsColor::Dark => "#1E201E",
            DropdownLabelsColor::Light => "#F7F7F8",
            DropdownLabelsColor::Custom(color) => color,
        }
    }
}
/// Defines different color options for dropdown background hover color.
#[derive(PartialEq, Clone)]
pub enum DropdownHoverColor {
    Freyr,
    Dark,
    Light,
    Custom(&'static str),
}

impl DropdownHoverColor {
    /// Returns the CSS class or custom color for the dropdown background hover color.
    pub fn as_css_class(&self) -> &'static str {
        match self {
            DropdownHoverColor::Freyr => "#3795BD",
            DropdownHoverColor::Dark => "#1E201E",
            DropdownHoverColor::Light => "#F7F7F8",
            DropdownHoverColor::Custom(color) => color,
        }
    }
}

/// `DropdownItem` represents an individual item in the dropdown menu,
/// containing a label and a URL for navigation.
#[derive(PartialEq, Clone)]
pub struct DropdownItem {
    pub label: String,
    pub url: Option<String>,
}

impl DropdownItem {
    /// Creates a new `DropdownItem` with a label and an optional URL.
    /// If the URL is not provided, it defaults to `None`.
    pub fn new(label: impl Into<String>, url: Option<String>) -> Self {
        Self {
            label: label.into(),
            url,
        }
    }

    /// Creates a `DropdownItem` with only a label and no URL.
    pub fn without_url(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            url: None,
        }
    }
}


/// `DropdownConfig` holds the configuration for the dropdown menu, including
/// the list of dropdown items, background color, and text color.
#[derive(PartialEq, Clone)]
pub struct DropdownConfig {
    pub title: String,
    pub label: Vec<DropdownItem>,
    pub background_color: DropdownColorScheme,
    pub title_color: DropdownTitleColor,
    pub labels_color: DropdownLabelsColor,
    pub hover_color: DropdownHoverColor,
}

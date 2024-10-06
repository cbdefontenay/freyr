pub const DROPDOWN_STYLES: &str = r#"
    .dropdown {
        position: relative;
        display: inline-block;
        font-family: Arial, sans-serif;
    }

    .dropdown-toggle {
        background-color: #3498db;
        color: white;
        padding: 12px 20px;
        font-size: 16px;
        border: none;
        cursor: pointer;
        border-radius: 5px;
        transition: background-color 0.3s ease;
    }

    .dropdown-content {
        position: absolute;
        background-color: #ffffff;
        min-width: 180px;
        box-shadow: 0px 10px 20px rgba(0, 0, 0, 0.1);
        z-index: 1;
        border-radius: 8px;
        padding: 8px 0;

    .link {
        color: #333;
        padding: 12px 20px;
        text-decoration: none;
        display: block;
        transition: background-color 0.3s ease, color 0.3s ease;
        font-size: 15px;
    }

    .link:hover {
        background-color: #f2f2f2;
        color: #3498db;
    }

    .dropdown-toggle:hover {
        background-color: #2980b9;
    }
"#;

pub const DROPDOWN_STYLES: &str = r#"
    .dropdown {
        position: relative;
        display: inline-block;
        font-family: Arial, sans-serif;
    }

.dropdown-toggle {
    color: white;
    padding: 12px 14px;
    font-size: 16px;
    border: none;
    cursor: pointer;
    border-radius: 5px;
    transition: background-color 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.dropdown-toggle svg {
    margin-left: 10px;
    width: 16px;
    height: 16px;
}

    .dropdown-content {
        position: absolute;
        min-width: 180px;
        box-shadow: 0px 10px 20px rgba(0, 0, 0, 0.1);
        z-index: 1;
        border-radius: 8px;
        padding: 8px 0;

    .link {
        padding: 12px 20px;
        text-decoration: none;
        display: block;
        transition: background-color 0.3s ease, color 0.3s ease;
        font-size: 15px;

          &:hover {
             background: var(--custom_color);
            }
    }

    .dropdown-toggle:hover {
        background-color: #2980b9;
    }

      @media (max-width: 768px) {
        .dropdown {
            width: 100%;
        }

        .dropdown-toggle {
            width: 100%;
            padding: 12px;
            font-size: 18px;
        }

        .dropdown-content {
            position: static;
            width: 100%;
            box-shadow: none;
            border-radius: 0;
            padding: 0;
            background-color: #1E201E;
        }

        .link {
            padding: 16px 20px;
            font-size: 16px;
            text-align: center;
        }

        .dropdown-toggle svg {
            margin-left: auto;
        }
    }
"#;

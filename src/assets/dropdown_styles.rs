pub const DROPDOWN_STYLES: &str = r#"
    .dropdown {
        position: relative;
        display: inline-block;
    }

.dropdown-toggle {
    color: white;
    padding: 7px 9px;
    border: none;
    cursor: pointer;
    border-radius: 5px;
    transition: background-color 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.dropdown-toggle svg {
    width: 16px;
    height: 16px;
    margin-left: 10px;
}

    .dropdown-content {
        position: absolute;
        box-shadow: 0px 10px 20px rgba(0, 0, 0, 0.1);
        z-index: 1;
        border-radius: 8px;
        padding: 8px 0;
        width: max-content;
    }

    .link {
        padding: 10px 13px;
        text-decoration: none;
        display: block;
        transition: background-color 0.3s ease, color 0.3s ease;

          &:hover {
             background: var(--custom_color);
            }
    }

    .button-config {
        padding: 10px 13px;
        display: block;
        transition: background-color 0.3s ease, color 0.3s ease;

          &:hover {
             background: var(--custom_color);
            }
    }

    .dropdown-toggle:hover {
        background-color: #2980b9;
    }

      @media (max-width: 768px) {
        .dropdown-toggle {
            padding: 8px;
        }

        .dropdown-content {
            border-radius: 2px;
            padding: 0;
        }

        .link {
            padding: 10px 12px;
        }

        .button-config {
            padding: 10px 12px;
        }

        .dropdown-toggle svg {
            margin-left: 10px;
        }
    }
"#;

pub const BUTTON_STYLES: &str = r#"
    button {
        padding: 10px 20px;
        font-size: 16px;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        transition: background-color 0.3s ease;
        max-width: 300px;
        margin: 0 auto; /* Center the button horizontally */
    }

    .btn-default {
        background-color: #999999;
        color: white;
    }

    .btn-primary {
        background-color: #007bff;
        color: white;
    }

    .btn-success {
        background-color: #28a745;
        color: white;
    }

    .btn-danger {
        background-color: #dc3545;
        color: white;
    }

    .btn-black {
        background-color: black;
        color: white;
    }

    .btn-transparent {
        background-color: transparent;
        color: black;
        border: 1px solid black;
    }

    button:hover {
        filter: brightness(90%);
    }

    /* Media Queries for Responsive Design */
    @media (max-width: 600px) {
        button {
            padding: 12px; /* Increase padding on smaller screens */
            font-size: 14px; /* Slightly smaller font size */
        }
    }

    @media (min-width: 601px) and (max-width: 768px) {
        button {
            padding: 10px 15px; /* Adjust padding for medium screens */
            font-size: 15px; /* Adjust font size for medium screens */
        }
    }

    @media (min-width: 769px) {
        button {
            width: auto; /* Auto width on larger screens */
            padding: 10px 20px; /* Keep default padding */
            font-size: 16px; /* Keep default font size */
        }
    }
"#;

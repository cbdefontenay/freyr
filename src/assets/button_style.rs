pub const BUTTON_STYLES: &str = r#"
    button {
        padding: 10px 20px;
        font-size: 16px;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        transition: background-color 0.3s ease;
        max-width: 300px;
    }

    a {
        text-decoration: none;
    }

    .btn-freyr {
        background-color: #3795BD;
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
            padding: 12px;
            font-size: 14px;
        }
    }

    @media (min-width: 601px) and (max-width: 768px) {
        button {
            padding: 10px 15px;
            font-size: 15px;
        }
    }

    @media (min-width: 769px) {
        button {
            width: auto;
            padding: 10px 20px;
            font-size: 16px;
        }
    }
"#;

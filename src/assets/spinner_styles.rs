pub const SPINNER_STYLES: &str = r#"
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
        .custom-spinner {
            animation: spin 1s linear infinite;
        }
    "#;
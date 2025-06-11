pub const CARD_STYLES: &str = r#"
    .card-wrapper, .card-shadow {
        height: auto;
        border-radius: 1rem;
        transition: box-shadow 0.3s ease;
        margin: 0;
        overflow: hidden;
        background-color: inherit;
    }

    .card-shadow {
        box-shadow:
            0 1px 3px rgba(0, 0, 0, 0.12),
            0 4px 6px rgba(0, 0, 0, 0.16);
    }
   
    @media (max-width: 640px) {
        .card-wrapper, .card-shadow {
            margin: 0.5rem;
            border-radius: 0.75rem;
        }
    }
"#;
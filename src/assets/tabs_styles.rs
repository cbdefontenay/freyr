pub const TABS_STYLES: &str = r#"
#tabs {
    display: flex;
    flex-direction: column;
    align-items: center;
    min-height: 100%;
    padding: 10px;
}

.tabs-navigation {
    display: flex;
    justify-content: center;
    gap: 1rem;
    border-bottom: 1px solid #d1d5db;
    padding-bottom: 4px;
    margin-bottom: 6px;
    flex-wrap: nowrap;
}

.tab-item {
    cursor: pointer;
    color: #4b5563;
    font-weight: 500;
    padding-bottom: 2px;
    border-bottom: 2px solid transparent;
    transition: all 0.3s;
}

.tab-item:hover {
    color: #3b82f6;
    border-color: #3b82f6;
}

.city-content {
    border-radius: 8px;
    padding: 8px;
    width: 100%;
    text-align: center;
    animation: fade-in 0.5s;
}

.city-title {
    font-size: 1.5rem;
    font-weight: bold;
    color: #1f2937;
}

.city-text {
    color: #4b5563;
    margin-top: 1rem;
}

.tabs-freyr {
    cursor: pointer;
    color: #3795BD;
    font-weight: 500;
    padding-bottom: 2px;
    border-bottom: 2px solid transparent;
    transition: all 0.3s;
}

.tabs-freyr:hover {
    color: #2E7FA4;
    border-color: #2E7FA4;
}

.tabs-black {
    cursor: pointer;
    font-weight: 500;
    padding-bottom: 2px;
    border-bottom: 2px solid transparent;
    transition: all 0.3s;
    color: #050505;
}

.tabs-black:hover {
    color: #211f1f;
    border-color: #211f1f;
}

.tabs-light {
    cursor: pointer;
    font-weight: 500;
    padding-bottom: 2px;
    border-bottom: 2px solid transparent;
    transition: all 0.3s;
    color: #fafafa;
}

.tabs-light:hover {
    color: #d8e3e3;
    border-color: #d8e3e3;
}

@keyframes fade-in {
    from {
        opacity: 0;
    }
    to {
        opacity: 1;
    }
}

@media (max-width: 640px) {
    .tabs-navigation {
        flex-direction: row;
        gap: 0.5rem;
    }

    .city-content {
        padding: 6px;
    }

    .city-title {
        font-size: 1.25rem;
    }

    .city-text {
        font-size: 0.875rem;
    }
}
"#;

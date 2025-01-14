pub const TABS_STYLES: &str = r#"
#tabs {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.tabs-navigation {
    display: flex;
    justify-content: center;
    gap: 1.5rem;
    border-bottom: 1px solid #c8ccd2;
    padding-bottom: 4px;
    margin-bottom: 6px;
    flex-wrap: nowrap;
}

.tab-content {
    // max-width: 60%;
}

.tab-item {
    cursor: pointer;
    color: #4b5563;
    font-weight: 500;
    padding-bottom: 2px;
    border-bottom: 2px solid transparent;
    transition: all 0.3s;
}

.tab-item.active-tab {
    border-bottom: 2px solid;
    // color: #3b82f6;
}

.tab-item:hover {
    border-bottom-color: #3b82f6;
    color: #3b82f6;
}

.tabs-freyr {
    cursor: pointer;
    color: #3795BD;
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

@media (max-width: 768px) {
    .tabs-navigation {
        flex-direction: row;
        gap: 1rem;
    }

    .tab-content {
        max-width: 90%;
    }

}
"#;
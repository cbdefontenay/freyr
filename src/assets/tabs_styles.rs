pub const TABS_STYLES: &str = r#"
/* Tabs container */
#tabs {
    display: flex;
    flex-direction: column;
    align-items: center;
    min-height: 100%;
    padding: 10px;
}

/* Tab navigation */
.tabs-navigation {
    display: flex; /* Use flexbox for row layout */
    justify-content: center;
    gap: 1rem; /* Space between tabs */
    border-bottom: 1px solid #d1d5db; /* Gray border */
    padding-bottom: 4px;
    margin-bottom: 6px;
    flex-wrap: nowrap; /* Prevent wrapping to new lines */
}

/* Tab items */
.tab-item {
    cursor: pointer;
    color: #4b5563; /* Gray text */
    font-weight: 500; /* Medium font weight */
    padding-bottom: 2px;
    border-bottom: 2px solid transparent;
    transition: all 0.3s;
}

.tab-item:hover {
    color: #3b82f6; /* Blue hover text */
    border-color: #3b82f6; /* Blue hover border */
}

/* City content */
.city-content {
    border-radius: 8px;
    padding: 8px;
    width: 100%;
    text-align: center;
    animation: fade-in 0.5s;
}

/* Title styles */
.city-title {
    font-size: 1.5rem; /* Equivalent to text-2xl */
    font-weight: bold;
    color: #1f2937; /* Dark gray */
}

/* Text styles */
.city-text {
    color: #4b5563; /* Gray */
    margin-top: 1rem;
}

/* Fade-in animation */
@keyframes fade-in {
    from {
        opacity: 0;
    }
    to {
        opacity: 1;
    }
}

/* Mobile responsiveness */
@media (max-width: 640px) {
    .tabs-navigation {
        flex-direction: column; /* Stack items in a column for smaller screens */
        gap: 0.5rem;
    }

    .city-content {
        padding: 6px;
    }

    .city-title {
        font-size: 1.25rem; /* Slightly smaller for mobile */
    }

    .city-text {
        font-size: 0.875rem; /* Slightly smaller for mobile */
    }
}

"#;

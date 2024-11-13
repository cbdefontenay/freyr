pub const CAROUSEL_STYLES: &str = r#"
.carousel-container {
    position: relative;
    overflow: hidden;
}

.carousel {
    display: flex;
    transition: transform 0.6s ease-in-out;
}

.carousel-item {
    min-width: 100%;
    box-sizing: border-box;
}

.carousel-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: center;
}

.carousel-dots {
    display: flex;
    justify-content: center;
    gap: 8px;
    position: absolute;
    bottom: 10px;
    width: 100%;
}

.carousel-dot {
    width: 10px;
    height: 10px;
    background-color: #ccc;
    border-radius: 50%;
    cursor: pointer;
}

.carousel-dot.active {
    background-color: #333;
}

@media (max-width: 768px) {

    .carousel-image {
        object-position: center center;
    }

    .carousel-dots {
        bottom: 8px;
    }

    .carousel-dot {
        width: 8px;
        height: 8px;
    }
}

@media (max-width: 480px) {

    .carousel-dots {
        bottom: 6px;
    }

    .carousel-dot {
        width: 6px;
        height: 6px;
    }
}

"#;
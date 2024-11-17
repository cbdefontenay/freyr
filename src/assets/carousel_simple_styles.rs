pub const CAROUSEL_STYLES: &str = r#"
.carousel-container {
    width: 100%;
    height: auto;
    max-width: 1200px;
    margin: 0 auto;
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
    background-color: #fff;
    border-radius: 50%;
    cursor: pointer;
}

.carousel-dot.active {
    background-color: #333;
}

@media (min-width: 1025px) {
    .carousel-container {
        width: 50%;
    }
}


@media (min-width: 769px) and (max-width: 1024px) {
    .carousel-container {
        width: 80%;
        margin: 0 auto;
    }
}

@media (max-width: 768px) {
    .carousel-image {
        object-position: center center;
    }

    .carousel-container {
        width: 80%;
        margin: 0 auto;
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

    .carousel-container {
        width: 100%;
    }

    .carousel-dot {
        width: 6px;
        height: 6px;
    }
}
"#;

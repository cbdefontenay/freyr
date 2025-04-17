pub const CAROUSEL_STYLES: &str = r#"
.carousel-container {
    margin: 0 auto;
    position: relative;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    align-items: center;
    height: fit-content;
}

.carousel-container-default {
    width: 40%;
    height: auto;
    max-width: 1200px;
}

.carousel-simple-dots {
    display: flex;
    justify-content: center;
    gap: 8px;
    position: absolute;
    bottom: 10px;
    left: 0;
    right: 0;
    z-index: 10;
    pointer-events: none;
}

.carousel-simple-dot {
    width: 10px;
    height: 10px;
    background-color: #fff;
    border-radius: 50%;
    cursor: pointer;
    pointer-events: auto;
    transition: background-color 0.3s ease;
}

.carousel-simple-dot.active {
    background-color: #333;
}

.carousel-content-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
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
    display: block;
}

.carousel-dots {
    display: flex;
    justify-content: center;
    gap: 8px;
    position: absolute;
    bottom: 10px;
    left: 0;
    right: 0;
    z-index: 10;
    pointer-events: none;
}

.carousel-dot {
    width: 10px;
    height: 10px;
    background-color: #fff;
    border-radius: 50%;
    cursor: pointer;
    pointer-events: auto;
}

.carousel-dot.active {
    background-color: #333;
}

.carousel-numbers {
    display: flex;
    justify-content: center;
    gap: 8px;
    position: absolute;
    bottom: 0;
    width: 100%;
    font-size: 1rem;
   padding: 8px 0;
}

.carousel-number {
    padding: 4px 8px;
    background-color: #ddd;
    border-radius: 4px;
    cursor: pointer;
    text-align: center;
}

.carousel-number.active {
    background-color: #333;
    color: #fff;
}

@media (max-width: 1024px) {
    .carousel-container-default {
        width: 60%;
    }

    .carousel-dots {
        bottom: 8px;
    }

    .carousel-dot {
        width: 10px;
        height: 10px;
    }

    .carousel-numbers {
        font-size: 0.10rem;
        gap: 6px;
    }
}

@media (max-width: 768px) {
    .carousel-image {
        object-position: center center;
    }

    .carousel-container-default {
        width: 70%;
        margin: 0 auto;
    }

    .carousel-dots {
        bottom: 6px;
    }

    .carousel-dot {
        width: 10px;
        height: 10px;
    }

    .carousel-numbers {
        font-size: 0.8rem;
        gap: 5px;
    }

    .carousel-number {
        padding: 2px 4px;
    }
}

@media (max-width: 480px) {
    .carousel-container-default {
        width: 90%;
        height: 50%;
    }
   
    .carousel-dot {
        width: 10px;
        height: 10px;
    }

    .carousel-numbers {
        font-size: 0.6rem;
        gap: 4px;
    }
}
"#;

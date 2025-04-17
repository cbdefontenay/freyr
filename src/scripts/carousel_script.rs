pub const CAROUSEL_SCRIPT: &str = r#"
    if (!window.carouselState) {
        window.carouselState = {
            currentIndex: {initial_index},
            interval: null
        };
    }

    const itemsLength = {items_len};
    const items = {items_data};
    const timerMs = {timer_ms};
    const imageElement = document.getElementById('carousel-image');
    const dots = document.querySelectorAll('.carousel-dot');

    function updateCarousel(newIndex, updateSignal = true) {
        window.carouselState.currentIndex = newIndex;
        imageElement.src = items[newIndex].image_url;
        imageElement.alt = items[newIndex].image_alt;

        dots.forEach((dot, i) => {
            dot.classList.toggle('active', i === newIndex);
        });

        if (updateSignal && window.setCarouselIndex) {
            window.setCarouselIndex(newIndex);
        }
    }
    
    function startCarouselInterval() {
        if (window.carouselState.interval) {
            clearInterval(window.carouselState.interval);
        }

        // Create and store new interval
        window.carouselState.interval = setInterval(() => {
            const nextIndex = (window.carouselState.currentIndex + 1) % itemsLength;
            updateCarousel(nextIndex);
        }, timerMs);
    }

    // Manual dot click handler
    window.setCarouselIndex = (newIndex) => {
        updateCarousel(newIndex, false);
        startCarouselInterval(); // Reset interval

    };
    updateCarousel(window.carouselState.currentIndex, false);
    startCarouselInterval();
"#;
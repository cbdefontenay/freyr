pub const CAROUSEL_SCRIPT: &str = r#"
    let index = 0;
    const itemsLength = {items_len};
    const items = {items_data};

    const currentIndexElement = document.getElementById('carousel-image');
    const dots = document.querySelectorAll('.carousel-dot');

    setInterval(() => {
        index = (index + 1) % itemsLength;
        currentIndexElement.src = items[index].image_url;
        currentIndexElement.alt = items[index].image_alt;

        dots.forEach(dot => dot.classList.remove('active'));
        dots[index].classList.add('active');
    }, {timer_ms});
"#;
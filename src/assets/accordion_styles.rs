pub const ACCORDION_STYLES: &str = r#"
    .accordion {
    width: 100%;
    max-width: 600px;
    margin: 20px auto;
}

.accordion-wrapper {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    padding: 10px;
    background-color: #6081b9;
    border-radius: 5px 5px 0 0;
    cursor: pointer;
    position: relative;
}

.title-wrapper {
    flex-grow: 1;
}

.title {
    color: #e5e7eb;
    font-size: 18px;
}

.accordion-button {
    display: flex;
    align-items: center;
    background: none;
    border: none;
    cursor: pointer;
}

.icon-wrapper {
    display: flex;
    justify-content: center;
    align-items: center;
}

.accordion-content {
    display: none;
    background-color: #6081b9;
    padding: 10px;
    color: #fff;
    transition: max-height 0.4s ease-out, padding 0.4s ease-out;
    max-height: 0;
    overflow: hidden;
}

.accordion-content.open {
    display: block;
    max-height: 200px;
    padding: 10px;
    text-align: justify;
}

.icon {
    width: 24px;
    height: 24px;
    cursor: pointer;
}
"#;
pub const ACCORDION_STYLES: &str = r#"
.accordion {
    width: 100%;
    max-width: 700px;
    // margin: 20px auto;
}

.accordion-wrapper {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    padding: 10px;
    cursor: pointer;
    position: relative;
}

.accordion-wrapper-default {
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

.title-wrapper-default {
    flex-grow: 1;
    font-size: 18px;
    color: #fff;
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

.accordion-content-default {
    display: none;
    background-color: #a2c3db;
    padding: 10px;
    color: #fff;
    transition: max-height 0.4s ease-out, padding 0.4s ease-out;
    max-height: 100%;
    overflow: hidden;
}

.accordion-content-default.open-default {
    display: block;
    padding: 10px;
    text-align: justify;
}

.accordion-content {
    display: none;
    padding: 10px;
    transition: max-height 0.4s ease-out, padding 0.4s ease-out;
    overflow: hidden;
}

.accordion-content.open {
    display: block;
    padding: 10px;
}

.icon {
    width: 24px;
    height: 24px;
    cursor: pointer;
}
"#;
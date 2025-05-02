pub const ACCORDION_STYLES: &str = r#"
.accordion {
    width: 100%;
    max-width: 700px;
}

.accordion-wrapper {
    display: flex;
    justify-content: space-between;
    align-items: center;
    // width: 100%;
    padding: 10px;
    cursor: pointer;
    position: relative;
}

.accordion-wrapper-default {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 28rem;
    padding: 10px;
    background-color: #6081b9;
    border-radius: 5px 5px 0 0;
    cursor: pointer;
    position: relative;
}

.title-wrapper-default {
    flex-grow: 1;
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

@media (max-width: 768px) {
    .accordion-wrapper-default {
        width: 19rem;
    }
"#;

pub const ACCORDION_NEW_STYLES: &str = r#"
    .accordion-left {
        background-color: var(--header-background-color, #f0f9ff);
        width: var(--accordion-width, 700px);
        margin: 0.5rem auto;
        border-radius: var(--border-radius, 10px);
        overflow: hidden;
        box-shadow: var(--box-shadow, none);
    }
   
    .accordion-left-header {
        display: flex;
        align-items: center;
        padding: 1rem;
        cursor: pointer;
        transition: all 0.3s ease;
    }
    
    .accordion-left-icon {
        margin-right: 1rem;
        transition: transform 0.3s ease;
    }
   
    .accordion-left-icon.open {
        transform: rotate(90deg);
    }
   
    .accordion-left-title {
        flex-grow: 1;
        color: var(--title-color, #000000);
        font-size: var(--title-font-size, 1.25rem);
        font-weight: var(--title-font-weight, 600);
        margin: 0;
    }
   
    .accordion-left-content {
        max-height: 0;
        overflow: hidden;
        padding: 0 var(--content-padding, 1rem);
        transition: max-height 0.3s ease, padding 0.3s ease;
    }
   
    .accordion-left-content.open {
        max-height: var(--accordion-open-max-height, 500px);
        padding: var(--content-padding, 1rem);
    }
   
    @media (max-width: 1000px) {
        .accordion-left {
            width: 100%;
        }
    }
    @media (max-width: 758px) {
        .accordion-left {
            width: 100%;
        }
   
        .accordion-left-title {
            font-size: 1rem;
        }
    }
"#;
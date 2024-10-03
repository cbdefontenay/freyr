pub const NAVBAR_STYLES: &str = r#"
    html, body {
        margin: 0;
        padding: 0;
    }

    .navbar {
        margin: 0;
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 15px 25px;
        position: relative;
    }

    .nav-div {
        display: flex;
        align-items: center;
    }

    .nav-header-wrapper {
        font-size: 1.5rem;
        font-weight: bold;
        color: #fff;
    }

    .hamburger {
        display: none;
    }

    .menu {
        display: flex;
        position: static;
        flex-direction: row;
        box-shadow: none;
        padding: 0;
        align-items: center;
        height: 40px;
    }

    .menu-item {
        padding: 10px 15px;
        font-size: 1.2rem;
        text-decoration: none;
        transition: color 0.3s ease, background-color 0.3s ease;
    }

    .menu-item:hover {
        color: #ccc;
        background-color: transparent;
    }

    @media (max-width: 600px) {
        .hamburger {
            cursor: pointer;
            display: flex;
            flex-direction: column;
            justify-content: space-around;
            height: 24px;
            width: 30px;
            background: none;
            border: none;
            outline: none;
            padding: 0;
            position: absolute;
            right: 15px;
        }

        .menu {
            display: none;
            flex-direction: row;
            justify-content: center;
            align-items: center;
            position: absolute;
            left: 0;
            top: 100%;
            width: 100%;
            height: 100vh;
            background-color: var(--background-color);
            padding: 20px 0;
            z-index: 999;
        }

        .menu.open {
            display: flex;
            animation: slideDown 0.3s ease-in-out;
        }

        .menu-item {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 15px 20px;
            font-size: 1.2rem;
            text-decoration: none;
            transition: color 0.3s ease, background-color 0.3s ease;
            color: var(--nav-item-color); /* Dynamic nav item color */
        }

        .menu-item:hover {
            color: #ccc;
            background-color: transparent;
        }
    }

    @keyframes slideDown {
        from {
            transform: translateY(-20%);
            opacity: 0;
        }
        to {
            transform: translateY(0);
            opacity: 1;
        }
    }
"#;

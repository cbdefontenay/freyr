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
        z-index: 99;
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
            z-index: 20; /* Ensure hamburger/cross is on top */
        }

        .menu {
            display: none;
            flex-direction: column; /* Change to column for full-screen nav items */
            justify-content: center;
            align-items: center;
            position: fixed; /* Fix position to cover the whole screen */
            left: 0;
            top: 0;
            width: 100%;
            height: 100vh; /* Full screen */
            background-color: var(--background-color);
            padding: 20px 0;
            z-index: 15; /* Menu appears over navbar */
        }

        .menu.open {
            display: flex;
            animation: slideDown 0.4s ease-in-out;
        }

        .menu-item {
            display: block;
            font-size: 1.5rem;
            text-decoration: none;
            transition: color 0.3s ease, background-color 0.3s ease;
            color: var(--nav-item-color); /* Dynamic nav item color */
            padding: 20px; /* Increased padding for full-screen items */
        }

        .menu-item:hover {
            color: #ccc;
            background-color: transparent;
        }
    }

    @keyframes slideDown {
        from {
            transform: translateY(-10%);
            opacity: 0;
        }
        to {
            transform: translateY(0);
            opacity: 1;
        }
    }
"#;
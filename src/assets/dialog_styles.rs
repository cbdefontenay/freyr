pub const DIALOG_STYLES: &str = r#"
.dialog-button-default {
    background-color: #2563eb; /* blue-600 */
    color: white;
    font-weight: bold;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    transition: background-color 0.3s;
}
.dialog-button-default:hover {
    background-color: #1d4ed8; /* blue-700 */
}

.dialog-overlay {
    position: fixed;
    inset: 0;
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
}

.dialog-wrap-default {
    background-color: white;
    border-radius: 0.5rem;
    box-shadow: 0 10px 15px -3px rgba(0,0,0,0.1), 0 4px 6px -2px rgba(0,0,0,0.05);
    width: 100%;
    max-width: 28rem; /* max-w-md */
    padding: 1.5rem;
    position: relative;
}

.dialog-close-cross {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    padding: 0.5rem;
    border-radius: 9999px;
    transition: background-color 0.3s;
    cursor: pointer;
}
.dialog-close-cross:hover {
    background-color: #e5e7eb; /* hover:bg-gray-200 */
}

.dialog-cross-svg {
    width: 1.5rem; /* w-6 */
    height: 1.5rem; /* h-6 */
    color: #6b7280; /* text-gray-500 */
    transition: color 0.3s;
}
.dialog-cross-svg:hover {
    color: #374151; /* hover:text-gray-700 */
}

.dialog-default-text {
    color: #374151; /* text-gray-700 */
    margin-bottom: 1rem;
}

.dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    margin-top: 1rem;
    cursor: pointer;
}

.dialog-close-button-default {
    background-color: #dc2626; /* red-600 */
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    transition: background-color 0.3s;
}
.dialog-close-button-default:hover {
    background-color: #b91c1c; /* red-700 */
}
"#;
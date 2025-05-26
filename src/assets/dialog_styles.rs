pub const DIALOG_STYLES: &str = r#"
.dialog-overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 999;
    padding: 0 10px;
}

.dialog-wrap {
    background-color: white;
    border-radius: 0.5rem;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
    max-width: 28rem;
    width: 100%;
    padding: 1.5rem;
    position: relative;
}

.dialog-close-icon {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: none;
    border: none;
    cursor: pointer;
}

.dialog-cross-svg {
    width: 1.5rem;
    height: 1.5rem;
    color: #4B5563; /* Tailwind gray-600 */
    transition: color 0.2s;
}
.dialog-cross-svg:hover {
    color: #1F2937; /* Tailwind gray-800 */
}

.dialog-body {
    margin-bottom: 1.5rem;
}

.dialog-placeholder {
    color: #6B7280; /* Tailwind gray-500 */
    text-align: center;
}

.dialog-actions {
    display: flex;
    justify-content: flex-end;
}

.dialog-close-button {
    background-color: #4F46E5; /* Tailwind indigo-600 */
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    transition: background-color 0.2s;
}
.dialog-close-button:hover {
    background-color: #4338CA; /* Tailwind indigo-700 */
}
"#;
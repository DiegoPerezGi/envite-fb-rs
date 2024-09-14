
export function copyToClipboardFallback(textarea) {
    textarea.select();
    document.execCommand('copy');
}
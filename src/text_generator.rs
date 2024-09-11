use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[wasm_bindgen(inline_js = "
export function copyToClipboardFallback(textarea) {
    textarea.select();
    document.execCommand('copy');
}")]
extern "C" {
    fn copyToClipboardFallback(textarea: &HtmlTextAreaElement);
}

#[function_component]
pub fn TextGenerator() -> Html {
    let textarea_ref = use_node_ref();

    // Callback para copiar al portapapeles usando el fallback execCommand
    let onclick_write_text = {
        let textarea_ref = textarea_ref.clone();
        Callback::from(move |_| {
            if let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() {
                // Intenta copiar usando el fallback de execCommand
                copyToClipboardFallback(&textarea);
            }
        })
    };

    html! {
        <div>
            <textarea ref={textarea_ref} rows=10 cols=60 placeholder="Escribe algo aquí..." />
            <button onclick={onclick_write_text}>{ "Copiar al portapapeles" }</button>
        </div>
    }
}

use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::get_participants::get_participants;

#[wasm_bindgen(inline_js = "
export function copyToClipboardFallback(textarea) {
    textarea.select();
    document.execCommand('copy');
}")]
extern "C" {
    fn copyToClipboardFallback(textarea: &HtmlTextAreaElement);
}

fn filter_participants(div: String) -> String {
    let names: Vec<String> = get_participants()
        .iter()
        .filter_map(|participant| {
            if participant.div == div {
                Some(participant.name.clone())
            } else {
                None
            }
        })
        .collect();
    names.join(", \n")
}

#[function_component]
pub fn TextGenerator() -> Html {
    let textarea_ref = use_node_ref();
    let text = use_state(|| "".to_string());

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

    let onchange_update_text = {
        let text = text.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            text.set(input.value());
        })
    };
    let onclick_generate_text = {
        let text = text.clone();
        Callback::from(move |_| text.set(filter_participants("A".to_string())))
    };

    html! {
        <div>
            <textarea ref={textarea_ref} value={(*text).clone()} oninput={onchange_update_text} rows=10 cols=60 placeholder="Escribe algo aquí...">
            </textarea>
            <button
                class={classes!("bg-green-500", "hover:bg-green-700", "rounded", "px-4", "py-1", "mx-1")}
                onclick={onclick_write_text}
            >
            { "Copiar al portapapeles" }
            </button>
            <button
                class={classes!("bg-green-500", "hover:bg-green-700", "rounded", "px-4", "py-1", "mx-1")} onclick={onclick_generate_text}>{"Generar texto"}</button>
        </div>
    }
}

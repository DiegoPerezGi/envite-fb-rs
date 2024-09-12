use yew::{platform::spawn_local, prelude::*};

use reqwasm::http::Request;
use serde_json::Value;

use gloo_console::log;

mod get_participants;
mod participants_list;
mod text_generator;

use participants_list::ParticipantsList;
use text_generator::TextGenerator;

async fn fetch_pokemon(pokemon_name: &str) -> Result<String, String> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name);

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let json: Value = response
        .json()
        .await
        .map_err(|_| "No se pudo parsear el string".to_string())?;

    if let Some(name) = json.get("name").and_then(Value::as_str) {
        log!("ok");
        Ok(name.to_string())
    } else {
        Err("Nombre no encontrado".into())
    }
}

#[function_component]
fn App() -> Html {
    let result = use_state(|| "Empty".to_string());

    {
        let result = result.clone();
        use_effect_with((), move |_| {
            log!("effect");
            spawn_local(async move {
                let response = fetch_pokemon("ditto").await;
                match response {
                    Ok(name) => result.set(name),
                    Err(err) => result.set(err),
                }
            });
            || ()
        });
    }

    /*let fetch_pokemon_callback = {
        let result = result.clone();
        Callback::from(move || {
            let result = result.clone();
            spawn_local(async move {
                let response = fetch_pokemon("pikachu").await;
                match response {
                    Ok(name) => result.set(name),
                    Err(err) => result.set(err),
                }
            })
        })
    };*/

    html! {
        <div class = {classes!("px-10")}>
            <TextGenerator />
            <ParticipantsList />
            <p>{ (*result).clone() }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

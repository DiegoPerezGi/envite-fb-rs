use yew::prelude::*;

use crate::get_participants::get_participants;

#[function_component]
pub fn ParticipantsList() -> Html {
    let participants = get_participants().iter().map(|participant| html! {
        <button key={participant.id} class={classes!("bg-sky-500", "hover:bg-sky-700", "rounded", "px-4", "py-1", "mx-1")} >
            { format!("{}, {}, {}, {}, {}", participant.id, participant.name, participant.div, participant.team, participant.player) }
        </button>
    }).collect::<Html>();

    html! {
        <div class={classes!("px-10")}>
            { participants }
        </div>
    }
}

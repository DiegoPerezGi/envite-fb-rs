use yew::prelude::*;

pub struct Participant {
    id: usize,
    name: String,
    div: String,
    team: String,
    player: String,
}

#[function_component]
pub fn ParticipantsList() -> Html {
    let participants = vec![
        Participant {
            id: 1,
            name: "Diego".to_string(),
            div: "A".to_string(),
            team: "Uruguay".to_string(),
            player: "Erick Pulgar".to_string(),
        },
        Participant {
            id: 2,
            name: "María".to_string(),
            div: "B".to_string(),
            team: "Brasil".to_string(),
            player: "Neymar Jr.".to_string(),
        },
        Participant {
            id: 3,
            name: "Luis".to_string(),
            div: "C".to_string(),
            team: "Argentina".to_string(),
            player: "Lionel Messi".to_string(),
        },
        Participant {
            id: 4,
            name: "Ana".to_string(),
            div: "A".to_string(),
            team: "Francia".to_string(),
            player: "Kylian Mbappé".to_string(),
        },
        Participant {
            id: 5,
            name: "José".to_string(),
            div: "B".to_string(),
            team: "Alemania".to_string(),
            player: "Joshua Kimmich".to_string(),
        },
    ];

    let participants = participants.iter().map(|participant| html! {
        <button key={participant.id} class={classes!("bg-sky-500", "hover:bg-sky-700", "rounded", "px-4", "py-1", "mx-1")} >
            { format!("{}, {}, {}, {}, {}", participant.id, participant.name, participant.div, participant.team, participant.player) }
        </button>
        }).collect::<Html>();

    html! {
        <div class = {classes!("px-10")}>
            { participants }
        </div>
    }
}

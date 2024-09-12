pub struct Participant {
    pub id: usize,
    pub name: String,
    pub div: String,
    pub team: String,
    pub player: String,
}

// Función para obtener la lista de participantes
pub fn get_participants() -> Vec<Participant> {
    vec![
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
    ]
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Driver {
    name: String,
    nationality: String,
    championships_won: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct F1Team {
    name: String,
    base_country: String,
    championships_won: u32,
    engine_supplier: String,
    team_principal: String,
    drivers: [Driver; 2],
}

fn main() {
    // ---------- DESERIALIZE FROM RAW JSON ----------
    let json_data = r#"
    {
        "name": "Ferrari",
        "base_country": "Italy",
        "championships_won": 16,
        "engine_supplier": "Ferrari",
        "team_principal": "Frédéric Vasseur",
        "match": "Season 2024",
        "drivers": [
            {
                "name": "Charles Leclerc",
                "nationality": "Monaco",
                "championships_won": 0,
                "type": "Primary"
            },
            {
                "name": "Carlos Sainz",
                "nationality": "Spain",
                "championships_won": 0,
                "type": "Secondary"
            }
        ]
    }
    "#;

    let team: F1Team = serde_json::from_str(json_data).unwrap();
    println!("Ferrari team:\n{:#?}\n", team);

    // ---------- STRUCT → JSON → STRUCT ----------
    let red_bull = F1Team {
        name: "Red Bull Racing".to_string(),
        base_country: "United Kingdom".to_string(),
        championships_won: 6,
        engine_supplier: "Honda RBPT".to_string(),
        team_principal: "Christian Horner".to_string(),
        drivers: [
            Driver {
                name: "Max Verstappen".to_string(),
                nationality: "Dutch".to_string(),
                championships_won: 3,
            },
            Driver {
                name: "Sergio Pérez".to_string(),
                nationality: "Mexican".to_string(),
                championships_won: 0,
            },
        ],
    };

    let json = serde_json::to_string_pretty(&red_bull).unwrap();
    let deserialized_team: F1Team = serde_json::from_str(&json).unwrap();
    println!("Deserialized struct:\n{:#?}", deserialized_team);
}

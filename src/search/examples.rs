use super::state_graph::{StateGraph, StateGraphEdges};

pub static ORADEA: &'static str = "Oradea";
pub static ZERIND: &'static str = "Zerind";
pub static ARAD: &'static str = "Arad";
pub static SIBIU: &'static str = "Sibiu";
pub static FAGARAS: &'static str = "Fagaras";
pub static TIMISOARA: &'static str = "Timisoara";
pub static LUGOJ: &'static str = "Lugoj";
pub static MEHADIA: &'static str = "Mehadia";
pub static DROBETA: &'static str = "Drobeta";
pub static CRAIOVA: &'static str = "Craiova";
pub static RIMNICU_VILCEA: &'static str = "Rimnicu Vilcea";
pub static PITESTI: &'static str = "Pitesti";
pub static BUCHAREST: &'static str = "Bucharest";
pub static GIURGIU: &'static str = "Giurgiu";
pub static URZICENI: &'static str = "Urziceni";
pub static HIRSOVA: &'static str = "Hirsova";
pub static EFORIE: &'static str = "Eforie";
pub static VASLUI: &'static str = "Vaslui";
pub static IASI: &'static str = "Iasi";
pub static NEAMT: &'static str = "Neamt";

static ROMANIAN_CITIES: &'static [&'static str] = &[
    ORADEA,
    ZERIND,
    ARAD,
    SIBIU,
    FAGARAS,
    TIMISOARA,
    LUGOJ,
    MEHADIA,
    DROBETA,
    CRAIOVA,
    RIMNICU_VILCEA,
    PITESTI,
    BUCHAREST,
    GIURGIU,
    URZICENI,
    HIRSOVA,
    EFORIE,
    VASLUI,
    IASI,
    NEAMT,
];

pub fn generate_romania_roadmap() -> StateGraph<'static, &'static str, u16> {
    StateGraph::new(
        ROMANIAN_CITIES,
        StateGraphEdges::from_bidirectional(&[
            (&ORADEA, &ZERIND, 71),
            (&ZERIND, &ARAD, 75),
            (&ARAD, &TIMISOARA, 118),
            (&TIMISOARA, &LUGOJ, 111),
            (&LUGOJ, &MEHADIA, 70),
            (&MEHADIA, &DROBETA, 75),
            (&DROBETA, &CRAIOVA, 120),
            (&CRAIOVA, &RIMNICU_VILCEA, 146),
            (&RIMNICU_VILCEA, &SIBIU, 80),
            (&SIBIU, &ARAD, 140),
            (&SIBIU, &ORADEA, 151),
            (&SIBIU, &FAGARAS, 99),
            (&CRAIOVA, &PITESTI, 138),
            (&PITESTI, &RIMNICU_VILCEA, 97),
            (&FAGARAS, &BUCHAREST, 211),
            (&PITESTI, &BUCHAREST, 101),
            (&GIURGIU, &BUCHAREST, 90),
            (&URZICENI, &BUCHAREST, 85),
            (&URZICENI, &HIRSOVA, 98),
            (&EFORIE, &HIRSOVA, 86),
            (&URZICENI, &VASLUI, 142),
            (&IASI, &VASLUI, 92),
            (&IASI, &NEAMT, 87),
        ]),
    )
}

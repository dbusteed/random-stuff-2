use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Deserialize, Debug)]
pub struct LevelData {
    pub ball: Position,
    pub chaser: Vec<Position>,
    pub green: Vec<Rectangle>,
    pub hole: Position,
    pub par: usize,
    pub patrol: Vec<Vec<Position>>,
    pub wall: Vec<Rectangle>,
}

pub fn read_levels() -> Vec<LevelData> {
    let json = std::fs::read_to_string("./assets/levels.json").expect("");
    let data: Vec<LevelData> = serde_json::from_str(&json).unwrap();
    data
}
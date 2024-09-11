use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Command {
    Spawn { name: String, hue: f32 },
    Turn { token: String, side: Side },
    Walk { token: String, side: Side },
    Paint { token: String },
    Scan { token: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Side {
    Right,
    Left,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum CommandResult {
    Spawn {
        token: String,
    },
    Turn {},
    Walk {
        success: bool,
        point: f32,
        totalPoint: f32,
    },
    Paint {
        success: bool,
        yourPaints: Vec<Position>,
        totalPoint: f32,
    },
    Scan {
        whatYouCanSee: WhatYouCanSee,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum WhatYouCanSee {
    Food,
    Wall,
    Crab,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    x: f32,
    y: f32,
}

use crate::prelude::*;

pub struct GameState {
    pub map: Map,
    pub player: Player,
    pub rays: Vec<Ray>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player: Player::new(),
            map: Map::new(),
            rays: vec![Ray::new(); NUM_RAYS as usize],
        }
    }
}

use crate::prelude::*;

pub struct GameState {
    pub map: Map,
    pub player: Player,
    pub rays: Vec<Ray>,
}

impl GameState {
    pub fn new() -> Self {
        let mut rays: Vec<Ray> = vec![];
        rays.reserve_exact(NUM_RAYS as usize);

        GameState {
            player: Player::new(),
            map: Map::new(),
            rays,
        }
    }
}

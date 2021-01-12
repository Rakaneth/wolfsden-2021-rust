use coord_2d::{Coord, Size};
use direction::CardinalDirection;

pub struct GameState {
    screen_size: Size,
    player_coord: Coord,
}

impl GameState {
    pub fn new(screen_size: Size) -> Self {
        Self {
            player_coord: screen_size.to_coord().unwrap() / 2,
            screen_size,
        }
    }

    pub fn maybe_move_player(&mut self, direction: CardinalDirection) {
        let new_player_coord = self.player_coord + direction.coord();
        if new_player_coord.is_valid(self.screen_size) {
            self.player_coord = new_player_coord
        }
    }

    pub fn player_coord(&self) -> Coord {
        self.player_coord
    }
}

/*
#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Player,
}

entity_table::declare_entity_module! {
    components: {
        coord: Coord,
        tile: Tile,
    }
}

use components::Components;
*/
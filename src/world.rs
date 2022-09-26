use crate::player::{Player, PlayerHandle};
use crate::worlddata::WorldData;

pub struct World {
    pub players: Vec<Player>,
    pub data: WorldData,
}

impl World {
    pub fn new() -> World {
        World {
            players: Vec::new(),
            data: WorldData::new(),
        }
    }

    pub fn tick(&mut self) {
        for player in self.players.iter_mut() {
            player.tick(&mut self.data);
        }
    }

    pub fn add_player(&mut self, player: Player) -> PlayerHandle {
        self.players.push(player);
        self.players.len() - 1
    }

    pub fn get_player_by_handle(&mut self, player_handle: PlayerHandle) -> Option<&mut Player> {
        if player_handle < self.players.len() {
            Some(&mut self.players[player_handle])
        } else {
            None
        }
    }
}

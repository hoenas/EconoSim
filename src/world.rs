use serde::{Deserialize, Serialize};

use crate::player::{Player, PlayerHandle};
use crate::worlddata::WorldData;

#[derive(Serialize, Deserialize)]
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
        self.data.market_place.perform_paybacks(&mut self.players);
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

    pub fn get_player_name_by_handle(&self, player_handle: PlayerHandle) -> Option<&str> {
        if player_handle < self.players.len() {
            Some(&self.players[player_handle].name)
        } else {
            None
        }
    }
}

use crate::player::{Player, PlayerHandle};
use crate::resource::{Resource, ResourceHandle};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct World {
    players: Vec<Player>,
    resources: Vec<Resource>
}

impl World {
    pub fn add_player(&mut self, player: Player) -> PlayerHandle {
        self.players.push(player);
        self.players.len() - 1
    }

    pub fn get_player_by_handle(&self, player_handle: PlayerHandle) -> Option<&Player> {
        if player_handle < self.players.len() {
            Some(&self.players[player_handle])
        } else {
            None
        }
    }

    pub fn add_resource(&mut self, resource: Resource) -> ResourceHandle {
        self.resources.push(resource);
        self.resources.len() - 1
    }

    pub fn get_resource_by_handle(&self, resource_handle: ResourceHandle) -> Option<&Resource> {
        if resource_handle < self.resources.len() {
            Some(&self.resources[resource_handle])
        } else {
            None
        }
    }
}

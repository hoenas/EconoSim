use crate::market::marketplace::Marketplace;
use crate::player::{Player, PlayerHandle};
use crate::recipe::{Recipe, RecipeHandle};
use crate::resource::{Resource, ResourceHandle};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct WorldData {
    pub players: Vec<Player>,
    pub resources: Vec<Resource>,
    pub recipes: Vec<Recipe>,
    pub market_place: Marketplace,
}

impl WorldData {
    pub fn add_player(&mut self, player: Box<Player>) -> PlayerHandle {
        self.players.push(*player);
        self.players.len() - 1
    }

    pub fn get_player_by_handle(&self, player_handle: PlayerHandle) -> Option<&Player> {
        if player_handle < self.players.len() {
            Some(&self.players[player_handle])
        } else {
            None
        }
    }

    pub fn add_resource(&mut self, resource: Box<Resource>) -> ResourceHandle {
        self.resources.push(*resource);
        self.resources.len() - 1
    }

    pub fn get_resource_by_handle(&self, resource_handle: ResourceHandle) -> Option<&Resource> {
        if resource_handle < self.resources.len() {
            Some(&self.resources[resource_handle])
        } else {
            None
        }
    }

    pub fn add_recipe(&mut self, recipe: Box<Recipe>) -> RecipeHandle {
        self.recipes.push(*recipe);
        self.recipes.len() - 1
    }

    pub fn get_recipe_by_handle(&self, recipe_handle: RecipeHandle) -> Option<&Recipe> {
        if recipe_handle < self.recipes.len() {
            Some(&self.recipes[recipe_handle])
        } else {
            None
        }
    }
}

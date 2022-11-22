use crate::economy::recipe::{Recipe, RecipeHandle};
use crate::economy::resource::{Resource, ResourceHandle};
use crate::market::marketplace::Marketplace;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WorldData {
    pub processor_price: f64,
    pub resources: Vec<Resource>,
    pub recipes: Vec<Recipe>,
    pub market_place: Marketplace,
}

impl WorldData {
    pub fn new() -> WorldData {
        WorldData {
            processor_price: 1000.0,
            resources: Vec::new(),
            recipes: Vec::new(),
            market_place: Marketplace::new(),
        }
    }

    pub fn add_resource(&mut self, resource: Resource) -> ResourceHandle {
        self.resources.push(resource);
        let resource_count: ResourceHandle = self.resources.len() - 1;
        self.market_place.resource_count = resource_count;
        return resource_count;
    }

    pub fn get_resource_by_handle(
        &mut self,
        resource_handle: ResourceHandle,
    ) -> Option<&mut Resource> {
        if resource_handle < self.resources.len() {
            Some(&mut self.resources[resource_handle])
        } else {
            None
        }
    }

    pub fn get_resource_name_by_handle(&self, resource_handle: ResourceHandle) -> Option<&str> {
        if resource_handle < self.resources.len() {
            Some(&self.resources[resource_handle].name)
        } else {
            None
        }
    }

    pub fn add_recipe(&mut self, recipe: Recipe) -> RecipeHandle {
        self.recipes.push(recipe);
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

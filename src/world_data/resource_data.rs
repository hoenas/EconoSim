use crate::economy::resource::{Resource, ResourceHandle};
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

#[derive(Serialize, Deserialize)]
pub struct ResourceData {
    pub resources: Vec<Resource>,
}

impl ResourceData {
    pub fn new() -> ResourceData {
        ResourceData {
            resources: Vec::new(),
        }
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

    pub fn get_resource_handle_by_name(&self, resource_name: &str) -> Option<ResourceHandle> {
        for (handle, resource) in self.resources.iter().enumerate() {
            if resource.name == *resource_name {
                return Some(handle);
            }
        }
        log::error!("Resource '{}' not found!", resource_name);
        None
    }
}

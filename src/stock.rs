use crate::resource::Resource;

pub struct Stock <'a> {
    resources: Vec<(&'a Resource, f64)>,
}

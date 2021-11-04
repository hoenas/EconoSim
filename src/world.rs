use crate::worlddata::WorldData;

pub struct World {
    pub data: WorldData,
}

impl World {
    pub fn tick(&mut self) {
        for player in self.data.players.iter_mut() {
            player.tick()
        }
    }
}

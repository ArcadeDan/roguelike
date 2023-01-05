use specs::{WriteExpect, ReadStorage, System, Join, RunNow};

use crate::{GameComponents::context::{Position, BlocksTile, State}, Map::map::Map};

use super::{monster_ai_system::MonsterAI, visibility_system::VisibilitySystem};


pub struct MapIndexSystem {}

impl<'a> System<'a> for MapIndexSystem {
    type SystemData = (WriteExpect<'a, Map>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, BlocksTile>);
                        
    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers) = data;

        map.populate_blocked();
        for (position, _blocks) in (&position, &blockers).join() {
            let idx = map.xy_idx(position.x, position.y);
            map.blocked[idx] = true;
        }
    }
}


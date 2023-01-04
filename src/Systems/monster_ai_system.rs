use rltk::{console, Point};
use specs::{Join, ReadExpect, ReadStorage, System};

use crate::{
    GameComponents::context::{Position, Viewshed},
    NPC::enemy::Monster,
};

pub struct MonsterAI {}
impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewshed, monster) = data;
        // when player is in view on a monster
        for (viewshed, _monster) in (&viewshed, &monster).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("Monster shouts insults"));
            }
        }
    }
}

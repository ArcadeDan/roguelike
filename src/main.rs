#![allow(unused_imports)]

use rltk::{RGB};
use specs::{
    Builder, World, WorldExt,
};

mod Rect;
use Rect::rect;
mod Map;
use Map::{*};
mod NPC;

mod Components;
use Components as GameComponents;
mod Systems;
use Systems as GameSystems;
use crate::map::Map as GameMap;
use crate::{
    GameComponents::context::{Player, Position, Renderable, State, Viewshed},
    Map::map::new_map_and_corridoors,
};


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map: GameMap = new_map_and_corridoors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    rltk::main_loop(context, gs)
}

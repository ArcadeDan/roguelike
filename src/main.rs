#![allow(unused_imports)]

use rltk::RGB;
use specs::{Builder, World, WorldExt};
#[allow(non_snake_case)]
mod Rect;
use Rect::rect;
#[allow(non_snake_case)]
mod Map;
use Map::*;
#[allow(non_snake_case)]
mod Components;
#[allow(non_snake_case)]
mod NPC;
use Components as GameComponents;
#[allow(non_snake_case)]
mod Systems;
use crate::map::Map as GameMap;
use crate::{
    GameComponents::context::{Player, Position, Renderable, State, Viewshed},
    Map::map::new_map_and_corridoors,
};
use Systems as GameSystems;

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Rogue")
        .with_fullscreen(true)
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map: GameMap = new_map_and_corridoors();
    let (player_x, player_y) = map.rooms[0].center();

    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: rltk::to_cp437('b'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .build();
    }

    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
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

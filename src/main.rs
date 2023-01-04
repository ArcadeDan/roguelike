#![allow(unused_imports)]

use rltk::{Point, RGB};
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
use crate::NPC::enemy::Monster;
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
    let mut gs = State {
        ecs: World::new(),
        runstate: GameComponents::context::RunState::Running,
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();

    let map: GameMap = new_map_and_corridoors();
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = rltk::RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        let rand_glyph: rltk::FontCharType;
        let roll = rng.roll_dice(1, 2);

        match roll {
            1 => rand_glyph = rltk::to_cp437('b'),
            _ => rand_glyph = rltk::to_cp437('g'),
        }

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: rand_glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

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

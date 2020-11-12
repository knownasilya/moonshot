use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod visibility_system;
use visibility_system::VisibilitySystem;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            }
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("moonshot").build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Moonshot>();
    gs.ecs.register::<Viewshed>();

    let map: Map = Map::test_map();
    let (player_x, player_y) = (35, 26);

    // let mut rng = rltk::RandomNumberGenerator::new();
    // for room in map.rooms.iter().skip(1) {
    //     let (x, y) = room.center();

    //     let glyph: rltk::FontCharType;
    //     let roll = rng.roll_dice(1, 2);
    //     match roll {
    //         1 => glyph = rltk::to_cp437('g'),
    //         _ => glyph = rltk::to_cp437('o'),
    //     }

    //     gs.ecs
    //         .create_entity()
    //         .with(Position { x, y })
    //         .with(Renderable {
    //             glyph: glyph,
    //             fg: RGB::named(rltk::RED),
    //             bg: RGB::named(rltk::BLACK),
    //         })
    //         .with(Viewshed {
    //             visible_tiles: Vec::new(),
    //             range: 8,
    //             dirty: true,
    //         })
    //         .build();
    // }

    gs.ecs.insert(map);

    // Add player
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

    // Add moonshot
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x + 2,
            y: player_y + 2,
        })
        .with(Moonshot {})
        .with(Renderable {
            glyph: rltk::to_cp437('m'),
            fg: RGB::named(rltk::PURPLE),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    rltk::main_loop(context, gs)
}

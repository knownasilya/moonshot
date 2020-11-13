use rltk::Point;
use rltk::{console, GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
mod gamelog;
mod gui;
mod map;
mod moonshot_ai;
mod player;
mod rect;
mod spawners;
mod visibility_system;

use components::*;
use map::*;
use moonshot_ai::*;
use player::*;
use rect::Rect;
use visibility_system::VisibilitySystem;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut moon = MoonshotAI {};
        moon.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

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

        gui::draw_ui(&self.ecs, ctx);
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("moonshot").build()?;
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Moonshot>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksVisibility>();
    gs.ecs.register::<Door>();

    let map: Map = Map::test_map();
    let (player_x, player_y) = (35, 26);

    spawners::door(&mut gs.ecs, 38, 29);

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

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
        .with(Position { x: 37, y: 30 })
        .with(Moonshot {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 12,
            dirty: true,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('m'),
            fg: RGB::named(rltk::PURPLE),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    // Add Gamelog
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["You wake to the sound of scratching coming from the door".to_string()],
    });

    rltk::main_loop(context, gs)
}

use super::{
  BlocksTile, BlocksVisibility, Door, Moonshot, Name, Player, Position, Renderable, Viewshed,
};
use rltk::RGB;
use specs::prelude::*;

pub fn player(ecs: &mut World, x: i32, y: i32) {
  ecs
    .create_entity()
    .with(Position { x, y })
    .with(Renderable {
      glyph: rltk::to_cp437('@'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::BLACK),
      render_order: 0,
    })
    .with(Player {})
    .with(Name {
      name: "Player".to_string(),
    })
    .with(Viewshed {
      visible_tiles: Vec::new(),
      range: 8,
      dirty: true,
    })
    .build();
}

pub fn moonshot(ecs: &mut World, x: i32, y: i32) {
  ecs
    .create_entity()
    .with(Position { x, y })
    .with(Moonshot {})
    .with(Name {
      name: "Moonshot".to_string(),
    })
    .with(BlocksTile {})
    .with(Viewshed {
      visible_tiles: Vec::new(),
      range: 12,
      dirty: true,
    })
    .with(Renderable {
      glyph: rltk::to_cp437('m'),
      fg: RGB::named(rltk::PURPLE),
      bg: RGB::named(rltk::BLACK),
      render_order: 0,
    })
    .build();
}

pub fn door(ecs: &mut World, x: i32, y: i32) {
  ecs
    .create_entity()
    .with(Position { x, y })
    .with(BlocksTile {})
    .with(Renderable {
      glyph: rltk::to_cp437('+'),
      fg: RGB::named(rltk::CHOCOLATE),
      bg: RGB::named(rltk::BLACK),
      render_order: 2,
    })
    .with(Name {
      name: "Door".to_string(),
    })
    // .with(BlocksTile {})
    .with(BlocksVisibility {})
    .with(Door { open: false })
    .build();
}

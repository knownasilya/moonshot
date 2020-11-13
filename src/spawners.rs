use super::{BlocksVisibility, Door, Name, Position, Renderable};
use rltk::RGB;
use specs::prelude::*;

pub fn door(ecs: &mut World, x: i32, y: i32) {
  ecs
    .create_entity()
    .with(Position { x, y })
    .with(Renderable {
      glyph: rltk::to_cp437('+'),
      fg: RGB::named(rltk::CHOCOLATE),
      bg: RGB::named(rltk::BLACK),
    })
    .with(Name {
      name: "Door".to_string(),
    })
    // .with(BlocksTile {})
    .with(BlocksVisibility {})
    .with(Door { open: false })
    .build();
}

use crate::gamelog::GameLog;
use rltk::{Console, Rltk, RGB};
use specs::prelude::*;

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
  ctx.draw_box(
    0,
    43,
    79,
    6,
    RGB::named(rltk::WHITE),
    RGB::named(rltk::BLACK),
  );

  let log = ecs.fetch::<GameLog>();

  let mut y = 44;
  for s in log.entries.iter().rev() {
    if y < 49 {
      ctx.print(2, y, s);
    }
    y += 1;
  }
}

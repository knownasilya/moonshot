use super::{Map, Moonshot, Position, Viewshed};
use rltk::{console, field_of_view, Point};
use specs::prelude::*;

pub struct MoonshotAI {}

impl<'a> System<'a> for MoonshotAI {
  type SystemData = (
    ReadExpect<'a, Point>,
    ReadStorage<'a, Viewshed>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Moonshot>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (player_pos, viewshed, pos, moonshot) = data;

    for (viewshed, _pos, _moonshot) in (&viewshed, &pos, &moonshot).join() {
      if viewshed.visible_tiles.contains(&*player_pos) {
        console::log("Moonshot sits lazily in the heat of the sun");
      }
    }
  }
}

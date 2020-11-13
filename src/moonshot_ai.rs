use super::{Map, Moonshot, Position, Viewshed};
use rltk::{console, field_of_view, Point};
use specs::prelude::*;

pub struct MoonshotAI {}

impl<'a> System<'a> for MoonshotAI {
  type SystemData = (
    ReadStorage<'a, Viewshed>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Moonshot>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (viewshed, pos, moonshot) = data;

    for (_viewshed, _pos, _moonshot) in (&viewshed, &pos, &moonshot).join() {
      console::log("Moonshot sits lazily in the heat of the sun");
    }
  }
}

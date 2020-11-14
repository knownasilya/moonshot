use super::{Map, Moonshot, Position, Viewshed};
use rltk::{console, field_of_view, Point};
use specs::prelude::*;

pub struct MoonshotAI {}

impl<'a> System<'a> for MoonshotAI {
  type SystemData = (
    WriteExpect<'a, Map>,
    ReadExpect<'a, Point>,
    WriteStorage<'a, Viewshed>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Moonshot>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut map, player_pos, mut viewshed, mut pos, moonshot) = data;

    for (mut viewshed, mut pos, _moonshot) in (&mut viewshed, &mut pos, &moonshot).join() {
      let distance =
        rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
      if distance < 1.5 {
        console::log("Moonshot sits in the warm heat of the sun");
      } else if viewshed.visible_tiles.contains(&*player_pos) {
        console::log("Moonshot follows you");

        let path = rltk::a_star_search(
          map.xy_idx(pos.x, pos.y) as i32,
          map.xy_idx(player_pos.x, player_pos.y) as i32,
          &mut *map,
        );
        if path.success && path.steps.len() > 1 {
          pos.x = path.steps[1] as i32 % map.width;
          pos.y = path.steps[1] as i32 / map.width;
          viewshed.dirty = true;
        }
      }
    }
  }
}

use super::{Map, Moonshot, Name, Position, RunState, Viewshed};
use rltk::{console, field_of_view, Point};
use specs::prelude::*;

pub struct MoonshotAI {}

impl<'a> System<'a> for MoonshotAI {
  type SystemData = (
    WriteExpect<'a, Map>,
    ReadExpect<'a, Point>,
    ReadExpect<'a, RunState>,
    WriteStorage<'a, Viewshed>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Name>,
    ReadStorage<'a, Moonshot>,
    WriteExpect<'a, rltk::RandomNumberGenerator>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut map, player_pos, runstate, mut viewshed, mut pos, name, moonshot, mut rng) = data;

    if *runstate != RunState::NpcTurn {
      return;
    }

    for (mut viewshed, mut pos, _moonshot, name) in
      (&mut viewshed, &mut pos, &moonshot, &name).join()
    {
      let distance =
        rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
      if distance < 2.0 {
        console::log(format!("{} stops to rest", name.name.to_string()));
      } else if viewshed.visible_tiles.contains(&*player_pos) {
        console::log(format!("{} follows you", name.name.to_string()));

        let mut x = player_pos.x;
        let mut y = player_pos.y;
        let move_roll = rng.roll_dice(1, 5);
        match move_roll {
          1 => x -= 2,
          2 => x += 2,
          3 => y -= 2,
          4 => y += 2,
          _ => {}
        }

        let path = rltk::a_star_search(
          map.xy_idx(pos.x, pos.y) as i32,
          map.xy_idx(x, y) as i32,
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

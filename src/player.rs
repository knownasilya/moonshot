use super::{
  BlocksTile, BlocksVisibility, Door, Map, Player, Position, Renderable, RunState, State, Viewshed,
};
use rltk::Point;
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut doors = ecs.write_storage::<Door>();
  let mut positions = ecs.write_storage::<Position>();
  let mut blocks_visibility = ecs.write_storage::<BlocksVisibility>();
  let mut blocks_movement = ecs.write_storage::<BlocksTile>();
  let mut players = ecs.write_storage::<Player>();
  let mut viewsheds = ecs.write_storage::<Viewshed>();
  let mut ppos = ecs.write_resource::<Point>();
  let mut renderables = ecs.write_storage::<Renderable>();
  let map = ecs.fetch::<Map>();

  for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
    let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
    if !map.blocked[destination_idx] {
      pos.x = min(79, max(0, pos.x + delta_x));
      pos.y = min(49, max(0, pos.y + delta_y));

      // Update PlayerPosition resource
      ppos.x = pos.x;
      ppos.y = pos.y;

      viewshed.dirty = true;
    }

    let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

    for potential_target in map.tile_content[destination_idx].iter() {
      let door = doors.get_mut(*potential_target);
      if let Some(door) = door {
        door.open = true;
        blocks_visibility.remove(*potential_target);
        blocks_movement.remove(*potential_target);
        // let glyph = renderables.get_mut(*potential_target).unwrap();
        // glyph.glyph = rltk::to_cp437('/');
        viewshed.dirty = true;
      }
    }
  }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
  // Player movement
  match ctx.key {
    None => return RunState::Paused, // Nothing happened
    Some(key) => match key {
      VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
        try_move_player(-1, 0, &mut gs.ecs)
      }

      VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
        try_move_player(1, 0, &mut gs.ecs)
      }

      VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
        try_move_player(0, -1, &mut gs.ecs)
      }

      VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
        try_move_player(0, 1, &mut gs.ecs)
      }

      // Diagonals
      VirtualKeyCode::Numpad9 | VirtualKeyCode::Y => try_move_player(1, -1, &mut gs.ecs),

      VirtualKeyCode::Numpad7 | VirtualKeyCode::U => try_move_player(-1, -1, &mut gs.ecs),

      VirtualKeyCode::Numpad3 | VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

      VirtualKeyCode::Numpad1 | VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

      _ => return RunState::Paused,
    },
  }
  RunState::Running
}

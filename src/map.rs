use rltk::{Rltk, RGB};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
  Wall,
  Floor,
  Tree,
}

impl TileType {
  pub fn is_blocked(&self) -> bool {
    match *self {
      TileType::Wall => true,
      TileType::Tree => true,
      _ => false,
    }
  }
}

pub fn new_map_start() -> Vec<TileType> {
  let mut map = vec![TileType::Floor; 80 * 50];

  // Make the boundaries walls
  for x in 0..80 {
    map[xy_idx(x, 0)] = TileType::Wall;
    map[xy_idx(x, 49)] = TileType::Wall;
  }
  for y in 0..50 {
    map[xy_idx(0, y)] = TileType::Wall;
    map[xy_idx(79, y)] = TileType::Wall;
  }

  map[xy_idx(20, 15)] = TileType::Tree;

  map
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn new_map_test() -> Vec<TileType> {
  let mut map = vec![TileType::Floor; 80 * 50];

  // Make the boundaries walls
  for x in 0..80 {
    map[xy_idx(x, 0)] = TileType::Wall;
    map[xy_idx(x, 49)] = TileType::Wall;
  }
  for y in 0..50 {
    map[xy_idx(0, y)] = TileType::Wall;
    map[xy_idx(79, y)] = TileType::Wall;
  }

  // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
  // First, obtain the thread-local RNG:
  let mut rng = rltk::RandomNumberGenerator::new();

  for _i in 0..400 {
    let x = rng.roll_dice(1, 79);
    let y = rng.roll_dice(1, 49);
    let idx = xy_idx(x, y);
    if idx != xy_idx(40, 25) {
      map[idx] = TileType::Wall;
    }
  }

  map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
  let mut y = 0;
  let mut x = 0;
  for tile in map.iter() {
    // Render a tile depending upon the tile type
    match tile {
      TileType::Floor => {
        ctx.set(
          x,
          y,
          RGB::from_f32(0.431, 0.475, 0.467),
          // RGB::from_f32(0., 0., 0.),
          // RGB::named(rltk::GREEN),
          RGB::named(rltk::WHITE),
          rltk::to_cp437('.'),
        );
      }
      TileType::Wall => {
        ctx.set(
          x,
          y,
          RGB::from_f32(0.188, 0.239, 0.231),
          // RGB::from_f32(0., 0., 0.),
          // RGB::named(rltk::GREEN),
          RGB::named(rltk::WHITE),
          rltk::to_cp437('#'),
        );
      }
      TileType::Tree => {
        ctx.set(
          x,
          y,
          // RGB::from_f32(0.188, 0.239, 0.231),
          // RGB::from_f32(0., 0., 0.),
          RGB::named(rltk::DARK_OLIVE),
          RGB::named(rltk::WHITE),
          rltk::to_cp437('♣'),
        );
      }
    }

    // Move the coordinates
    x += 1;
    if x > 79 {
      x = 0;
      y += 1;
    }
  }
}

pub fn xy_idx(x: i32, y: i32) -> usize {
  (y as usize * 80) + x as usize
}

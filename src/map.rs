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

pub struct Map {
  pub tiles: Vec<TileType>,
  pub width: i32,
  pub height: i32,
}

impl Map {
  pub fn draw_map(&self, ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in self.tiles.iter() {
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

  pub fn xy_idx(width: i32, x: i32, y: i32) -> usize {
    (y as usize * width as usize) + x as usize
  }

  pub fn new_map_start() -> Map {
    let mut map = Map {
      tiles: vec![TileType::Floor; 80 * 50],
      width: 80,
      height: 50,
    };

    // Make the boundaries walls
    for x in 0..80 {
      map.tiles[Map::xy_idx(map.width, x, 0)] = TileType::Wall;
      map.tiles[Map::xy_idx(map.width, x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
      map.tiles[Map::xy_idx(map.width, 0, y)] = TileType::Wall;
      map.tiles[Map::xy_idx(map.width, 79, y)] = TileType::Wall;
    }

    map.tiles[Map::xy_idx(map.width, 20, 15)] = TileType::Tree;

    map
  }

  /// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
  /// look awful.
  pub fn new_map_test() -> Map {
    let mut map = Map {
      tiles: vec![TileType::Floor; 80 * 50],
      width: 80,
      height: 50,
    };

    // Make the boundaries walls
    for x in 0..80 {
      map.tiles[Map::xy_idx(map.width, x, 0)] = TileType::Wall;
      map.tiles[Map::xy_idx(map.width, x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
      map.tiles[Map::xy_idx(map.width, 0, y)] = TileType::Wall;
      map.tiles[Map::xy_idx(map.width, 79, y)] = TileType::Wall;
    }

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
      let x = rng.roll_dice(1, 79);
      let y = rng.roll_dice(1, 49);
      let idx = Map::xy_idx(map.width, x, y);
      if idx != Map::xy_idx(map.width, 40, 25) {
        map.tiles[idx] = TileType::Wall;
      }
    }

    map
  }
}

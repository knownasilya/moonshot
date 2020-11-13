use super::{Door, Position, Rect, Renderable};
use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;
use std::cmp::{max, min};
use std::collections::HashSet;

const MAPWIDTH: usize = 80;
const MAPHEIGHT: usize = 50;
const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
  Empty,
  WallV,
  WallH,
  Floor,
  Door,
  Tree,
  TallGrass,
  Mountain,
}

impl TileType {
  pub fn tile_from_glyph(glyph: char) -> TileType {
    match glyph {
      '#' => TileType::Tree,
      '\'' => TileType::TallGrass,
      '|' => TileType::WallV,
      '-' => TileType::WallH,
      '.' => TileType::Floor,
      '+' => TileType::Door,
      '▲' => TileType::Mountain,
      ' ' => TileType::Empty,
      _ => TileType::Empty,
    }
  }

  pub fn is_blocked(&self) -> bool {
    match &self {
      TileType::Tree => true,
      TileType::WallV => true,
      TileType::WallH => true,
      TileType::Mountain => true,
      TileType::Door => false,
      TileType::TallGrass => false,
      TileType::Floor => false,
      TileType::Empty => false,
    }
  }
}

#[derive(Default)]
pub struct Map {
  pub tiles: Vec<TileType>,
  pub rooms: Vec<Rect>,
  pub width: i32,
  pub height: i32,
  pub revealed_tiles: Vec<bool>,
  pub visible_tiles: Vec<bool>,
  pub view_blocked: HashSet<usize>,
}

impl Map {
  pub fn xy_idx(&self, x: i32, y: i32) -> usize {
    (y as usize * self.width as usize) + x as usize
  }

  fn apply_room_to_map(&mut self, room: &Rect) {
    for y in room.y1 + 1..=room.y2 {
      for x in room.x1 + 1..=room.x2 {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = TileType::Floor;
      }
    }
  }

  fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
      let idx = self.xy_idx(x, y);
      if idx > 0 && idx < self.width as usize * self.height as usize {
        self.tiles[idx as usize] = TileType::Floor;
      }
    }
  }

  fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
      let idx = self.xy_idx(x, y);
      if idx > 0 && idx < self.width as usize * self.height as usize {
        self.tiles[idx as usize] = TileType::Floor;
      }
    }
  }

  pub fn test_map() -> Map {
    let mut map = Map {
      tiles: vec![],
      rooms: Vec::new(),
      width: MAPWIDTH as i32,
      height: MAPHEIGHT as i32,
      revealed_tiles: vec![false; MAPCOUNT],
      visible_tiles: vec![false; MAPCOUNT],
      view_blocked: HashSet::new(),
    };

    let tiles = vec![
      "▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲",
      "▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲",
      "▲...............................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "..................................---------...#.................................",
      "...........................#......|.......|.....................................",
      "..................................|.......|.#..........''.......................",
      "......................#...........|.......|...........'.''......................",
      "..................................|.......|...#.........'.......................",
      "..................................---- ----.....................................",
      "................................................................................",
      "........................................................''......................",
      "..............................................#.................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "................................................................................",
      "▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲",
    ];

    for row in tiles.iter() {
      let line: Vec<char> = row.chars().collect();

      for glyph in line {
        map.tiles.push(TileType::tile_from_glyph(glyph));
      }
    }

    map
  }

  /// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
  /// This gives a handful of random rooms and corridors joining them together.
  pub fn new_map_rooms_and_corridors() -> Map {
    let mut map = Map {
      tiles: vec![TileType::Tree; MAPCOUNT],
      rooms: Vec::new(),
      width: MAPWIDTH as i32,
      height: MAPHEIGHT as i32,
      revealed_tiles: vec![false; MAPCOUNT],
      visible_tiles: vec![false; MAPCOUNT],
      view_blocked: HashSet::new(),
    };

    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..MAX_ROOMS {
      let w = rng.range(MIN_SIZE, MAX_SIZE);
      let h = rng.range(MIN_SIZE, MAX_SIZE);
      let x = rng.roll_dice(1, map.width - w - 1) - 1;
      let y = rng.roll_dice(1, map.height - h - 1) - 1;
      let new_room = Rect::new(x, y, w, h);
      let mut ok = true;
      for other_room in map.rooms.iter() {
        if new_room.intersect(other_room) {
          ok = false
        }
      }
      if ok {
        map.apply_room_to_map(&new_room);

        if !map.rooms.is_empty() {
          let (new_x, new_y) = new_room.center();
          let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
          if rng.range(0, 2) == 1 {
            map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
            map.apply_vertical_tunnel(prev_y, new_y, new_x);
          } else {
            map.apply_vertical_tunnel(prev_y, new_y, prev_x);
            map.apply_horizontal_tunnel(prev_x, new_x, new_y);
          }
        }

        map.rooms.push(new_room);
      }
    }

    map
  }
}

impl BaseMap for Map {
  fn is_opaque(&self, idx: usize) -> bool {
    let is_match = match self.tiles[idx] {
      TileType::WallV => true,
      TileType::WallH => true,
      TileType::Door => true,
      _ => false,
    };

    is_match || self.view_blocked.contains(&idx)
  }
}

impl Algorithm2D for Map {
  fn dimensions(&self) -> Point {
    Point::new(self.width, self.height)
  }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
  let map = ecs.fetch::<Map>();

  let mut y = 0;
  let mut x = 0;
  for (idx, tile) in map.tiles.iter().enumerate() {
    // Render a tile depending upon the tile type
    if map.revealed_tiles[idx] {
      let glyph;
      let mut fg;
      match tile {
        TileType::Empty => {
          glyph = rltk::to_cp437(' ');
          fg = RGB::from_f32(0.0, 0.0, 0.0);
        }
        TileType::Floor => {
          glyph = rltk::to_cp437('.');
          fg = RGB::from_f32(0.0, 0.5, 0.5);
        }
        TileType::WallV => {
          glyph = rltk::to_cp437('|');
          fg = RGB::from_f32(0., 1.0, 0.);
        }
        TileType::WallH => {
          glyph = rltk::to_cp437('-');
          fg = RGB::from_f32(0., 1.0, 0.);
        }
        TileType::Door => {
          glyph = rltk::to_cp437('+');
          fg = RGB::from_f32(0., 1.0, 0.);
        }
        TileType::Tree => {
          glyph = rltk::to_cp437('#');
          fg = RGB::from_f32(0., 1.0, 0.);
        }
        TileType::TallGrass => {
          glyph = rltk::to_cp437('\'');
          fg = RGB::from_f32(0., 0.5, 0.);
        }
        TileType::Mountain => {
          glyph = rltk::to_cp437('▲');
          fg = RGB::from_f32(0.0, 0.0, 1.0);
        }
      }
      if !map.visible_tiles[idx] {
        fg = fg.to_greyscale()
      }
      ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
    }

    // let doors = ecs.write_storage::<Door>();
    // let positions = ecs.write_storage::<Position>();
    // let mut renderables = ecs.write_storage::<Renderable>();

    // for (pos, _door, renderable) in (&positions, &doors, &mut renderables).join() {
    //   if idx == map.xy_idx(pos.x, pos.y) {
    //     if map.revealed_tiles[idx] {
    //       renderable.fg = RGB::named(rltk::CHOCOLATE)
    //     } else if map.visible_tiles[idx] {
    //       renderable.fg = renderable.fg.to_greyscale()
    //     }
    //   }
    // }

    // Move the coordinates
    x += 1;
    if x > MAPWIDTH - 1 {
      x = 0;
      y += 1;
    }
  }
}

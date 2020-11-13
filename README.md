# moonshot

## Setup

Use the latest version (not nightly) to get `rltk` to compile.

```sh
rustup update 1.47.0
```

### Build wasm

```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/moonshot.wasm --out-dir wasm --no-modules --no-typescript
```

## Resources

- Map Editor https://notimetoplay.itch.io/ascii-mapper
- Tutorial https://bfnightly.bracketproductions.com/rustbook/chapter_0.html
- Character table http://dwarffortresswiki.org/index.php/Character_table
- Rougelike dev resource https://www.reddit.com/r/roguelikedev/comments/3y4z3x/faq_friday_28_map_object_representation/
- Podcast http://www.roguelikeradio.com/2013/12/episode-83-ascii.html
- Color schema http://paletton.com/#uid=73d0u0k5qgb2NnT41jT74c8bJ8X
- Design inspiration
  - https://sites.google.com/site/broguegame/
  - https://camo.githubusercontent.com/7766948b0b37b64497df0c8d6f8db45dd5d90ede/68747470733a2f2f692e696d6775722e636f6d2f4e756e4f49684e2e706e67

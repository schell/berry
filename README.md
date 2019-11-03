# berry
`berry` is a user interface library for games and multimedia applications. It's
also a bit like a database. Widgets' behaviors are defined by their relationships
to each other and their state can be queried by interested parties (including you).

## What it is
Under the hood `berry` is the `specs` entity component system paired with a
simple declarative 2d graphics API and the Cassowary linear constraint solving
algorithm.

## Build
First get the sdl2 libs:

ubuntu:
```
sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev
```

then `cargo build` or `cargo run`.

## TODO (besides the ones found in the source code)
- [ ] screenshots
- [ ] documentation
- [ ] examples
- [ ] wasm/emscripten backends
- [ ] place berry with respect to
  [other GUIs](https://www.reddit.com/r/rust/comments/dr04ce/reclutch_a_simple_rust_ui_foundation/f6d4i9r?utm_source=share&utm_medium=web2x)

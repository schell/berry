# berry
This is a berry different kind of user interface. In a world of immediate and
retained mode interfaces, complex widget trees and callback hell, berry has no
place. Instead berry is a bit more like a database - widgets' behaviors are
defined by their relationships to each other and their state can be queried by
interested parties (including you).

## Build
First get the sdl2 libs:
```
sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev
```
then `cargo build`.

## What it is
Under the hood `berry` is the `specs` entity component system paired with a
simple declarative 2d graphics API and the Cassowary linear constraint solving
algorithm.

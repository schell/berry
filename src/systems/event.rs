use sdl2::render::{BlendMode, Texture, TextureAccess, TextureCreator, TextureQuery, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::pixels;
use sdl2::rect::Rect;
use specs::prelude::*;
use std::collections::HashMap;

use super::super::components::*;


pub struct EventSystem;


impl<'a> System<'a> for EventSystem {
  type SystemData = (
    ReadStorage<'a, Sizes>
  );
}

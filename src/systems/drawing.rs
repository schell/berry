use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas, WindowContext};
use sdl2::ttf::{Font, Sdl2TtfContext};
use specs::prelude::{Component, ReadStorage, System, VecStorage, World, WorldExt};
use std::collections::HashMap;


type FontMap<'ctx> = HashMap<(String, u16), Font<'ctx, 'static>>;

type TextureMap<'ctx> = HashMap<String, Texture<'ctx>>;


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
  x: i32,
  y: i32
}


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct FontDesc {
  path: String,
  size: u16
}


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
  a: u8
}


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Text(String);


pub struct DrawingSystem<'ctx> {
  fonts: FontMap<'ctx>,
  textures: TextureMap<'ctx>,
  canvas: Option<&'ctx mut WindowCanvas>,
  tex_creator: Option<&'ctx TextureCreator<WindowContext>>,
  ttf: Option<&'ctx Sdl2TtfContext>
}


impl<'ctx> DrawingSystem<'ctx> {
  pub fn new(
    canvas: &'ctx mut WindowCanvas,
    tex_creator: &'ctx TextureCreator<WindowContext>,
    ttf: &'ctx Sdl2TtfContext
  ) -> DrawingSystem<'ctx> {
    DrawingSystem {
      fonts: HashMap::new(),
      textures: HashMap::new(),
      canvas: Some(canvas),
      tex_creator: Some(tex_creator),
      ttf: Some(ttf)
    }
  }
}


impl<'a, 'ctx> System<'a> for DrawingSystem<'ctx> {
  type SystemData = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, FontDesc>,
    ReadStorage<'a, Color>,
    ReadStorage<'a, Text>,
  );

  fn run(&mut self, (positions, font_descs, colors, texts): Self::SystemData) {

  }
}

use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Font;

use specs::prelude::*;

use super::text;
use super::{FontMap, UI, Widget};


/// A label for displaying some bit of text on the screen.
#[derive(Debug, Component, Clone, Hash)]
#[storage(HashMapStorage)]
pub struct Label;


impl Label {
  pub fn new() -> Label {
    Label {
      text: "Label".to_string(),
      font: ("komika.ttf".to_string(), 16),
      color: Color::RGB(0x33, 0x33, 0x33),
      position: (0, 0)
    }
  }
}


impl Widget for Label {
  //pub fn new(
  //  font: &'ctx Font<'ctx, 'ctx>,
  //  ui: &'ctx UI
  //) -> Label<'ctx> {
  //  Label {
  //    text: "Label".to_string(),
  //    color: Color::RGB(0, 0, 0),
  //    position: (0, 0),
  //    font,
  //    texture: None,
  //    ui
  //  }
  //}

  ///// Change the color of the label
  //pub fn color(mut self, c: Color) -> Self {
  //  self.set_color(c);
  //  self
  //}

  //pub fn set_color(&mut self, c:Color) {
  //  self.color = c;
  //  self.texture = None;
  //}

  ///// Change the font of the label
  //pub fn font(mut self, f: &'ctx Font<'ctx, 'ctx>) -> Self {
  //  self.set_font(f);
  //  self
  //}

  //pub fn set_font(&mut self, f:&'ctx Font<'ctx, 'ctx>) {
  //  self.font = f;
  //  self.texture = None;
  //}

  ///// Change the text of the label
  //pub fn text(mut self, t: &str) -> Self {
  //  self.set_text(t);
  //  self
  //}

  //pub fn set_text(&mut self, t: &str) {
  //  self.text = t.to_string();
  //  self.texture = None;
  //}

  ///// Change the position of the label
  //pub fn position(mut self, p: (i32, i32)) -> Self {
  //  self.position = p;
  //  self
  //}

  //pub fn set_position(&mut self, p:(i32, i32)) {
  //  self.position = p;
  //}

  /// Draw the label into the provided canvas.
  fn draw<'ctx>(&self, ui: &'ctx mut UI, fonts: &'ctx mut FontMap<'ctx>) -> (Texture<'ctx>, u32, u32) {
    let font =
      UI::get_font(&ui.ttf, &self.font.0, self.font.1, fonts)
      .unwrap();

    text::cache(
      self.text.as_str(),
      self.color,
      font,
      &ui.tex_creator
    )
  }

  //pub fn draw(&mut self, canvas: &mut WindowCanvas) {
  //  let (x, y) =
  //    self.position;

  //  let (tex, w, h) =
  //    self
  //    .get_cached();

  //  let tex_rect =
  //    Rect::new(
  //      x, y,
  //      w, h
  //    );

  //  canvas
  //    .copy(
  //      &tex,
  //      None,
  //      Some(tex_rect)
  //    )
  //    .unwrap();
  //}

  ///// Returns the rendered size of the text. If the label has not been cached
  ///// this causes it to be cached.
  //pub fn get_size(&mut self) -> (u32, u32) {
  //  let (_, w, h) =
  //    self
  //    .get_cached();
  //  (w, h)
  //}
}

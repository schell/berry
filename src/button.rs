use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::render::{BlendMode, WindowCanvas};
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureAccess, TextureCreator};
use sdl2::ttf::Font;

use super::label::Label;
use super::MouseUpdate;
use super::{UI, Widget};

/// The on/over/off state of a button.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum ButtonState {
  Off,
  Over,
  Down
}


/// A button - for clicking
#[derive(Clone, Hash)]
pub struct Button {
  label: Label,
  position: (i32, i32),
  padding: u32,
  state: ButtonState,
}


impl Button {

  //pub fn new(
  //  font: &'ctx Font<'ctx, 'ctx>,
  //  ui: &'ctx mut UI
  //) -> Button<'ctx> {

  //  let label =
  //    Label::new(
  //      font,
  //      ui
  //    )
  //    .position((2, 2))
  //    .text("Button");

  //  Button {
  //    label,
  //    position: (0, 0),
  //    padding: 2,
  //    state: ButtonState::Off,
  //    texture: None,
  //    ui
  //  }
  //}

  ///// Change the label
  //pub fn label(mut self, label: Label<'ctx>) -> Self {
  //  self.label = label;
  //  self.texture = None;
  //  self
  //}

  ///// Change the label text
  //pub fn text(mut self, t: &str) -> Self {
  //  self.label =
  //    self
  //    .label
  //    .text(t);
  //  self.texture = None;
  //  self
  //}

  ///// Change the label color
  //pub fn text_color(mut self, c: Color) -> Self {
  //  self.label =
  //    self
  //    .label
  //    .color(c);
  //  self.texture = None;
  //  self
  //}

  ///// Change the position
  //pub fn position(mut self, p: (i32, i32)) -> Self {
  //  self.position = p;
  //  self
  //}

  //fn draw_background(
  //  &self,
  //  label_width: u32,
  //  label_height: u32
  //) {
  //  self
  //    .ui
  //    .canvas
  //    .set_draw_color(Color::RGB(0x33, 0x33, 0x33));
  //  self
  //    .ui
  //    .canvas
  //    .fill_rect(
  //      Rect::new(
  //        self.padding as i32, self.padding as i32,
  //        label_width + self.padding * 2, label_height + self.padding * 2
  //      )
  //    )
  //    .unwrap();
  //}

  //fn draw_foreground(
  //  &self,
  //  canvas: &mut WindowCanvas,
  //  width: u32,
  //  height: u32,
  //  state: &ButtonState
  //) {
  //  canvas
  //    .set_draw_color(Color::RGB(255, 255, 255));
  //  let foreground_rect =
  //    match state {
  //      ButtonState::Down => {
  //        Rect::new(
  //          self.padding as i32, self.padding as i32,
  //          width - self.padding, height - self.padding
  //        )
  //      }
  //      _ => {
  //        Rect::new(
  //          0, 0,
  //          width - self.padding, height - self.padding
  //        )
  //      }
  //    };
  //  canvas
  //    .fill_rect(foreground_rect)
  //    .unwrap();
  //}

  //pub fn get_cached(
  //  &mut self,
  //) -> (&Texture<'ctx>, u32, u32) {
  //  let (tex, w, h) =
  //    self
  //    .texture
  //    .take()
  //    .unwrap_or({
  //      let (label_w, label_h) =
  //        self
  //        .label
  //        .get_size();
  //      let total_w =
  //        label_w + self.padding * 3;
  //      let total_h =
  //        label_h + self.padding * 3;
  //      let mut tex =
  //        self
  //        .ui
  //        .tex_creator
  //        .create_texture(
  //          None,
  //          TextureAccess::Target,
  //          total_w,
  //          total_h
  //        )
  //        .expect("Could not create texture cache for button");

  //      tex
  //        .set_blend_mode(BlendMode::Blend);
  //      self
  //        .ui
  //        .canvas
  //        .with_texture_canvas(&mut tex, |sub_canvas: &mut WindowCanvas| {
  //          self
  //            .draw_background(label_w, label_h);
  //          self
  //            .draw_foreground(sub_canvas, total_w, total_h, &self.state);
  //          self
  //            .label
  //            .draw(sub_canvas)
  //        })
  //        .expect("Could not cache button into texture");

  //      (tex, total_w, total_h)
  //    });
  //  self.texture =
  //    Some((tex, w, h));
  //  let (ref tex_ref, w, h) =
  //    self
  //    .texture
  //    .as_ref()
  //    .unwrap();
  //  (tex_ref, *w, *h)
  //}

  //pub fn get_size(&mut self, canvas: &mut WindowCanvas) -> (u32, u32) {
  //  let (_, w, h) =
  //    self
  //    .get_cached(canvas);
  //  (w, h)
  //}

  //pub fn draw(&mut self, canvas: &mut WindowCanvas) {
  //  let (x, y) =
  //    self
  //    .position;

  //  let (tex, w, h) =
  //    self
  //    .get_cached(canvas);

  //  let dest =
  //    Rect::new(
  //      x, y,
  //      w, h
  //    );

  //  canvas
  //    .copy(
  //      &tex,
  //      None,
  //      Some(dest)
  //    )
  //    .unwrap();
  //}

  ///// Update the button with the new state of the mouse. Returns the new button
  ///// state if it has changed.
  //pub fn update(
  //  &mut self,
  //  event: MouseUpdate,
  //  canvas: &mut WindowCanvas
  //) -> Option<ButtonState> {
  //  let (x, y) =
  //    self
  //    .position;
  //  let (w, h) =
  //    self
  //    .get_size(canvas);
  //  let is_inside =
  //    event.x >= x
  //    && event.x <= (x + w as i32)
  //    && event.y >= y
  //    && event.y <= (y + h as i32);

  //  let new_state:ButtonState =
  //    match (&self.state, is_inside, event.left_is_down) {
  //      (ButtonState::Off,  true, false) => { ButtonState::Over }
  //      (ButtonState::Off,     _,     _) => { self.state.clone() }
  //      (               _,  true,  true) => { ButtonState::Down }
  //      (               _, false,     _) => { ButtonState::Off }
  //      (               _,     _, false) => { ButtonState::Over }

  //    };

  //  let off_color =
  //    Color::RGB(0x33, 0x33, 0x33);
  //  let over_color =
  //    Color::RGB(0xf0, 0x0e, 0x0e);
  //  let down_color =
  //    Color::RGB(0xf0, 0x33, 0x33);

  //  let may_changed_state =
  //    if new_state != self.state {
  //      Some(new_state.clone())
  //    } else {
  //      None
  //    };

  //  if may_changed_state.is_some() {
  //    match new_state {
  //      ButtonState::Over => {
  //        self
  //          .label
  //          .set_color(over_color);
  //        self
  //          .label
  //          .set_position((self.padding as i32, self.padding as i32));
  //      }
  //      ButtonState::Off => {
  //        self
  //          .label
  //          .set_color(off_color);
  //        self
  //          .label
  //          .set_position((self.padding as i32, self.padding as i32));
  //      }
  //      ButtonState::Down => {
  //        self
  //          .label
  //          .set_color(down_color);
  //        self
  //          .label
  //          .set_position((self.padding as i32 * 2, self.padding as i32 * 2));
  //      }
  //    }
  //    self.texture = None;
  //  }

  //  self.state = new_state;

  //  may_changed_state
  //}
}

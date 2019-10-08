extern crate sdl2;

use sdl2::Sdl;
//use sdl2::event::Event;
//use sdl2::keyboard::{Keycode, Mod};
use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::render::{BlendMode, WindowCanvas};
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};

mod text;

/// Create a new context (window), set the title, size, etc.
pub fn new_ctx(
  title: &str,
  (ww, wh): (u32, u32)
) -> (Sdl, WindowCanvas, TextureCreator<WindowContext>, Sdl2TtfContext) {
  let ctx =
    sdl2::init()
    .expect("Could not create sdl2 context.");
  let vsys =
    ctx
    .video()
    .expect("Could not init video system.");
  let window =
    vsys
    .window(title, ww, wh)
    .position_centered()
    .resizable()
    .build()
    .expect("Could not create a window.");
  let mut canvas =
    window
    .into_canvas()
    .build()
    .expect("Could not create a canvas.");
  canvas
    .set_blend_mode(BlendMode::Blend);
  canvas
    .set_draw_color(Color::RGB(0, 0, 0));
  canvas
    .clear();
  canvas
    .present();
  let texture_creator =
    canvas
    .texture_creator();
  let ttf_ctx =
    sdl2::ttf::init()
    .unwrap();
  (ctx, canvas, texture_creator, ttf_ctx)
}


struct Label<'ctx> {
  text: String,
  font: &'ctx Font<'ctx, 'ctx>,
  texture: Option<(Texture<'ctx>, u32, u32)>,
  tex_creator: &'ctx TextureCreator<WindowContext>
}


impl<'ctx> Label<'ctx> {
  pub fn new(
    s: &str,
    font: &'ctx Font<'ctx, 'ctx>,
    tex_creator: &'ctx TextureCreator<WindowContext>
  ) -> Label<'ctx> {
    Label {
      text: s.to_string(),
      font,
      texture: None,
      tex_creator
    }
  }

  pub fn draw(&mut self, canvas: &mut WindowCanvas) {
    let (tex, w, h) =
      self
      .texture
      .take()
      .unwrap_or(
        text::cache(self.text.as_str(), Color::RGB(255, 255, 255), self.font, self.tex_creator)
      );

    let tex_rect =
      Rect::new(
        0, 0,
        w, h
      );

    canvas
      .copy(
        &tex,
        None,
        Some(tex_rect)
      )
      .unwrap();

    self.texture =
      Some((tex, w, h));
  }

}


fn main() {
  let (ctx, mut canvas, texture_creator, ttf_ctx) =
    new_ctx("berry playground", (800, 600));

  let komika =
    ttf_ctx
    .load_font("komika.ttf", 16)
    .unwrap();

  let mut event_pump =
    ctx
    .event_pump()
    .expect("Could not pump events.");

  let mut hello_label:Label =
    Label::new("Hello", &komika, &texture_creator);

  'mainloop: loop {
    let _event =
      event_pump
      .wait_event();

    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    hello_label.draw(&mut canvas);
    canvas.present();
  }
}

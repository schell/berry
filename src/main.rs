extern crate sdl2;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::render::{BlendMode, WindowCanvas};
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureAccess, TextureCreator};
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


/// A label for displaying some bit of text on the screen.
struct Label<'ctx> {
  text: String,
  color: Color,
  position: (i32, i32),
  font: &'ctx Font<'ctx, 'ctx>,
  texture: Option<(Texture<'ctx>, u32, u32)>,
  tex_creator: &'ctx TextureCreator<WindowContext>
}


impl<'ctx> Label<'ctx> {
  pub fn new(
    s: &str,
    color: Color,
    font: &'ctx Font<'ctx, 'ctx>,
    tex_creator: &'ctx TextureCreator<WindowContext>
  ) -> Label<'ctx> {
    Label {
      text: s.to_string(),
      color,
      position: (0, 0),
      font,
      texture: None,
      tex_creator
    }
  }

  /// Return a reference to the label's cached texture, along with the width and
  /// height. If the label has not been cached, cache it first.
  pub fn get_cached(&mut self) -> (&Texture<'ctx>, u32, u32) {
    // Render our cached texture if need be. After this we *know* self owns a
    // texture.
    self.texture =
      Some(
        self
        .texture
        .take()
        .unwrap_or(
          text::cache(
            self.text.as_str(),
            self.color,
            self.font, self.tex_creator
          )
        )
      );
    let (ref tex, w, h) =
      self
      .texture
      .as_ref()
      .unwrap();
    (tex, *w, *h)
  }

  pub fn draw(&mut self, canvas: &mut WindowCanvas) {
    let (x, y) =
      self.position;

    let (tex, w, h) =
      self
      .get_cached();

    let tex_rect =
      Rect::new(
        x, y,
        w, h
      );

    canvas
      .copy(
        &tex,
        None,
        Some(tex_rect)
      )
      .unwrap();
  }

  /// Returns the rendered size of the text. If the label has not been cached
  /// this causes it to be cached.
  pub fn get_size(&mut self) -> (u32, u32) {
    let (_, w, h) =
      self
      .get_cached();
    (w, h)
  }
}


/// The on/over/off state of a button.
#[derive(Clone, Debug, PartialEq)]
pub enum ButtonState {
  Off,
  Over,
  Down
}


/// A button - for clicking
struct Button<'ctx> {
  label: Label<'ctx>,
  pub position: (i32, i32),
  pub padding: u32,
  pub state: ButtonState,
  texture: Option<(Texture<'ctx>, u32, u32)>,
  tex_creator: &'ctx TextureCreator<WindowContext>
}


impl<'ctx> Button<'ctx> {

  pub fn new(
    text: &str,
    font: &'ctx Font<'ctx, 'ctx>,
    tex_creator: &'ctx TextureCreator<WindowContext>
  ) -> Button<'ctx> {

    let mut label =
      Label::new(
        text,
        Color::RGB(0,0,0),
        font,
        tex_creator
      );
    label.position = (2, 2);

    Button {
      label,
      position: (0, 0),
      padding: 2,
      state: ButtonState::Off,
      texture: None,
      tex_creator
    }
  }

  fn draw_background(
    &self,
    canvas: &mut WindowCanvas,
    label_width: u32,
    label_height: u32
  ) {
    canvas
      .set_draw_color(Color::RGB(0x33, 0x33, 0x33));
    canvas
      .fill_rect(
        Rect::new(
          self.padding as i32, self.padding as i32,
          label_width + self.padding * 2, label_height + self.padding * 2
        )
      )
      .unwrap();
  }

  fn draw_foreground(
    &self,
    canvas: &mut WindowCanvas,
    width: u32,
    height: u32,
    state: &ButtonState
  ) {
    canvas
      .set_draw_color(Color::RGB(255, 255, 255));
    let foreground_rect =
      match state {
        ButtonState::Down => {
          Rect::new(
            self.padding as i32, self.padding as i32,
            width - self.padding, height - self.padding
          )
        }
        _ => {
          Rect::new(
            0, 0,
            width - self.padding, height - self.padding
          )
        }
      };
    canvas
      .fill_rect(foreground_rect)
      .unwrap();
  }

  pub fn get_cached(
    &mut self,
    canvas: &mut WindowCanvas
  ) -> (&Texture<'ctx>, u32, u32) {
    let (tex, w, h) =
      self
      .texture
      .take()
      .unwrap_or({
        let (label_w, label_h) =
          self
          .label
          .get_size();
        let total_w =
          label_w + self.padding * 3;
        let total_h =
          label_h + self.padding * 3;
        let mut tex =
          self
          .tex_creator
          .create_texture(
            None,
            TextureAccess::Target,
            total_w,
            total_h
          )
          .expect("Could not create texture cache for button");

        tex
          .set_blend_mode(BlendMode::Blend);

        canvas
          .with_texture_canvas(&mut tex, |sub_canvas: &mut WindowCanvas| {
            self
              .draw_background(sub_canvas, label_w, label_h);
            self
              .draw_foreground(sub_canvas, total_w, total_h, &self.state);
            self
              .label
              .draw(sub_canvas)
          })
          .expect("Could not cache button into texture");

        (tex, total_w, total_h)
      });
    self.texture =
      Some((tex, w, h));
    let (ref tex_ref, w, h) =
      self
      .texture
      .as_ref()
      .unwrap();
    (tex_ref, *w, *h)
  }

  pub fn get_size(&mut self, canvas: &mut WindowCanvas) -> (u32, u32) {
    let (_, w, h) =
      self
      .get_cached(canvas);
    (w, h)
  }

  pub fn draw(&mut self, canvas: &mut WindowCanvas) {
    let (x, y) =
      self
      .position;

    let (tex, w, h) =
      self
      .get_cached(canvas);

    let dest =
      Rect::new(
        x, y,
        w, h
      );

    canvas
      .copy(
        &tex,
        None,
        Some(dest)
      )
      .unwrap();
  }

  pub fn update(&mut self, event: &Event, canvas: &mut WindowCanvas) {
    let may_response:Option<(i32, i32, bool)> =
      match event {
        Event::MouseMotion { x, y, mousestate, ..} => {
          let is_down =
            mousestate
            .left();
          Some ((*x, *y, is_down))
        }
        Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, ..} => {
          Some((*x, *y, true))
        }
        Event::MouseButtonUp { x, y, mouse_btn: MouseButton::Left, ..} => {
          Some((*x, *y, false))
        }
        _ => {
          None
        }
      };

    may_response
      .map(|(mx, my, is_down)| {
        let (x, y) =
          self
          .position;
        let (w, h) =
          self
          .get_size(canvas);
        let is_inside =
          mx >= x
          && mx <= (x + w as i32)
          && my >= y
          && my <= (y + h as i32);

        println!("is_inside: {:?} is_down: {:?} state: {:?}", is_inside, is_down, self.state);
        let new_state:ButtonState =
          match (&self.state, is_inside, is_down) {
            (ButtonState::Off,  true, false) => { ButtonState::Over }
            (ButtonState::Off,     _,     _) => { self.state.clone() }
            (               _,  true,  true) => { ButtonState::Down }
            (               _, false,     _) => { ButtonState::Off }
            (               _,     _, false) => { ButtonState::Over }

          };

        let off_color =
          Color::RGB(0x33, 0x33, 0x33);
        let over_color =
          Color::RGB(0xf0, 0x0e, 0x0e);
        let down_color =
          Color::RGB(0xf0, 0x33, 0x33);
        if new_state != self.state {
          match new_state {
            ButtonState::Over => {
              self.label.color = over_color;
              self.label.position = (self.padding as i32, self.padding as i32);
              self.label.texture = None;
              self.texture = None;
            }
            ButtonState::Off => {
              self.label.color = off_color;
              self.label.position = (self.padding as i32, self.padding as i32);
              self.label.texture = None;
              self.texture = None;
            }
            ButtonState::Down => {
              self.label.color = down_color;
              self.label.position = (self.padding as i32 * 2, self.padding as i32 * 2);
              self.label.texture = None;
              self.texture = None;
            }
          }
        }

        self.state = new_state;
      });
  }
}


pub fn with_viewport<F>(
  vp: Option<Rect>,
  canvas: &mut WindowCanvas,
  f: F
)
where
  F: FnOnce(&mut WindowCanvas)
{
  let prev:Rect =
    canvas.viewport();
  canvas
    .set_viewport(vp);
  f(canvas);
  canvas
    .set_viewport(prev);
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
    Label::new(
      "Hello",
      Color::RGB(255, 255, 255),
      &komika,
      &texture_creator
    );

  let label_width =
    hello_label
    .get_size()
    .0;

  let mut button:Button =
    Button::new("Press me", &komika, &texture_creator);
  button.position = (label_width as i32, 0);

  'mainloop: loop {
    let event =
      event_pump
      .wait_event();

    match event {
      Event::Quit {..} => {
        break 'mainloop;
      }

      Event::KeyDown { keycode: Some(Keycode::Q), keymod, ..} => {
        if keymod.contains(Mod::LCTRLMOD)
          || keymod.contains(Mod::RCTRLMOD) {
            break 'mainloop;
          }
      }
      _ => {}
    }

    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.clear();

    hello_label
      .draw(&mut canvas);

    button
      .update(&event, &mut canvas);
    button
      .draw(&mut canvas);
    canvas.present();
  }
}

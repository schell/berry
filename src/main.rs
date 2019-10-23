extern crate sdl2;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use sdl2::Sdl;
use sdl2::video::WindowContext;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, TextureCreator, WindowCanvas};

use cassowary::*;

pub mod components;
pub mod systems;
pub mod picture;
pub mod rasterizer;
pub mod ui;

use components::*;
use rasterizer::*;
use ui::*;
use picture::Picture;


/// Used to update the button from whatever owns the button.
#[derive(Debug, Clone, PartialEq)]
pub struct MouseUpdate {
  /// The mouse x in global coordinates
  pub x: i32,
  /// The mouse y in global coordinates
  pub y: i32,

  pub left_is_down: bool,

  pub middle_is_down: bool,

  pub right_is_down: bool,
}


/// Updates that are given unto widgets from their owners.
#[derive(Debug, Clone, PartialEq)]
pub enum Update {
  Mouse(MouseUpdate),
  Quit
}

/// Create a new context (window), set the title, size, etc.
pub fn new_contexts(
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
  let tex_creator =
    canvas
    .texture_creator();
  let ttf =
    sdl2::ttf::init()
    .unwrap();
  (ctx, canvas, tex_creator, ttf)
}
//
//   pub fn get_font<'ctx, P: AsRef<Path>>(
//     ttf: &'ctx Sdl2TtfContext,
//     path: P,
//     point_size: u16,
//     fonts: &'ctx mut FontMap<'ctx>
//   ) -> Result<&'ctx Font<'ctx, 'static>, String> {
//     let font_path:String =
//       path
//       .as_ref()
//       .to_str()
//       .unwrap()
//       .to_string();
//     let key =
//       (font_path, point_size);
//
//     let already_have_font =
//       fonts
//       .contains_key(&key);
//     if !already_have_font {
//       let font =
//         ttf
//         .load_font(path, point_size)?;
//       fonts
//         .insert(key.clone(), font);
//     }
//
//     fonts
//       .get(&key)
//       .ok_or("This is impossible - I know this font is here".to_string())
//   }
//
//   pub fn wait_event_timeout(&mut self, timeout: u32) -> Option<Update> {
//     let mut event_pump =
//       self
//       .may_event_pump
//       .take()
//       .unwrap_or(
//         self
//           .sdl
//           .event_pump()
//           .expect("Could not pump events!")
//       );
//     let event =
//       event_pump
//       .wait_event_timeout(timeout)?;
//     mk_update(&event)
//   }
//
//   pub fn draw<W: Widget>(&mut self, w: &W) {
//
//   }
// }


/// Maps sdl2 events into "updates"
pub fn mk_update(event: &Event) -> Option<Update> {
  match event {
    Event::MouseMotion { x, y, mousestate, ..} => {
      Some (
        Update::Mouse(
          MouseUpdate {
            x: *x,
            y: *y,
            left_is_down: mousestate.left(),
            middle_is_down: mousestate.middle(),
            right_is_down: mousestate.right()
          }
        )
      )
    }
    Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, ..} => {
      Some (
        Update::Mouse(
          MouseUpdate {
            x: *x,
            y: *y,
            left_is_down: true,
            middle_is_down: false,
            right_is_down: false
          }
        )
      )
    }
    Event::MouseButtonUp { x, y, mouse_btn: MouseButton::Left, ..} => {
      Some(
        Update::Mouse(
          MouseUpdate {
            x: *x,
            y: *y,
            left_is_down: false,
            middle_is_down: false,
            right_is_down: false
          }
        )
      )
    }
    Event::Quit {..} => {
      Some(Update::Quit)
    }
    Event::KeyDown { keycode: Some(Keycode::Q), keymod, ..} => {
      let ctrl_is_down =
        keymod.contains(Mod::LCTRLMOD)
        || keymod.contains(Mod::RCTRLMOD);
      if ctrl_is_down {
        Some(Update::Quit)
      } else {
        None
      }
    }
    _ => {
      None
    }
  }
}

pub struct WindowSize {
  pub width: u32,
  pub height: u32
}


impl Default for WindowSize {
  fn default() -> WindowSize {
    WindowSize {
      width: 0,
      height: 0
    }
  }
}

fn main() {
  let (sdl, mut canvas, tex_creator, ttf) =
    new_contexts("berry playground", (800, 600));

  let mut rasterizer =
    Rasterizer::new(&mut canvas, &tex_creator, &ttf);

  let mut ui = UI::new();

  let pic_def =
    Picture::new()
    .set_color(255, 255, 0, 255)
    .fill_rect(0, 0, 100, 100)
    .set_color(255, 0, 255, 255)
    .fill_rect(50, 50, 100, 100);

  let (_, pw, ph) =
    rasterizer
    .get_picture(&pic_def);

  let pic =
    EntityBuilder::new()
    .name("pic")
    .picture(&pic_def)
    .left(100)
    .top(100)
    .width(pw)
    .height(ph)
    .build(&mut ui);

  assert!(ui.get::<Name>(pic).is_some());

  let text_def =
    Text::new("<- Look at this thing to the left!")
    .color(0, 0, 0, 255);

  let (_, lw, lh) =
    rasterizer
    .get_text(&text_def);

  let label =
    EntityBuilder::new()
    .name("label")
    .text(&text_def)
    .left(pic.right())
    .top(pic.bottom() - 10.0)
    .width(lw)
    .height(lh)
    .build(&mut ui);

  ui.maintain(&mut rasterizer);

  let pic_pos =
    ui
    .get_position(pic)
    .expect("pic has no position");
  assert_eq!(100, pic_pos.0, "pic.x is not 100");
  assert_eq!(100, pic_pos.1, "pic.y is not 100");

  let pic_size =
    ui
    .get_size(pic)
    .unwrap();
  assert_eq!(pw, pic_size.0, "pic.width is not 300");
  assert_eq!(ph, pic_size.1, "pic.height is not 150");

  let label_pos =
    ui
    .get_position(label)
    .unwrap();
  assert_eq!(pic_pos.0 + pic_size.0 as i32, label_pos.0, "label's left doesn't match pic's right");

  let corner_square_pic =
    Picture::new()
    .set_color(0, 0, 0, 255)
    .fill_rect(0, 0, 25, 25);

  let _ =
    rasterizer
    .get_picture(&corner_square_pic);

  let _corner_square =
    EntityBuilder::new()
    .name("corner_square")
    .picture(&corner_square_pic)
    .width(25)
    .height(25)
    .right(ui.stage().right())
    .bottom(ui.stage().bottom())
    .build(&mut ui);

  ui.maintain(&mut rasterizer);

  let box1 =
    EntityBuilder::new()
    .name("box1")
    .picture(
      &Picture::new()
        .set_color(255, 0, 0, 128)
        .fill_rect(0, 0, 50, 100)
    )
    .build(&mut ui);

  let box2 =
    EntityBuilder::new()
    .name("box1")
    .picture(
      &Picture::new()
        .set_color(255, 0, 0, 128)
        .fill_rect(0, 0, 50, 100)
    )
    .build(&mut ui);

  let _box_relation =
    EntityBuilder::new()
    .x_constraints(
      vec![
        box1.left().is(0),
        box2.right().is(
          ui.stage().right()
        ),
        box2.left().is_ge(box1.left()),

        box1.width().is(50.0).with_strength(strength::WEAK),
        box2.width().is(100.0).with_strength(strength::WEAK)
      ]
    )
    .y_constraints(
      vec![
        box1.height().is(100),
        box2.height().is(100)
      ]
    )
    .build(&mut ui);

  let mut event_pump =
    sdl
    .event_pump()
    .unwrap();

  'mainloop: loop {
    let may_update:Option<Update> =
      event_pump
      .wait_event_timeout(1000/12)
      .map(|event| mk_update(&event))
      .unwrap_or(None);

    if may_update == Some(Update::Quit) {
      break 'mainloop;
    }

    ui
      .maintain(&mut rasterizer);
  }
}

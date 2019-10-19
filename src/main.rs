extern crate sdl2;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, TextureCreator, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;

use specs::prelude::{Dispatcher, DispatcherBuilder, World, WorldExt};

pub mod components;
pub mod systems;
pub mod picture;

use systems::drawing::*;
use systems::layout::*;
use components::*;


//pub struct UI {
//  pub sdl: Sdl,
//  pub canvas: WindowCanvas,
//  pub tex_creator: TextureCreator<WindowContext>,
//  pub ttf: Sdl2TtfContext,
//  may_event_pump: Option<EventPump>
//}




//pub trait Widget {
//  fn draw<'ctx>(
//    &self,
//    ui: &'ctx mut UI,
//    fonts: &'ctx mut FontMap<'ctx>
//  ) -> (Texture<'ctx>, u32, u32);
//}


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


pub struct UI<'a, 'b> {
  world: World,
  dispatcher: Dispatcher<'a, 'b>
}


pub fn dispatcher_sdl2<'a, 'b>(
  canvas: &'b mut WindowCanvas,
  tex_creator: &'b TextureCreator<WindowContext>,
  ttf: &'b Sdl2TtfContext
) -> Dispatcher<'a, 'b> {
  DispatcherBuilder::new()
    .with_thread_local(DrawingSystem::new(
      canvas,
      tex_creator,
      ttf
    ))
    .with_thread_local(LayoutSystem::new())
    .build()
}


impl<'a, 'b> UI<'a, 'b> {
  pub fn new<'c, 'd>(dispatcher: Dispatcher<'c, 'd>) -> UI<'c, 'd> {
    let mut world
      = World::new();

    let mut dispatcher =
      dispatcher;
    dispatcher
      .setup(&mut world);

    UI {
      world,
      dispatcher
    }
  }

  pub fn maintain(&mut self) {
    self
      .dispatcher
      .dispatch(&mut self.world);
    self
      .world
      .maintain()
  }
}


fn main() {
  let (sdl, mut canvas, tex_creator, ttf) =
    new_contexts("berry playground", (800, 600));

  let mut ui =
    UI::new(dispatcher_sdl2(&mut canvas, &tex_creator, &ttf));

  let pic =
    EntityBuilder::new()
    .name("pic")
    .picture(
      vec![
        picture::set_color(255, 255, 0, 255),
        picture::fill_rect(0, 0, 100, 100),
        picture::set_color(255, 0, 255, 255),
        picture::fill_rect(50, 50, 100, 100),
      ]
    )
    .left(100)
    .top(100)
    .width(300)
    .height(150)
    .build(&ui);

  let _label =
    EntityBuilder::new()
    .name("label")
    .text("<- Look at this thing to the left!")
    .color(0, 0, 0, 255)
    .left(pic.right())
    .top(200)
    .build(&ui);

  ui
    .maintain();

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
      .maintain();
  }
}

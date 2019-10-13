use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::pixels;
use specs::prelude::*;
use std::collections::HashMap;

use super::super::components::*;

mod text;

type FontMap<'ctx> = HashMap<(String, u16), Font<'ctx, 'static>>;

type TextureMap<'ctx> = HashMap<String, Texture<'ctx>>;


pub struct DrawingSystem<'ctx> {
  fonts: FontMap<'ctx>,
  textures: TextureMap<'ctx>,
  _canvas: Option<&'ctx mut WindowCanvas>,
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
      _canvas: Some(canvas),
      tex_creator: Some(tex_creator),
      ttf: Some(ttf)
    }
  }
}


impl<'a, 'ctx> System<'a> for DrawingSystem<'ctx> {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Text>,
    ReadStorage<'a, WidgetType>,
  );

  fn run(&mut self, (entities, _positions, texts, _wtypes): Self::SystemData) {
    // First run through everything that has text and texturize it
    for (_ent, text) in (&entities, &texts).join() {
      let tex_key =
        format!("{:?}", text);
      let has_texture =
        self
        .textures
        .contains_key(&tex_key);

      if !has_texture {
        let font_key =
          (text.font_path.clone(), text.font_size);
        let has_font =
          self
          .fonts
          .contains_key(&font_key);

        if !has_font {
          let font =
            self
            .ttf
            .expect("DrawingSystem has no ttf contexnt to load fonts with.")
            .load_font(&text.font_path, text.font_size)
            .expect(
              &format!("Could not load font: {:?}", font_key)
            );
          self
            .fonts
            .insert(font_key.clone(), font);
        }

        let font =
          self
          .fonts
          .get(&font_key)
          .expect("Impossible missing font.");

        let color =
          pixels::Color::RGBA(
            text.text_color.r,
            text.text_color.g,
            text.text_color.b,
            text.text_color.a
          );

        let tex_creator =
          self
          .tex_creator
          .take()
          .expect("DrawingSystem does not have a TextureCreator.");

        let (tex, _, _) =
          text::cache(
            &text.text,
            color,
            font,
            &tex_creator
          );

        self
          .textures
          .insert(tex_key.clone(), tex);

        self.tex_creator =
          Some(tex_creator);
      }
    }
  }
}

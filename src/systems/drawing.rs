use sdl2::render::{BlendMode, Texture, TextureAccess, TextureCreator, TextureQuery, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::pixels;
use sdl2::rect::Rect;
use specs::prelude::*;
use std::collections::HashMap;

use super::super::components::*;

mod text;

type FontMap<'ctx> = HashMap<(String, u16), Font<'ctx, 'static>>;

type TextCache<'ctx> = HashMap<Text, Texture<'ctx>>;

type WidgetCache<'ctx> = HashMap<u32, Texture<'ctx>>;


pub struct DrawingSystem<'ctx> {
  fonts: FontMap<'ctx>,
  text_cache: TextCache<'ctx>,
  widget_cache: WidgetCache<'ctx>,
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
      text_cache: HashMap::new(),
      widget_cache: HashMap::new(),
      canvas: Some(canvas),
      tex_creator: Some(tex_creator),
      ttf: Some(ttf)
    }
  }
}


impl<'ctx> DrawingSystem<'ctx> {
  fn size_for_widget(&self, texts: &ReadStorage<Text>, ent: &Entity, wtype: &WidgetType) -> (u32, u32) {
    match wtype {
      WidgetType::Label => {
        // it's just the size of its text
        let text =
          texts
          .get(*ent)
          .expect("Label does not have text");
        let tex =
          self
          .text_cache
          .get(text)
          .expect("Label's text has not been cached!");
        let TextureQuery{ width, height, ..} =
          tex
          .query();
        (width, height)
      }
    }
  }
}


impl<'a, 'ctx> System<'a> for DrawingSystem<'ctx> {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, Position>,
    WriteStorage<'a, Size>,
    ReadStorage<'a, Text>,
    ReadStorage<'a, WidgetType>,
  );

  fn run(&mut self, (entities, positions, mut sizes, texts, wtypes): Self::SystemData) {
    let tex_creator =
      self
      .tex_creator
      .take()
      .expect("DrawingSystem does not have a TextureCreator.");

    // First run through everything that has text and texturize it
    for text in (&texts).join() {
      let has_texture =
        self
        .text_cache
        .contains_key(&text);

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

        let (tex, _, _) =
          text::cache(
            &text.text,
            color,
            font,
            &tex_creator
          );

        self
          .text_cache
          .insert((*text).clone(), tex);
      }
    }

    // Now start drawing the screen
    let canvas =
      self
      .canvas
      .take()
      .expect("DrawingSystem does not have a WindowCanvas");

    canvas
      .set_draw_color(pixels::Color::RGB(128, 128, 128));
    canvas
      .clear();

    // Run through each widget and cache if need be
    for (ent, wtype) in (&entities, &wtypes).join() {
      let is_cached =
        self
        .widget_cache
        .contains_key(&ent.id());
      // Get the size of the widget and make sure it's stored
      let (w, h) =
        self
        .size_for_widget(&texts, &ent, wtype);
      sizes
        .insert(
          ent,
          Size{
            width: w,
            height: h
          }
        )
        .unwrap();
      if !is_cached {
        println!("Cacheing widget {:?} {:?}", ent, wtype);
        let mut tex =
          tex_creator
          .create_texture(
            None,
            TextureAccess::Target,
            w,
            h
          )
          .expect("Could not create texture cache for widget");
        tex
          .set_blend_mode(BlendMode::Blend);
        match wtype {
          WidgetType::Label => {
            let text =
              texts
              .get(ent)
              .expect("Label does not have a Text");
            let label_tex =
              self
              .text_cache
              .get(text)
              .expect("Label text is not cached.");

            canvas
              .with_texture_canvas(&mut tex, |sub_canvas| {
                sub_canvas
                  .copy(
                    &label_tex,
                    None,
                    None
                  )
                  .unwrap();
              })
              .unwrap();
          }
        }
        self
          .widget_cache
          .insert(ent.id(), tex);
      }
    }

    // Run through each entity with a position and render it to the screen
    for (ent, position) in (&entities, &positions).join() {
      let cached_tex =
        self
        .widget_cache
        .get(&ent.id())
        .expect("Widget has not been cached!");
      let TextureQuery{ width, height, ..} =
        cached_tex
        .query();
      canvas
        .copy(
          &cached_tex,
          None,
          Some(Rect::new(
            position.x, position.y,
            width, height
          ))
        )
        .unwrap();
    }

    canvas
      .present();

    self.tex_creator =
      Some(tex_creator);
    self.canvas = Some(canvas);
  }
}

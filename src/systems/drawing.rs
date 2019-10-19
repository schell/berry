use sdl2::render::{BlendMode, Texture, TextureAccess, TextureCreator, TextureQuery, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::pixels;
use sdl2::rect::Rect;
use specs::prelude::*;
use std::collections::HashMap;

use super::super::components::*;
use super::super::picture::*;
use super::super::WindowSize;

mod text;

type FontMap<'ctx> = HashMap<(String, u16), Font<'ctx, 'static>>;

type TextCache<'ctx> = HashMap<Text, Texture<'ctx>>;

type PictureCache<'ctx> = HashMap<Picture, Texture<'ctx>>;


/// This system is responsible for
/// * rasterizing our graphics primitives and displaying them on the screen
/// * updating the window size for downstream systems
pub struct DrawingSystem<'ctx> {
  fonts: FontMap<'ctx>,
  text_cache: TextCache<'ctx>,
  picture_cache: PictureCache<'ctx>,
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
      picture_cache: HashMap::new(),
      canvas: Some(canvas),
      tex_creator: Some(tex_creator),
      ttf: Some(ttf)
    }
  }
}


impl<'ctx> DrawingSystem<'ctx> {
  fn rasterize_picture(picture: &Picture, canvas: &mut WindowCanvas) {
    picture
      .0
      .iter()
      .for_each(|cmd| {
        match *cmd {
          PictureCmd::SetColor(r,g,b,a) => {
            canvas
              .set_draw_color(pixels::Color::RGBA(r,g,b,a));
          }
          PictureCmd::FillRect(x,y,w,h) => {
            canvas
              .fill_rect(Some(
                Rect::new(x as i32, y as i32, w, h)
              ))
              .unwrap();
          }
        }
      });
  }
}


impl<'a, 'ctx> System<'a> for DrawingSystem<'ctx> {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, ElementBox>,
    ReadStorage<'a, Name>,
    ReadStorage<'a, Picture>,
    ReadStorage<'a, Text>,
    Write<'a, WindowSize>
  );

  fn run(
    &mut self,
    (entities, element_boxes, names, pictures, texts, mut window_size): Self::SystemData
  ) {
    let tex_creator =
      self
      .tex_creator
      .take()
      .expect("DrawingSystem does not have a TextureCreator.");
    let canvas =
      self
      .canvas
      .take()
      .expect("DrawingSystem does not have a WindowCanvas");

    // Update the window size
    let (ww, wh) =
      canvas
      .logical_size();
    *window_size =
      WindowSize {
        width: ww,
        height: wh
      };

    // First run through all texts and texturize them
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

    // Then run through all our raster graphics
    for picture in (&pictures).join() {
      let has_picture =
        self
        .picture_cache
        .contains_key(&picture);

      if !has_picture {
        let (w, h) =
          picture
          .size();
        let mut tex =
          tex_creator
          .create_texture(
            None,
            TextureAccess::Target,
            w, h
          )
          .expect("Could not create texture cache for picture");
        tex
          .set_blend_mode(BlendMode::Blend);
        canvas
          .with_texture_canvas(&mut tex, |sub_canvas| {
            Self::rasterize_picture(&picture, sub_canvas);
          })
          .unwrap();

        self
          .picture_cache
          .insert(picture.clone(), tex);
      }
    }

    // Now start drawing the screen
    canvas
      .set_draw_color(pixels::Color::RGB(128, 128, 128));
    canvas
      .clear();

    // Run through each entity and render it to the screen
    for ent in (&entities).join() {
      let mut draw_tex = |tex: &Texture| {
        let may_name =
          names
          .get(ent);
        let may_el =
          element_boxes
          .get(ent);
        let x =
          may_el
          .map(|el| el.x)
          .unwrap_or(0);
        let y =
          may_el
          .map(|el| el.y)
          .unwrap_or(0);
        let TextureQuery{ width: tw, height: th, ..} =
          tex
          .query();
        let w =
          may_el
          .map(|el| el.w)
          .unwrap_or(0);
        let w =
          if w == 0 {
            tw
          } else {
            w
          };
        assert!(w != 0, format!("width of {:?} = {:?}", may_name, w));
        let h =
          may_el
          .map(|el| el.h)
          .unwrap_or(0);
        let h =
          if h == 0 {
            th
          } else {
            h
          };
        assert!(h != 0, format!("height of {:?} = {:?}", may_name, h));
        canvas
          .copy(
            tex,
            None,
            Some(Rect::new(x,y,w,h))
          )
          .unwrap();
      };

      // If this thing is a piece of text, draw that
      texts
        .get(ent)
        .map(|text| {
          let tex =
            self
            .text_cache
            .get(text)
            .expect("Text was not cached! This should be impossible");
          draw_tex(tex);
        });

      // If this thing is a rasterized picture, draw that
      pictures
        .get(ent)
        .map(|pic| {
          let tex =
            self
            .picture_cache
            .get(pic)
            .expect("Picture was not cached! This should be impossible");
          draw_tex(tex);
        });
    }

    canvas
      .present();

    self.tex_creator =
      Some(tex_creator);
    self.canvas = Some(canvas);
  }
}

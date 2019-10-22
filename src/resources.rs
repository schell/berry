use sdl2::video::WindowContext;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::{BlendMode, Texture, TextureAccess, TextureCreator, TextureQuery, WindowCanvas};
use std::collections::HashMap;

use super::components::*;
use super::picture::*;


pub type FontMap<'ctx> = HashMap<(String, u16), Font<'ctx, 'static>>;

pub type TextCache<'ctx> = HashMap<Text, Texture<'ctx>>;

pub type PictureCache<'ctx> = HashMap<Picture, Texture<'ctx>>;


pub struct Resources<'ctx> {
  pub fonts: FontMap<'ctx>,
  pub text_cache: TextCache<'ctx>,
  pub picture_cache: PictureCache<'ctx>,
  pub canvas: Option<&'ctx mut WindowCanvas>,
  pub tex_creator: Option<&'ctx TextureCreator<WindowContext>>,
  pub ttf: Option<&'ctx Sdl2TtfContext>
}


impl<'ctx> Resources<'ctx> {
  pub fn new(
    canvas: &'ctx mut WindowCanvas,
    tex_creator: &'ctx TextureCreator<WindowContext>,
    ttf: &'ctx Sdl2TtfContext
  ) -> Resources<'ctx> {
    Resources {
      fonts: HashMap::new(),
      text_cache: HashMap::new(),
      picture_cache: HashMap::new(),
      canvas: Some(canvas),
      tex_creator: Some(tex_creator),
      ttf: Some(ttf)
    }
  }

  /// Texturize a string of text in a font and color.
  pub fn rasterize_text(
    &self,
    s: &str,
    c: Color,
    font: &Font,
  ) -> (Texture<'ctx>, u32, u32) {
    // Generate the texture and copy the text into it
    let surface =
      font
      .render(s)
      .blended(c)
      .map_err(|e| e.to_string())
      .unwrap();
    let mut texture =
      self
      .tex_creator
      .expect("Resources does not have a tex_creator to rasterize text with")
      .create_texture_from_surface(&surface)
      .map_err(|e| e.to_string())
      .unwrap();
    texture
      .set_blend_mode(BlendMode::Blend);
    texture
      .set_alpha_mod(c.a);

    let TextureQuery{ width, height, ..} =
      texture
      .query();
    (texture, width, height)
  }


  /// Get the given text as a rasterized texture and its width and height.
  /// If the given text's font has not yet been loaded it will be cached.
  /// If the given text has not yet been rasterized it will be cached.
  pub fn get_text(&mut self, text: &Text) -> (&Texture<'ctx>, u32, u32) {
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
          Color::RGBA(
            text.text_color.r,
            text.text_color.g,
            text.text_color.b,
            text.text_color.a
          );

        let (tex, _, _) =
          self.rasterize_text(
            &text.text,
            color,
            font
          );

        self
          .text_cache
          .insert((*text).clone(), tex);
      }

    let tex =
      self
      .text_cache
      .get(&text)
      .expect("Could not get cached text");
    let TextureQuery{ width, height, ..} =
      tex.query();
    (tex, width, height)
  }

  fn rasterize_picture(picture: &Picture, canvas: &mut WindowCanvas) {
    picture
      .0
      .iter()
      .for_each(|cmd| {
        match *cmd {
          PictureCmd::SetColor(r,g,b,a) => {
            canvas
              .set_draw_color(Color::RGBA(r,g,b,a));
          }
          PictureCmd::FillRect(x,y,w,h) => {
            canvas
              .fill_rect(Some(
                rect::Rect::new(x as i32, y as i32, w, h)
              ))
              .unwrap();
          }
        }
      });
  }

  pub fn get_picture(&mut self, picture: &Picture) -> (&Texture<'ctx>, u32, u32) {
    let canvas =
      self
      .canvas
      .take()
      .expect("Could not get resource's canvas to rasterize a picture");

    let has_picture =
      self
      .picture_cache
      .contains_key(&picture);

    if !has_picture {
      let (w, h) =
        picture
        .size();
      let mut tex =
        self
        .tex_creator
        .expect("Resources does not have a tex_creator to rasterize a picture with")
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

    let tex =
      self
      .picture_cache
      .get(&picture)
      .expect("Could not get cached picture``");

    let TextureQuery{ width, height, ..} =
      tex.query();

    self.canvas =
      Some(canvas);

    (tex, width, height)
  }
}

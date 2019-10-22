use sdl2::render::{Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::pixels;
use specs::prelude::*;

use super::*;
use super::components::*;
use super::picture::*;
use super::resources::*;


pub type DrawingSystemData<'a> = (
  Entities<'a>,
  ReadStorage<'a, ElementBox>,
  ReadStorage<'a, Name>,
  ReadStorage<'a, Picture>,
  ReadStorage<'a, Text>,
  Write<'a, WindowSize>
);


pub fn run_sdl2_drawing<'a, 'ctx>(
  resources: &mut Resources<'ctx>,
  (entities, element_boxes, names, pictures, texts, mut _window_size): DrawingSystemData<'a>
) {
  let canvas =
    resources
    .canvas
    .take()
    .expect("Could not take resource's canvas for sdl2 drawing");

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
          resources
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
          resources
          .picture_cache
          .get(pic)
          .expect("Picture was not cached! This should be impossible");
        draw_tex(tex);
      });
  }

  canvas
    .present();

  resources.canvas =
    Some(canvas);
}

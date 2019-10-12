use sdl2::render::BlendMode;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;


/// Texturize a string of text in a font and color.
pub fn cache<'ctx>(
  s: &str,
  c: Color,
  font: &Font,
  tex_creator: &'ctx TextureCreator<WindowContext>
) -> (Texture<'ctx>, u32, u32) {
  // Generate the texture and copy the text into it
  let surface =
    font
    .render(s)
    .blended(c)
    .map_err(|e| e.to_string())
    .unwrap();
  let mut texture =
    tex_creator
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

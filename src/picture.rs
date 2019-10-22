use specs::prelude::{Component, VecStorage};


/// Primitive raster drawing commands.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PictureCmd {
  SetColor(u8, u8, u8, u8),
  FillRect(u32, u32, u32, u32),
}


/// A declarative way of drawing. A list of picture commands.
///
///```rust
/// Picture::new()
///   .set_color(Color { r: 255, g: 255, b: 0, a: 255})
///   .fill_rect(Rectangle::new(0, 0, 200, 100));
///```
#[derive(Debug, Component, Clone, Hash, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct Picture(pub Vec<PictureCmd>);


impl Picture {
  pub fn new() -> Picture {
    Picture(vec![])
  }

  pub fn size(&self) -> (u32, u32) {
    self
      .0
      .iter()
      .fold(
        (0, 0),
        |(max_w, max_h), cmd| {
          match cmd {
            PictureCmd::FillRect(x,y,w,h) => {
              (u32::max(max_w, x + w), u32::max(max_h, y + h))
            }
            _ => { (max_w, max_h) }
          }
        })
  }

  pub fn set_color(self, r:u8, g:u8, b:u8, a:u8) -> Self {
    let mut s = self;
    s.0.push(PictureCmd::SetColor(r,g,b,a));
    s
  }


  pub fn fill_rect(self, x: u32, y: u32, w:u32, h:u32) -> Self  {
    let mut s = self;
    s.0.push(PictureCmd::FillRect(x,y,w,h));
    s
  }
}

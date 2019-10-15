use specs::prelude::{
  Component,
  Builder,
  Entities,
  Entity,
  HashMapStorage,
  LazyUpdate,
  Read,
  VecStorage,
};

use super::UI;
use super::picture::{Picture, PictureCmd};


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
  pub x: i32,
  pub y: i32
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8
}

#[derive(Clone, Component, Debug, PartialEq, Hash, Eq)]
#[storage(VecStorage)]
pub struct Text {
  pub font_path: String,
  pub font_size: u16,
  pub text_color: Color,
  pub text: String
}


impl Text {
  pub fn new() -> Text {
    Text{
      font_path: "komika.ttf".to_string(),
      font_size: 16,
      text_color: Color{ r: 255, g: 255, b: 255, a: 255 },
      text: "Text".to_string()
    }
  }

  pub fn text(self, s: &str) -> Self {
    let mut text = self;
    text.text = s.to_string();
    text
  }
}


#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Name(String);


#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Container {
  pub items: Vec<Entity>
}


#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct ContainerItem {
  pub parent: Entity,
  pub offset: (i32, i32)
}


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub enum WidgetType {
  Label
}


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Size {
  pub width: u32,
  pub height: u32
}


pub struct TextBuilder {
  position: Position,
  text: Text,
  name: Option<Name>
}


impl TextBuilder {
  pub fn new() -> TextBuilder {
    TextBuilder {
      position: Position{ x: 0, y: 0 },
      text: Text::new(),
      name: None
    }
  }

  pub fn text(self, t: &str) -> Self {
    let mut label = self;
    label.text.text = t.to_string();
    label
  }

  pub fn color(self, r: u8, g: u8, b: u8, a:u8) -> Self {
    let mut label = self;
    label.text.text_color =
      Color {
        r, g, b, a
      };
    label
  }

  pub fn build(self, ui: &UI) -> Entity {
    let (entities, lazy): (Entities, Read<LazyUpdate>) =
      ui
      .world
      .system_data();

    let ent =
      lazy
      .create_entity(&entities)
      .with(self.position)
      .with(self.text)
      .build();

    self
      .name
      .map(|name| {
        lazy
          .insert(ent, name);
      });

    ent
  }
}


// TODO: Maybe kill all the container stuff because we have cassowary
pub struct ContainerBuilder {
  pub items: Vec<(Entity, (i32, i32))>,
  pub position: Position,
  pub name: Option<Name>
}


impl ContainerBuilder {
  pub fn new() -> ContainerBuilder {
    ContainerBuilder {
      items: vec![],
      position: Position{ x: 0, y: 0 },
      name: None
    }
  }

  pub fn with_item(self, ent: Entity, offset: (i32, i32)) -> ContainerBuilder {
    let mut c = self;
    c
      .items
      .push((ent, offset));
    c
  }

  pub fn build(self, ui: &UI) -> Entity {
    let (entities, lazy): (Entities, Read<LazyUpdate>) =
      ui
      .world
      .system_data();

    let ent =
      lazy
      .create_entity(&entities)
      .with(self.position)
      .with(
        Container{
          items:
          self
            .items
            .clone()
            .into_iter()
            .map(|(child, _)| child)
            .collect()
        }
      )
      .build();

    self
      .name
      .map(|name| {
        lazy
          .insert(ent, name)
      });

    self
      .items
      .iter()
      .for_each(|(child, offset)| {
        lazy
          .insert(
            *child,
            ContainerItem {
              parent: ent,
              offset: *offset
            }
          )
      });

    ent
  }
}


pub struct PictureBuilder {
  picture: Picture,
  position: Position,
  name: Option<Name>
}


impl PictureBuilder {
  pub fn new() -> PictureBuilder {
    PictureBuilder {
      picture: Picture::new(),
      position: Position{ x: 0, y: 0 },
      name: None
    }
  }

  pub fn picture(self, pic: Picture) -> Self {
    let mut pb = self;
    pb.picture = pic;
    pb
  }

  pub fn set_color(self, r: u8, g: u8, b: u8, a:u8) -> PictureBuilder {
    let mut pb = self;
    pb
      .picture
      .0
      .push(PictureCmd::SetColor(r,g,b,a));
    pb
  }

  pub fn fill_rect(self, x: u32, y: u32, width: u32, height: u32) -> PictureBuilder {
    let mut pb = self;
    pb
      .picture
      .0
      .push(PictureCmd::FillRect(x,y,width,height));
    pb
  }

  pub fn position(self, x: i32, y: i32) -> Self {
    let mut pb = self;
    pb.position = Position{ x, y };
    pb
  }

  pub fn build(self, ui: &UI) -> Entity {
    let (entities, lazy): (Entities, Read<LazyUpdate>) =
      ui
      .world
      .system_data();

    let ent =
      lazy
      .create_entity(&entities)
      .with(self.position)
      .with(self.picture)
      .build();

    self
      .name
      .map(|name| {
        lazy
          .insert(ent, name)
      });

    ent
  }
}

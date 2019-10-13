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
#[storage(VecStorage)]
pub enum WidgetType {
  Label
}


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Size {
  width: u32,
  height: u32
}


pub struct Label {
  position: Position,
  text: Text,
  //color: Color,
  name: Option<Name>
}


impl Label {
  pub fn new() -> Label {
    Label {
      position: Position{ x: 0, y: 0 },
      text: Text::new(),
      name: None
    }
  }

  pub fn text(self, t: Text) -> Self {
    let mut label = self;
    label.text = t;
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
      .with(WidgetType::Label)
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

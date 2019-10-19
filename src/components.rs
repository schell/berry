use cassowary::*;
use specs::prelude::{
  Component,
  Builder,
  DenseVecStorage,
  Entities,
  Entity,
  FlaggedStorage,
  HashMapStorage,
  LazyUpdate,
  Read,
  VecStorage,
};
use std::ops::*;

use super::UI;
use super::picture::{Picture, PictureCmd};


#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct ElementBox {
  pub x: i32,
  pub y: i32,
  pub w: u32,
  pub h: u32,
}


impl ElementBox {
  pub fn new() -> ElementBox {
    ElementBox {
      x: 0, y: 0, w: 0, h: 0
    }
  }
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
}


#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Name(String);


#[derive(Clone, Component, Debug, Eq, Hash, PartialEq)]
pub enum VariableX {
  Left(Option<Entity>), Width(Option<Entity>), Right(Option<Entity>)
}
derive_syntax_for!(VariableX);


#[derive(Clone, Component, Debug, Eq, Hash, PartialEq)]
pub enum VariableY {
  Top(Option<Entity>), Height(Option<Entity>), Bottom(Option<Entity>)
}
derive_syntax_for!(VariableY);


pub struct ConstraintsX(pub Vec<Constraint<VariableX>>);


impl Component for ConstraintsX {
  type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}


pub struct ConstraintsY(pub Vec<Constraint<VariableY>>);


impl Component for ConstraintsY {
  type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}


pub struct Stage;


pub const stage: Stage = Stage;


pub trait HasXConstraints {
  fn left(&self) -> VariableX;
  fn width(&self) -> VariableX;
  fn right(&self) -> VariableX;
}


pub trait HasYConstraints {
  fn top(&self) -> VariableY;
  fn height(&self) -> VariableY;
  fn bottom(&self) -> VariableY;
}


impl HasXConstraints for Entity {
  fn left(&self) -> VariableX {
    VariableX::Left(Some(*self))
  }
  fn width(&self) -> VariableX {
    VariableX::Width(Some(*self))
  }
  fn right(&self) -> VariableX {
    VariableX::Right(Some(*self))
  }
}


impl HasYConstraints for Entity {
  fn top(&self) -> VariableY {
    VariableY::Top(Some(*self))
  }
  fn height(&self) -> VariableY {
    VariableY::Height(Some(*self))
  }
  fn bottom(&self) -> VariableY {
    VariableY::Bottom(Some(*self))
  }
}


/// A builder for a UI entity.
pub struct EntityBuilder {
  left: Option<Expression<VariableX>>,
  width: Option<Expression<VariableX>>,
  right: Option<Expression<VariableX>>,
  top: Option<Expression<VariableY>>,
  height: Option<Expression<VariableY>>,
  bottom: Option<Expression<VariableY>>,
  text: Option<Text>,
  picture: Option<Vec<PictureCmd>>,
  name: Option<Name>,
}


impl EntityBuilder {
  pub fn new() -> EntityBuilder {
    EntityBuilder {
      left: None,
      width: None,
      right: None,
      top: None,
      height: None,
      bottom: None,
      picture: None,
      text: None,
      name: None
    }
  }

  pub fn name(self, n: &str) -> Self {
    let mut eb = self;
    eb.name = Some(Name(n.to_string()));
    eb
  }

  pub fn text(self, t: &str) -> Self {
    let mut eb = self;
    let mut text =
      eb
      .text
      .unwrap_or(Text::new());
    text.text = t.to_string();
    eb.text = Some(text);
    eb
  }

  pub fn color(self, r: u8, g: u8, b: u8, a:u8) -> Self {
    let mut eb = self;
    let mut text =
      eb
      .text
      .unwrap_or(Text::new());
    text.text_color =
      Color {
        r, g, b, a
      };
    eb.text = Some(text);
    eb
  }

  pub fn left<T: Into<Expression<VariableX>>>(self, t: T) -> Self {
    let mut eb = self;
    eb.left = Some(t.into());
    eb
  }

  pub fn width<T: Into<Expression<VariableX>>>(self, t: T) -> Self {
    let mut eb = self;
    eb.width = Some(t.into());
    eb
  }

  pub fn right<T: Into<Expression<VariableX>>>(self, t: T) -> Self {
    let mut eb = self;
    eb.right = Some(t.into());
    eb
  }

  pub fn top<T: Into<Expression<VariableY>>>(self, t:T) -> Self {
    let mut eb = self;
    eb.top = Some(t.into());
    eb
  }

  pub fn height<T: Into<Expression<VariableY>>>(self, t:T) -> Self {
    let mut eb = self;
    eb.height = Some(t.into());
    eb
  }

  pub fn bottom<T: Into<Expression<VariableY>>>(self, t:T) -> Self {
    let mut eb = self;
    eb.bottom = Some(t.into());
    eb
  }

  pub fn picture(self, cmds: Vec<PictureCmd>) -> Self {
    let mut eb = self;
    let mut cmds = cmds;
    let mut prev_cmds =
      eb
      .picture
      .unwrap_or(vec![]);
    prev_cmds.append(&mut cmds);
    eb.picture = Some(prev_cmds);
    eb
  }

  pub fn build(self, ui: &UI) -> Entity {
    let (entities, lazy): (Entities, Read<LazyUpdate>) =
      ui
      .world
      .system_data();

    let ent =
      lazy
      .create_entity(&entities)
      .build();

    let may_xs:Vec<Option<Constraint<VariableX>>> =
      vec![
        self.left.map(|x| ent.left().is(x)),
        self.width.map(|x| ent.width().is(x)),
        self.right.map(|x| ent.right().is(x)),
        // We need a concrete relationship between left, width, and right
        Some(ent.right().is(ent.left() + ent.width()))
      ];
    let xs:Vec<Constraint<VariableX>> =
      may_xs
      .into_iter()
      .filter_map(|expx:Option<Constraint<VariableX>>| expx)
      .into_iter()
      .collect();
    lazy.insert(ent, ConstraintsX(xs));

    let may_ys:Vec<Option<Constraint<VariableY>>> =
      vec![
        self.top.map(|y| ent.top().is(y)),
        self.height.map(|y| ent.height().is(y)),
        self.bottom.map(|y| ent.bottom().is(y)),
        // We need a concrete relationship between top, height, and bottom
        Some(ent.bottom().is(ent.top() + ent.height()))
      ];
    let ys:Vec<Constraint<VariableY>> =
      may_ys
      .into_iter()
      .filter_map(|eypy:Option<Constraint<VariableY>>| eypy)
      .into_iter()
      .collect();
    lazy.insert(ent, ConstraintsY(ys));

    self
      .text
      .map(|t| lazy.insert(ent, t));

    self
      .picture
      .map(|cmds| lazy.insert(ent, Picture(cmds)));

    self
      .name
      .map(|name| lazy.insert(ent, name));

    ent
  }
}

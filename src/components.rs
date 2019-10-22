use cassowary::*;
use specs::prelude::{
  Component,
  Entities,
  Entity,
  HashMapStorage,
  VecStorage,
  WriteStorage
};

mod constraints;

pub use constraints::*;
use super::UI;
use super::picture::Picture;


#[derive(Clone, Component, Debug, PartialEq)]
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
  pub fn new(s: &str) -> Text {
    Text{
      font_path: "komika.ttf".to_string(),
      font_size: 16,
      text_color: Color{ r: 255, g: 255, b: 255, a: 255 },
      text: s.to_string()
    }
  }

  pub fn color(self, r:u8, g:u8, b:u8, a:u8) -> Self {
    let mut t = self;
    t.text_color = Color{ r, g, b, a};
    t
  }
}


#[derive(Clone, Component, Debug)]
#[storage(HashMapStorage)]
pub struct Name(pub String);


type EntityBuildData<'a> = (
  Entities<'a>,
  WriteStorage<'a, ConstraintsX>,
  WriteStorage<'a, ConstraintsY>,
  WriteStorage<'a, ElementBox>,
  WriteStorage<'a, Name>,
  WriteStorage<'a, Picture>,
  WriteStorage<'a, Text>
);


/// A builder for a UI entity.
pub struct EntityBuilder {
  left: Option<Expression<VariableX>>,
  width: Option<Expression<VariableX>>,
  right: Option<Expression<VariableX>>,
  top: Option<Expression<VariableY>>,
  height: Option<Expression<VariableY>>,
  bottom: Option<Expression<VariableY>>,
  text: Option<Text>,
  picture: Option<Picture>,
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

  pub fn text(self, t: &Text) -> Self {
    let mut eb = self;
    eb.text = Some(t.clone());
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

  pub fn picture(self, pic: &Picture) -> Self {
    let mut eb = self;
    eb.picture = Some(pic.clone());
    eb
  }

  fn build_with(
    self,
    (entities,
     mut constraints_x,
     mut constraints_y,
     mut element_boxes,
     mut names,
     mut pictures,
     mut texts
    ):EntityBuildData
  ) -> Entity {

    let ent =
      entities
      .create();

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
    constraints_x
      .insert(ent, ConstraintsX(xs))
      .expect("Could not insert x constraints in EntityBuilder::build");

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
    constraints_y
      .insert(ent, ConstraintsY(ys))
      .expect("Could not insert y constraints in EntityBuilder::build");

    self
      .text
      .map(|t| texts.insert(ent, t));

    self
      .picture
      .map(|pic| pictures.insert(ent, pic));

    self
      .name
      .map(|name| names.insert(ent, name));

    element_boxes
      .insert(ent, ElementBox::new())
      .expect("Could not insert element box in EntityBuilder::build");

    ent

  }

  pub fn build(self, ui: &mut UI) -> Entity {
    ui
      .world
      .exec(|data| self.build_with(data))
  }
}


pub struct Stage;


impl HasXConstraints for Stage {
  fn left(&self) -> Expression<VariableX> {
    VariableX::Left(None).into()
  }
  fn width(&self) -> Expression<VariableX> {
    VariableX::Width(None).into()
  }
}


impl HasYConstraints for Stage {
  fn top(&self) -> Expression<VariableY> {
    VariableY::Top(None).into()
  }
  fn height(&self) -> Expression<VariableY> {
    VariableY::Height(None).into()
  }
}

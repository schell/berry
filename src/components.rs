pub use cassowary::*;
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
pub use super::systems::shrinkwrap::ContentSize;
pub use super::systems::event::{Event, Events};


#[derive(Clone, Component, Debug, PartialEq)]
#[storage(HashMapStorage)]
pub struct Invisible;


#[derive(Clone, Component, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct ElementBox {
  pub x: i32,
  pub y: i32,
  pub z: i32,
  pub width: u32,
  pub height: u32,
}


impl ElementBox {
  pub fn new() -> ElementBox {
    ElementBox {
      x: 0, y: 0, z: 0, width: 0, height: 0
    }
  }

  pub fn left(&self) -> i32 {
    self.x
  }

  pub fn right(&self) -> i32 {
    self.x + self.width as i32
  }

  pub fn top(&self) -> i32 {
    self.y
  }

  pub fn bottom(&self) -> i32 {
    self.y + self.height as i32
  }

  pub fn z_index(&self) -> i32 {
    self.z
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
  WriteStorage<'a, Constraints<VariableX>>,
  WriteStorage<'a, Constraints<VariableY>>,
  WriteStorage<'a, Constraints<VariableZ>>,
  WriteStorage<'a, ElementBox>,
  WriteStorage<'a, Name>,
  WriteStorage<'a, Picture>,
  WriteStorage<'a, Text>
);


/// A builder for a UI entity.
pub struct ElementBuilder {
  left: Option<Expression<VariableX>>,
  width: Option<Expression<VariableX>>,
  right: Option<Expression<VariableX>>,
  top: Option<Expression<VariableY>>,
  z: Option<Expression<VariableZ>>,
  height: Option<Expression<VariableY>>,
  bottom: Option<Expression<VariableY>>,
  text: Option<Text>,
  picture: Option<Picture>,
  name: Option<Name>,
  x_constraints: Option<Vec<Constraint<VariableX>>>,
  y_constraints: Option<Vec<Constraint<VariableY>>>,
  z_constraints: Option<Vec<Constraint<VariableZ>>>,
  shrinkwrap: bool
}


impl ElementBuilder {
  pub fn new() -> ElementBuilder {
    ElementBuilder {
      left: None,
      width: None,
      right: None,
      top: None,
      z: None,
      height: None,
      bottom: None,
      picture: None,
      text: None,
      name: None,
      x_constraints: None,
      y_constraints: None,
      z_constraints: None,
      shrinkwrap: false
    }
  }

  pub fn shrink_to_contents(self) -> Self {
    let mut eb = self;
    eb.shrinkwrap = true;
    eb
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

  pub fn z_index<T: Into<Expression<VariableZ>>>(self, t:T) -> Self {
    let mut eb = self;
    eb.z = Some(t.into());
    eb
  }

  pub fn picture(self, pic: &Picture) -> Self {
    let mut eb = self;
    eb.picture = Some(pic.clone());
    eb
  }

  pub fn x_constraints(self, xs:Vec<Constraint<VariableX>>) -> Self {
    let mut eb = self;
    eb.x_constraints = Some(xs);
    eb
  }

  pub fn y_constraints(self, ys:Vec<Constraint<VariableY>>) -> Self {
    let mut eb = self;
    eb.y_constraints = Some(ys);
    eb
  }

  pub fn z_constraints(self, zs:Vec<Constraint<VariableZ>>) -> Self {
    let mut eb = self;
    eb.z_constraints = Some(zs);
    eb
  }

  pub fn update(self, ui: &mut UI, ent:Entity) {
    ui.world
      .exec(|data:EntityBuildData| self.build_with(data, Some(ent)));
  }

  fn build_with(
    self,
    (entities,
     mut constraints_x,
     mut constraints_y,
     mut constraints_z,
     mut element_boxes,
     mut names,
     mut pictures,
     mut texts
    ):EntityBuildData,
    may_ent: Option<Entity>
  ) -> Entity {
    let ent =
      may_ent
      .unwrap_or(entities.create());

    let has_x_constraints =
      self.left.is_some()
      || self.width.is_some()
      || self.right.is_some()
      || self.x_constraints.is_some();
    if has_x_constraints {
      let may_xs:Vec<Option<Constraint<VariableX>>> =
        vec![
          self.left.map(|x| ent.left().is(x)),
          self.width.map(|x| ent.width().is(x)),
          self.right.map(|x| ent.right().is(x)),
          // We need a concrete relationship between left, width, and right
          Some(ent.right().is(ent.left() + ent.width()))
        ];
      let mut xs:Vec<Constraint<VariableX>> =
        may_xs
        .into_iter()
        .filter_map(|expx:Option<Constraint<VariableX>>| expx)
        .into_iter()
        .collect();
      xs.extend(
        self
          .x_constraints
          .unwrap_or(vec![])
          .into_iter()
      );
      constraints_x
        .insert(ent, Constraints(xs))
        .expect("Could not insert x constraints in ElementBuilder::build");
    }

    let has_y_constraints =
      self.top.is_some()
      || self.height.is_some()
      || self.bottom.is_some()
      || self.y_constraints.is_some();
    if has_y_constraints {
      let may_ys:Vec<Option<Constraint<VariableY>>> =
        vec![
          self.top.map(|y| ent.top().is(y)),
          self.height.map(|y| ent.height().is(y)),
          self.bottom.map(|y| ent.bottom().is(y)),
          // We need a concrete relationship between top, height, and bottom
          Some(ent.bottom().is(ent.top() + ent.height()))
        ];
      let mut ys:Vec<Constraint<VariableY>> =
        may_ys
        .into_iter()
        .filter_map(|expy:Option<Constraint<VariableY>>| expy)
        .into_iter()
        .collect();
      ys.extend(
        self
          .y_constraints
          .unwrap_or(vec![])
          .into_iter()
      );
      constraints_y
        .insert(ent, Constraints(ys))
        .expect("Could not insert y constraints in ElementBuilder::build");
    }

    let mut zs:Vec<Constraint<VariableZ>> =
      self
      .z_constraints
      .unwrap_or(vec![]);
    self
      .z
      .into_iter()
      .for_each(|z| {
        zs.push(ent.z_index().is(z));
      });
    if zs.len() > 0 {
      constraints_z
        .insert(ent, Constraints(zs))
        .expect("Could not insert z constraints in ElementBuilder::build");
    }

    self
      .text
      .map(|t| texts.insert(ent, t));

    self
      .picture
      .map(|pic| pictures.insert(ent, pic));

    self
      .name
      .map(|name| names.insert(ent, name));

    if !element_boxes.contains(ent) {
      element_boxes
        .insert(ent, ElementBox::new())
        .expect("Could not insert element box in ElementBuilder::build");
    }

    ent
  }

  pub fn build(self, ui: &mut UI) -> Entity {
    ui.world
      .exec(|data:EntityBuildData| self.build_with(data, None))
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

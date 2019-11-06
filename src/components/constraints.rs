use cassowary::*;
use specs::prelude::{
  Component,
  DenseVecStorage,
  Entity,
  FlaggedStorage,
  ReadStorage,
  VecStorage,
};
use std::ops::*;
use std::any::Any;

use super::Name;


// TODO: Add MouseX and MouseY to constraint variables
#[derive(Clone, Component, Debug, Eq, Hash, PartialEq)]
pub enum VariableX {
  Left(Option<Entity>), Width(Option<Entity>)
}
derive_syntax_for!(VariableX);


impl VariableX {
  pub fn to_pathy_string(&self, names: &ReadStorage<Name>) -> String {
    let (dir, may_ent) =
      match self {
        VariableX::Left(may_ent) => {("left", may_ent)}
        VariableX::Width(may_ent) => {("width", may_ent)}
      };
    let me:String =
      may_ent
      .map(|ent| {
        names
          .get(ent)
          .map(|Name(s)| s.clone())
          .unwrap_or(format!("entity({:?})", ent.id()))
      })
      .unwrap_or("stage".to_string());

    format!("{}.{}", me, dir)
  }
}


#[derive(Clone, Component, Debug, Eq, Hash, PartialEq)]
pub enum VariableY {
  Top(Option<Entity>), Height(Option<Entity>)
}
derive_syntax_for!(VariableY);


impl VariableY {
  pub fn to_pathy_string(&self, names: &ReadStorage<Name>) -> String {
    let (dir, may_ent) =
      match self {
        VariableY::Top(may_ent) => {("top", may_ent)}
        VariableY::Height(may_ent) => {("height", may_ent)}
      };
    let me:String =
      may_ent
      .map(|ent| {
        names
          .get(ent)
          .map(|Name(s)| s.clone())
          .unwrap_or(format!("entity({:?})", ent.id()))
      })
      .unwrap_or("stage".to_string());

    format!("{}.{}", me, dir)
  }

}


#[derive(Clone, Component, Debug, Eq, Hash, PartialEq)]
pub struct VariableZ(pub Entity);
derive_syntax_for!(VariableZ);


#[derive(Clone, Debug)]
pub struct Constraints<T>(pub Vec<Constraint<T>>);


impl<T: Send + Sync + Any> Component for Constraints<T> {
  type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}


pub trait HasXConstraints {
  fn left(&self) -> Expression<VariableX>;
  fn width(&self) -> Expression<VariableX>;
  fn right(&self) -> Expression<VariableX> {
    self.left() + self.width()
  }
}


pub trait HasYConstraints {
  fn top(&self) -> Expression<VariableY>;
  fn height(&self) -> Expression<VariableY>;
  fn bottom(&self) -> Expression<VariableY> {
    self.top() + self.height()
  }
}


pub trait HasZConstraints {
  fn z_index(&self) -> Expression<VariableZ>;
}


impl HasXConstraints for Entity {
  fn left(&self) -> Expression<VariableX> {
    VariableX::Left(Some(*self)).into()
  }
  fn width(&self) -> Expression<VariableX> {
    VariableX::Width(Some(*self)).into()
  }
}


impl HasYConstraints for Entity {
  fn top(&self) -> Expression<VariableY> {
    VariableY::Top(Some(*self)).into()
  }
  fn height(&self) -> Expression<VariableY> {
    VariableY::Height(Some(*self)).into()
  }
}


impl HasZConstraints for Entity {
  fn z_index(&self) -> Expression<VariableZ> {
    VariableZ(*self).into()
  }
}

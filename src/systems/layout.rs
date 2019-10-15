use cassowary::Solver;
pub use cassowary::{Variable, Constraint};
pub use cassowary::WeightedRelation::*;
pub use cassowary::strength::{ WEAK, MEDIUM, STRONG, REQUIRED };

use specs::prelude::*;

use super::super::WindowSize;

pub struct LayoutBox {
  pub left: Variable,
  pub top: Variable
}


#[derive(Debug, Clone)]
pub enum Prim {
  Window,
  Other(Entity)
}

#[derive(Debug, Clone)]
pub enum Value {
  Left(Prim), Right(Prim)
}


#[derive(Debug, Clone)]
pub enum Expr {
  Equal(Value, Value)
}


// #[derive(Clone, Component, Debug)]
// #[storage(HashMapStorage)]
// pub struct Layout {
//   object: LayoutObject,
//   variable: Option<Edge>
// }


//impl Layout {
//  pub fn window() -> Layout {
//    Layout {
//      object: LayoutObject::Window,
//      variable: None
//    }
//  }
//
//  pub fn make(ent: Entity) -> Layout {
//    Layout {
//      object: LayoutObject::Other(ent),
//      variable: None
//    }
//  }
//
//  pub fn left() -> Layout
//}


pub struct LayoutSystem {
  window_width: Variable,
  window_height: Variable,
  solver: Solver
}


impl LayoutSystem {
  pub fn new() -> LayoutSystem {
    LayoutSystem {
      window_width: Variable::new(),
      window_height: Variable::new(),
      solver: Solver::new()
    }
  }
}


impl<'a> System<'a> for LayoutSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, Layout>,
    Read<'a, WindowSize>
  );

  fn run(&mut self, (entities, layouts, window_size): Self::SystemData) {
    return;
    let constraints:Vec<_> =
      panic!("constraints todo");

    self
      .solver
      .add_constraints(&constraints)
      .unwrap();

    self
      .solver
      .add_edit_variable(self.window_width, STRONG)
      .unwrap();

    self
      .solver
      .suggest_value(self.window_width, window_size.width as f64)
      .unwrap();
  }
}

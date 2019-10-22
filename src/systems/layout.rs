use std::fmt::Debug;
use std::hash::Hash;
pub use cassowary::*;
use specs::prelude::*;

use super::super::WindowSize;
use super::super::components::{
  ElementBox,
  ConstraintsX,
  ConstraintsY,
  Name,
  VariableX,
  VariableY
};


pub struct LayoutSystem {
  solver_x: Option<Solver<VariableX>>,
  solver_y: Option<Solver<VariableY>>,
  x_reader: Option<ReaderId<ComponentEvent>>,
  y_reader: Option<ReaderId<ComponentEvent>>
}


impl LayoutSystem {
  pub fn new() -> LayoutSystem {
    LayoutSystem {
      solver_x: None,
      solver_y: None,
      x_reader: None,
      y_reader: None
    }
  }

  fn run_solver<T, C, F>(
    solver: &mut Solver<T>,
    reader: &mut ReaderId<ComponentEvent>,

    entities: &Entities,
    constraints:&ReadStorage<C>,
    to_constraints: F
  ) -> Vec<(T, f64)>
  where
    T: Clone + Debug + Eq + Hash,
    C: Component,
    <C as Component>::Storage: Tracked,
    F: Fn(&C) -> Vec<Constraint<T>>
  {
    constraints
      .channel()
      .read(reader)
      .for_each(|event| {
        match event {
          ComponentEvent::Inserted(id) => {
            let ent =
              entities
              .entity(*id);
            let new_constraints =
              constraints
              .get(ent)
              .expect("Could not find inserted constraint");
            let owned_constraints =
              to_constraints(new_constraints);
            solver
              .add_constraints(owned_constraints)
              .expect("Could not add new constraints");
          }
          ComponentEvent::Modified(_id) => {
            // TODO: How does one find the previous constraints?
            panic!("No support for modifying constraints")

          }
          ComponentEvent::Removed(id) => {
            let _ent =
              entities
              .entity(*id);
            // TODO: Filter any constraints that contain a variable of id
            panic!("No support for removing constraints")
          }
        }
      });

    // Fetch changes from the solver
    solver
      .fetch_changes()
      .to_vec()
  }
}


impl<'a> System<'a> for LayoutSystem {
  type SystemData = (
    Entities<'a>,

    Read<'a, WindowSize>,

    ReadStorage<'a, ConstraintsX>,
    ReadStorage<'a, ConstraintsY>,
    WriteStorage<'a, ElementBox>,
    ReadStorage<'a, Name>
  );

  fn setup(&mut self, world: &mut World) {
    Self::SystemData::setup(world);
    let mut xs: WriteStorage<ConstraintsX> =
      SystemData::fetch(world);
    self.x_reader =
      Some(xs.register_reader());
    let mut ys: WriteStorage<ConstraintsY> =
      SystemData::fetch(world);
    self.y_reader =
      Some(ys.register_reader());
  }

  fn run(
    &mut self,
    (entities,
     window_size,
     constraints_x,
     constraints_y,
     mut element_boxes,
     _names,
    ): Self::SystemData
  ) {

    let mut x_solver =
      self
      .solver_x
      .take()
      .unwrap_or({
        let mut solver =
          Solver::new();
        solver
          .add_constraint(VariableX::Left(None).is(0))
          .expect("Could not add window left is 0 constraint");
        // Add the window width variable
        solver
          .add_edit_variable(VariableX::Width(None), strength::STRONG)
          .expect("Could not add edit variable for window width");
        solver
      });

    x_solver
      .suggest_value(VariableX::Width(None), window_size.width as f64)
      .expect("Could not suggest value for window width");

    let mut x_reader =
      self
      .x_reader
      .as_mut()
      .expect("LayoutSystem has no x constraint update reader");

    Self::run_solver(
      &mut x_solver,
      &mut x_reader,
      &entities,
      &constraints_x,
      |ConstraintsX(cs)| cs.clone()
    ) .into_iter()
      .for_each(|(var, val): (VariableX, f64)| {
        match var {
          VariableX::Left(Some(ent)) => {
            let mut el =
              element_boxes
              .get(ent)
              .cloned()
              .unwrap_or(ElementBox::new());
            el.x = val as i32;
            element_boxes
              .insert(ent, el)
              .expect("Could not update element box");
          }
          VariableX::Width(Some(ent)) => {
            let mut el =
              element_boxes
              .get(ent)
              .cloned()
              .unwrap_or(ElementBox::new());
            el.w = val as u32;
            element_boxes
              .insert(ent, el)
              .expect("Could not update element box");
          }
          _ => {}
        };

        //println!("layout: {} = {:?}", var.to_pathy_string(&names), val);
      });

    self.solver_x =
      Some(x_solver);

    let mut y_solver =
      self
      .solver_y
      .take()
      .unwrap_or({
        let mut solver =
          Solver::new();
        solver
          .add_constraint(VariableY::Top(None).is(0))
          .expect("Could not add window top is 0 constraint");
        // Add the window height variable
        solver
          .add_edit_variable(VariableY::Height(None), strength::STRONG)
          .expect("Could not add edit variable for window height");
        solver
      });

    y_solver
      .suggest_value(VariableY::Height(None), window_size.height as f64)
      .expect("Could not suggest value for window height");

    let mut y_reader =
      self
      .y_reader
      .as_mut()
      .expect("LayoutSystem has no y constraint update reader");

    Self::run_solver(
      &mut y_solver,
      &mut y_reader,
      &entities,
      &constraints_y,
      |ConstraintsY(cs)| cs.clone()
    ) .into_iter()
      .for_each(|(var, val): (VariableY, f64)| {
        match var {
          VariableY::Top(Some(ent)) => {
            let mut el =
              element_boxes
              .get(ent)
              .cloned()
              .unwrap_or(ElementBox::new());
            el.y = val as i32;
            element_boxes
              .insert(ent, el)
              .expect("Could not update element boy");
          }
          VariableY::Height(Some(ent)) => {
            let mut el =
              element_boxes
              .get(ent)
              .cloned()
              .unwrap_or(ElementBox::new());
            el.h = val as u32;
            element_boxes
              .insert(ent, el)
              .expect("Could not update element boy");
          }
          _ => {}
        };

        //println!("layout: {} = {:?}", var.to_pathy_string(&names), val);
      });

    self.solver_y =
      Some(y_solver);
  }
}

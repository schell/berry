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
     _window_size,
     constraints_x,
     _constraints_y,
     mut element_boxes,
     names,
    ): Self::SystemData
  ) {

    let mut x_solver =
      self
      .solver_x
      .take()
      .unwrap_or({
        // The first time a solver is created it must get all the constraints
        let mut solver =
          Solver::new();
        let constraints:Vec<Constraint<VariableX>> =
          (&constraints_x)
          .join()
          .flat_map(|ConstraintsX(cs)| cs.clone())
          .collect();
        solver
          .add_constraints(constraints)
          .expect("Could not add initial x constraints to solver");
        solver
      });

    // Run through any new x constraint updates
    let x_reader =
      self
      .x_reader
      .as_mut()
      .expect("LayoutSystem has no x constraint update reader");
    constraints_x
      .channel()
      .read(x_reader)
      .for_each(|event| {
        match event {
          ComponentEvent::Inserted(id) => {
            let ent =
              entities
              .entity(*id);
            let new_constraints =
              constraints_x
              .get(ent)
              .expect("Could not find inserted constraint");
            x_solver
              .add_constraints(new_constraints.0.clone())
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

    // Fetch changes from the solver and apply them to the ECS
    x_solver
      .fetch_changes()
      .into_iter()
      .for_each(|(var, val): &(VariableX, f64)| {
        let may_ent:Option<Entity> =
          match var {
            VariableX::Left(Some(ent)) => {
              let mut el =
                element_boxes
                .get(*ent)
                .cloned()
                .unwrap_or(ElementBox::new());
              el.x = *val as i32;
              element_boxes
                .insert(*ent, el)
                .expect("Could not update element box");
              Some(*ent)
            }
            VariableX::Width(Some(ent)) => {
              let mut el =
                element_boxes
                .get(*ent)
                .cloned()
                .unwrap_or(ElementBox::new());
              el.w = *val as u32;
              element_boxes
                .insert(*ent, el)
                .expect("Could not update element box");
              Some(*ent)
            }
            VariableX::Right(Some(ent)) => {
              let mut el =
                element_boxes
                .get(*ent)
                .cloned()
                .unwrap_or(ElementBox::new());
              el.x = *val as i32 - el.w as i32;
              element_boxes
                .insert(*ent, el)
                .expect("Could not update element box");
              Some(*ent)
            }
            _ => { None }
          };

        let name =
          may_ent
          .map(|e| names.get(e))
          .unwrap_or(None);
        println!("layout: {:?} of {:?} is {:?}", var, name, val);
      });

    // Save the x solver
    self.solver_x = Some(x_solver);
  }
}

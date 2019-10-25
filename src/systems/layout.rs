pub use cassowary::*;

use specs::prelude::*;

use std::collections::HashMap;
use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;

use super::super::WindowSize;
use super::super::components::{
  ElementBox,
  Constraints,
  VariableX,
  VariableY,
  Name,
};


/// The SystemData for an IsLayoutSystem implementation.
type LayoutSystemData<'a, T, R> = (
  Entities<'a>,

  Read<'a, R>,

  ReadStorage<'a, Constraints<T>>,
  WriteStorage<'a, ElementBox>,
  ReadStorage<'a, Name>
);


pub trait IsLayoutSystem<T, R>
where
  T: Any + Clone + Debug + Eq + Hash + Send + Sync,
  R: Any + Default + Send + Sync
{

  fn solver_mut(&mut self) -> &mut Option<Solver<T>>;
  fn reader_mut(&mut self) -> &mut Option<ReaderId<ComponentEvent>>;
  fn cache_mut(&mut self) -> &mut HashMap<u32, Constraints<T>>;

  fn initial_constraints(&self) -> Constraints<T>;
  fn edit_variables(&self) -> Vec<T>;
  fn get_edit_variable_value(&self, variable: &T, source: &Read<R>) -> f64;

  fn update_variable_value(
    &self,
    element_boxes: &mut WriteStorage<ElementBox>,
    names: &ReadStorage<Name>,
    variable: T,
    value: f64
  );

  fn setup(&mut self, world: &mut World) {
    <LayoutSystemData<T, R> as SystemData>::setup(world);
    let mut cs: WriteStorage<Constraints<T>> =
      SystemData::fetch(world);
    *self.reader_mut() =
      Some(cs.register_reader());
  }

  fn run<'a>(
    &mut self,
    (entities,
     edit_variable_values,
     constraints,
     mut element_boxes,
     names,
    ): LayoutSystemData<'a, T, R>
  ) {
    let mut solver =
      self
      .solver_mut()
      .take()
      .unwrap_or({
        let mut solver =
          Solver::new();
        solver
          .add_constraints(self.initial_constraints().0)
          .expect("Could not add initial constraints");

        // Add the edit variables
        self
          .edit_variables()
          .iter()
          .for_each(|v| {
            solver
              .add_edit_variable(v.clone(), strength::STRONG)
              .expect("Could not add edit variable");
          });

        solver
      });

    self
      .edit_variables()
      .iter()
      .for_each(|e| {
        let value =
          self
          .get_edit_variable_value(e, &edit_variable_values);
        solver
          .suggest_value(e.clone(), value)
          .expect(&format!("Could not suggest value for edit variable {:?}", e));
      });

    let reader =
      self
      .reader_mut()
      .as_mut()
      .expect("LayoutSystem has no constraint update reader");

    let insert = |id: u32, the_solver: &mut Solver<T>, cache: &mut HashMap<u32, Constraints<T>>| {
      let ent =
        entities
        .entity(id);
      let new_constraints =
        constraints
        .get(ent)
        .expect("Could not find inserted constraint");
      the_solver
        .add_constraints(new_constraints.0.clone())
        .expect("Could not add new constraints");
      cache
        .insert(id, new_constraints.clone());
    };

    let remove = |id: u32, the_solver: &mut Solver<T>, cache: &mut HashMap<u32, Constraints<T>>| {
      cache
        .remove(&id)
        .expect("Could not remove constraints from cache")
        .0
        .into_iter()
        .for_each(|c| {
          the_solver
            .remove_constraint(&c)
            .expect("Could not remove constraint");
        });
    };

    constraints
      .channel()
      .read(reader)
      .for_each(|event| {
        match event {
          ComponentEvent::Inserted(id) => {
            insert(*id, &mut solver, self.cache_mut());
          }
          ComponentEvent::Modified(id) => {
            remove(*id, &mut solver, self.cache_mut());
            insert(*id, &mut solver, self.cache_mut());
          }
          ComponentEvent::Removed(id) => {
            remove(*id, &mut solver, self.cache_mut());
          }
        }
      });

    // Fetch changes from the solver and input them into the ECS
    solver
      .fetch_changes()
      .into_iter()
      .for_each(|(variable, value)| {
        self.update_variable_value(&mut element_boxes, &names, variable.clone(), *value)
      });

    *self.solver_mut() =
      Some(solver);
  }
}


pub struct LayoutSystem<T>
where
  T: Any + Clone + Debug + Eq + Hash + Send + Sync,
{
  pub solver: Option<Solver<T>>,
  pub reader: Option<ReaderId<ComponentEvent>>,
  pub cache: HashMap<u32, Constraints<T>>
}


impl<T> LayoutSystem<T>
where
  T: Any + Clone + Debug + Eq + Hash + Send + Sync,
{
  pub fn new() -> LayoutSystem<T> {
    LayoutSystem {
      solver: None,
      reader: None,
      cache: HashMap::new()
    }
  }
}


impl IsLayoutSystem<VariableX, WindowSize> for LayoutSystem<VariableX> {
  fn solver_mut(&mut self) -> &mut Option<Solver<VariableX>> {
    &mut self.solver
  }

  fn reader_mut(&mut self) -> &mut Option<ReaderId<ComponentEvent>> {
    &mut self.reader
  }

  fn cache_mut(&mut self) -> &mut HashMap<u32, Constraints<VariableX>> {
    &mut self.cache
  }

  fn initial_constraints(&self) -> Constraints<VariableX> {
    Constraints(vec![VariableX::Left(None).is(0)])
  }

  fn edit_variables(&self) -> Vec<VariableX> {
    vec![VariableX::Width(None)]
  }

  fn get_edit_variable_value(&self, variable: &VariableX, window_size: &Read<WindowSize>) -> f64 {
    match variable {
      VariableX::Left(None) => { 0.0 }
      VariableX::Width(None) => { window_size.width as f64 }
      _ => { panic!("No support for using entities as edit variables") }
    }
  }

  fn update_variable_value(
    &self,
    element_boxes: &mut WriteStorage<ElementBox>,
    _names: &ReadStorage<Name>,
    var: VariableX,
    val: f64
  ) {
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
          .expect("Could not update element box x");
      }
      VariableX::Width(Some(ent)) => {
        let mut el =
          element_boxes
          .get(ent)
          .cloned()
          .unwrap_or(ElementBox::new());
        if (val as u32) == 4294967196 {
          panic!("wtf {:?}", val);
        }
        el.width = val as u32;
        element_boxes
          .insert(ent, el)
          .expect("Could not update element box width");
      }
      _ => {}
    };
    //println!("layout: {} = {:?}", var.to_pathy_string(&names), val);
  }
}


impl<'a> System<'a> for LayoutSystem<VariableX> {
  type SystemData = LayoutSystemData<'a, VariableX, WindowSize>;

  fn setup(&mut self, world: &mut World) {
    (self as &mut IsLayoutSystem<VariableX, WindowSize>).setup(world)
  }

  fn run(&mut self, data: Self::SystemData) {
    (self as &mut IsLayoutSystem<VariableX, WindowSize>).run(data);
  }
}


impl IsLayoutSystem<VariableY, WindowSize> for LayoutSystem<VariableY> {
  fn solver_mut(&mut self) -> &mut Option<Solver<VariableY>> {
    &mut self.solver
  }

  fn reader_mut(&mut self) -> &mut Option<ReaderId<ComponentEvent>> {
    &mut self.reader
  }

  fn cache_mut(&mut self) -> &mut HashMap<u32, Constraints<VariableY>> {
    &mut self.cache
  }

  fn initial_constraints(&self) -> Constraints<VariableY> {
    Constraints(vec![VariableY::Top(None).is(0)])
  }

  fn edit_variables(&self) -> Vec<VariableY> {
    vec![VariableY::Height(None)]
  }

  fn get_edit_variable_value(&self, variable: &VariableY, window_size: &Read<WindowSize>) -> f64 {
    match variable {
      VariableY::Top(None) => { 0.0 }
      VariableY::Height(None) => { window_size.height as f64 }
      _ => { panic!("No support for using entities as edit variables") }
    }
  }

  fn update_variable_value(
    &self,
    element_boyes: &mut WriteStorage<ElementBox>,
    _names: &ReadStorage<Name>,
    var: VariableY,
    val: f64
  ) {
    match var {
      VariableY::Top(Some(ent)) => {
        let mut el =
          element_boyes
          .get(ent)
          .cloned()
          .unwrap_or(ElementBox::new());
        el.y = val as i32;
        element_boyes
          .insert(ent, el)
          .expect("Could not update element y");
      }
      VariableY::Height(Some(ent)) => {
        let mut el =
          element_boyes
          .get(ent)
          .cloned()
          .unwrap_or(ElementBox::new());
        el.height = val as u32;
        element_boyes
          .insert(ent, el)
          .expect("Could not update element height");
      }
      _ => {}
    };
    //println!("layout: {} = {:?}", var.to_pathy_string(&names), val);
  }
}


impl<'a> System<'a> for LayoutSystem<VariableY> {
  type SystemData = LayoutSystemData<'a, VariableY, WindowSize>;

  fn setup(&mut self, world: &mut World) {
    (self as &mut IsLayoutSystem<VariableY, WindowSize>).setup(world)
  }

  fn run(&mut self, data: Self::SystemData) {
    (self as &mut IsLayoutSystem<VariableY, WindowSize>).run(data);
  }
}

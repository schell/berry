use specs::prelude::*;
use cassowary::*;

use super::super::components::{Constraints, HasXConstraints, HasYConstraints, VariableX, VariableY};

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct ShrinkwrapRequest;


#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct ContentSize {
  pub width: u32,
  pub height: u32
}


/// The shrinkwrap system simply updates the ElementBox of an entity to fit its
/// Picture and Text contents. The size of the contents themselves should already
/// exist in the ECS, put there by the rasterizer.
pub struct ShrinkwrapSystem;


impl<'a> System<'a> for ShrinkwrapSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, ContentSize>,
    WriteStorage<'a, Constraints<VariableX>>,
    WriteStorage<'a, Constraints<VariableY>>,
    WriteStorage<'a, ShrinkwrapRequest>,
  );

  fn run(&mut self, (entities, content_sizes, mut x_cs, mut y_cs, mut shrinkwraps): Self::SystemData) {
    let to_shrink:Vec<(Entity, ContentSize)> =
      (&entities, &content_sizes, &shrinkwraps)
      .join()
      .map(|(e, cs, _)| (e, cs.clone()))
      .collect::<Vec<_>>();
    to_shrink
      .into_iter()
      .for_each(|(ent, csize)| {
        let mut xs =
          x_cs
          .get(ent)
          .cloned()
          .unwrap_or(Constraints(vec![]));
        xs.0.push(ent.width().is(csize.width));
        x_cs
          .insert(ent, xs)
          .expect("Could not insert shrinkwrap x constraints");

        let mut ys =
          y_cs
          .get(ent)
          .cloned()
          .unwrap_or(Constraints(vec![]));
        ys.0.push(ent.height().is(csize.height));
        y_cs
          .insert(ent, ys)
          .expect("Could not insert shrinkwrap y constraints");

        // We only do this once per request
        shrinkwraps.remove(ent);
      });
  }
}

use specs::prelude::*;
use cassowary::*;
use cassowary::strength::*;

use super::super::components::{
  Constraints,
  ElementBox,
  HasXConstraints,
  HasYConstraints,
  Text,
  VariableX,
  VariableY
};

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct ContentSize {
  pub width: u32,
  pub height: u32
}


impl ContentSize {
  pub fn new() -> ContentSize {
    ContentSize {
      width: 0,
      height: 0
    }
  }
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
  );

  fn run(
    &mut self,
    (entities,
     content_sizes,
     mut x_cs,
     mut y_cs,
    ): Self::SystemData
  ) {
    for(ent, csize) in (&entities, &content_sizes).join() {
      let mut xs =
        x_cs
        .get(ent)
        .cloned()
        .unwrap_or(Constraints(vec![]));
      let x_constraint =
        ent.width().is(csize.width).with_strength(WEAK);
      if !xs.0.contains(&x_constraint) {
        xs.0.push(x_constraint);
        x_cs
          .insert(ent, xs)
          .expect("Could not insert shrinkwrap x constraints");
      }

      let mut ys =
        y_cs
        .get(ent)
        .cloned()
        .unwrap_or(Constraints(vec![]));
      let y_constraint =
        ent.height().is(csize.height).with_strength(WEAK);
      if !ys.0.contains(&y_constraint) {
        ys.0.push(y_constraint);
        y_cs
          .insert(ent, ys)
          .expect("Could not insert shrinkwrap y constraints");
      }
    }
  }
}

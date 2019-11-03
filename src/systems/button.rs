use specs::prelude::*;


use super::super::components::*;
use super::super::UI;


#[derive(Clone, Component, Debug, PartialEq)]
#[storage(HashMapStorage)]
pub struct Button {
  bg_up: Entity,
  bg_over: Entity,
  bg_down: Entity,
  text_up: Entity,
  text_over: Entity,
  text_down: Entity
};


pub struct ButtonSystem;


impl<'a> System<'a> for ButtonSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, Button>,
    WriteStorage<'a, Picture>,
    WriteStorage<'a, Text>
  );

  fn run(&mut self, (entities, buttons, mut pictures, mut texts): Self::SystemData) {
    // Run through all buttons without
  }
}

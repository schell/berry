use specs::prelude::*;

use super::WindowSize;
use super::components::*;
use super::systems::layout::LayoutSystem;
use super::resources::Resources;
use super::drawing::{DrawingSystemData, run_sdl2_drawing};


pub struct UI<'a, 'b> {
  pub world: World,
  dispatcher: Dispatcher<'a, 'b>
}


impl<'a, 'b> UI<'a, 'b> {
  pub fn new<'c, 'd>() -> UI<'c, 'd> {
    let mut world
      = World::new();

    world
      .setup::<DrawingSystemData>();

    let mut dispatcher =
      DispatcherBuilder::new()
      .with_thread_local(LayoutSystem::new())
      .build();
    dispatcher
      .setup(&mut world);

    UI {
      world,
      dispatcher
    }
  }

  pub fn maintain(&mut self, resources: &mut Resources) {
    // Update the size of the window so layout has something
    // to work with
    {
      let canvas =
        resources
        .canvas
        .take()
        .expect("UI resources has no canvas during maintain call");

      let mut window_size:Write<WindowSize> =
        self
        .world
        .system_data();
      // Update the window size
      let (ww, wh) =
        canvas
        .output_size()
        .expect("Could not get window output size");
      *window_size =
        WindowSize {
          width: ww,
          height: wh
        };

      resources.canvas =
        Some(canvas);
    }

    self
      .dispatcher
      .dispatch(&mut self.world);

    self
      .world
      .maintain();

    // Draw the things
    let data:DrawingSystemData =
      self
      .world
      .system_data();

    run_sdl2_drawing(resources, data);
  }

  pub fn get_size(&self, ent: Entity) -> Option<(u32, u32)> {
    let elements:ReadStorage<ElementBox> =
      self
      .world
      .system_data();
    elements
      .get(ent)
      .map(|elbox| {
        (elbox.w, elbox.h)
      })
  }

  pub fn get_position(&self, ent: Entity) -> Option<(i32, i32)> {
    let elements:ReadStorage<ElementBox> =
      self
      .world
      .system_data();
    elements
      .get(ent)
      .map(|elbox| {
        (elbox.x, elbox.y)
      })
  }

  pub fn get<T:Component + Clone>(&self, ent: Entity) -> Option<T> {
    let comps:ReadStorage<T> =
      self
      .world
      .system_data();
    comps
      .get(ent)
      .cloned()
  }

  pub fn stage(&self) -> Stage {
    Stage
  }
}

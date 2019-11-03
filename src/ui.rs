use specs::prelude::*;

use super::WindowSize;
use super::components::*;
use super::systems::event::{EventSystem, Mouse};
use super::systems::layout::*;
use super::systems::shrinkwrap::{ContentSize, ShrinkwrapSystem};
use super::rasterizer::{Rasterizer, DrawingSystemData};


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
      .with(ShrinkwrapSystem, "shrinkwrap", &[])
      .with(LayoutSystem::<VariableX>::new(), "layout_x", &[])
      .with(LayoutSystem::<VariableY>::new(), "layout_y", &[])
      .with(EventSystem::new(), "event", &[])
      .build();
    dispatcher
      .setup(&mut world);

    UI {
      world,
      dispatcher
    }
  }

  pub fn update_mouse(&mut self, mouse:Mouse) {
    let mut mouse_rez: Write<Mouse> =
      self
      .world
      .system_data();
    *mouse_rez = mouse;
  }

  pub fn update<C:Component>(&mut self, ent: &Entity, component:C) {
    let mut data:WriteStorage<C> =
      self
      .world
      .system_data();
    data
      .insert(*ent, component)
      .unwrap();
  }

  pub fn has_event(&self, ent: Entity, event:Event) -> bool {
    let data:ReadStorage<Events> =
      self
      .world
      .system_data();
    data
      .get(ent)
      .map(|evs| evs.0.contains(&event))
      .unwrap_or(false)
  }

  pub fn maintain(&mut self, rasterizer: &mut Rasterizer) {
    // Update the size of the window so layout has something
    // to work with
    {
      let canvas =
        rasterizer
        .canvas
        .take()
        .expect("UI rasterizer has no canvas during maintain call");

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

      rasterizer.canvas =
        Some(canvas);
    }

    // Do these things so that we have entity's content before running
    // systems.
    {
      let data:DrawingSystemData =
        self
        .world
        .system_data();

      let mut content_sizes: WriteStorage<ContentSize> =
        self
        .world
        .system_data();

      // Run through pictures and text and rasterize them, updating their
      // entity's content size
      (&data.0, &data.3)
        .join()
        .for_each(|(ent, pic)| {
          let (_, w, h) =
            rasterizer
            .get_picture(pic);
          let mut cs =
            content_sizes
            .get(ent)
            .cloned()
            .unwrap_or(ContentSize{width:0, height:0});
          cs.width = u32::max(w, cs.width);
          cs.height = u32::max(h, cs.height);
          content_sizes
            .insert(ent, cs)
            .expect("Could not insert content size");
        });
      (&data.0, &data.4)
        .join()
        .for_each(|(ent, text)| {
          let (_, w, h) =
            rasterizer
            .get_text(text);
          let mut cs =
            content_sizes
            .get(ent)
            .cloned()
            .unwrap_or(ContentSize{width:0, height:0});
          cs.width = u32::max(w, cs.width);
          cs.height = u32::max(h, cs.height);
          content_sizes
            .insert(ent, cs)
            .expect("Could not insert content size");
        });
    }

    self
      .dispatcher
      .dispatch(&mut self.world);

    self
      .world
      .maintain();

    let data:DrawingSystemData =
      self
      .world
      .system_data();

    // Draw the things
    rasterizer
      .run_sdl2_drawing(data);
  }

  pub fn get_size(&self, ent: Entity) -> Option<(u32, u32)> {
    let elements:ReadStorage<ElementBox> =
      self
      .world
      .system_data();
    elements
      .get(ent)
      .map(|elbox| {
        (elbox.width, elbox.height)
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

use specs::prelude::*;


use super::super::components::*;


/// The mouse state.
#[derive(Clone, Debug, PartialEq)]
pub struct Mouse {
  pub x: i32,
  pub y: i32,
}


impl Default for Mouse {
  fn default() -> Self {
    Mouse {
      x: 0,
      y: 0
    }
  }
}


#[derive(Clone, Debug, PartialEq)]
pub enum Event {
  MouseOver,
  MouseMove,
  MouseOut
}


#[derive(Clone, Component, Debug, PartialEq)]
#[storage(HashMapStorage)]
pub struct Events(pub Vec<Event>);



pub struct EventSystem {
  mouse: Mouse,
  entities_mouse_is_over: Vec<u32>
}


impl EventSystem {
  pub fn new() -> EventSystem {
    EventSystem {
      mouse: Mouse::default(),
      entities_mouse_is_over: vec![]
    }
  }

  /// Determine the current events for the given entity.
  // TODO: Use an r*tree for storing entity AABBs
  fn determine_current_events(
    &mut self,
    ent: Entity,
    element_box: &ElementBox,
    mouse: &Mouse,
    mouse_has_moved: bool
  ) -> Option<Events> {
    let mut events:Vec<Event> = vec![];
    let mouse_is_over =
      element_box.left() <= mouse.x
      && element_box.right() >= mouse.x
      && element_box.top() <= mouse.y
      && element_box.bottom() >= mouse.y;
    let was_previously_over =
      self
      .entities_mouse_is_over
      .contains(&ent.id());

    if mouse_is_over {
      if was_previously_over {
        if mouse_has_moved {
          events
            .push(Event::MouseMove);
        } else {
          // Nothing has changed, do nothing
        }
      } else {
        // Mouse is now over but was not previously -
        // push an event and store the entity
        events
          .push(Event::MouseOver);
        self
          .entities_mouse_is_over
          .push(ent.id());
      }
    } else {
      if was_previously_over {
        // Mouse is no longer over but was previously -
        // push an event and remove the entity
        events
          .push(Event::MouseOut);
        let index =
          self
          .entities_mouse_is_over
          .iter()
          .zip(0..)
          .fold(
            None,
            |may_ndx, (e, ndx)| {
              if *e == ent.id() {
                Some(ndx)
              } else {
                may_ndx
              }
            }
          )
          .expect("Could not find index of entity");
        self
          .entities_mouse_is_over
          .remove(index as usize);
      }
    }

    if events.len() > 0 {
      Some(Events(events))
    } else {
      None
    }
  }
}


impl<'a> System<'a> for EventSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, Mouse>,
    ReadStorage<'a, ElementBox>,
    WriteStorage<'a, Events>
  );

  fn run(&mut self, (entities, mouse, element_boxes, mut events): Self::SystemData) {
    // Remove any events that were in the system before
    let past_events:Vec<Entity> =
      (&entities, &events)
      .join()
      .map(|(e, _)| e)
      .collect::<Vec<_>>();
    past_events
      .into_iter()
      .for_each(|ent| {
        events
          .remove(ent)
          .unwrap();
      });

    // Figure out the new events
    let mouse_has_moved =
      self.mouse != *mouse;
    for (ent, element_box) in (&entities, &element_boxes).join() {
      self
        .determine_current_events(ent, element_box, &mouse, mouse_has_moved)
        .into_iter()
        .for_each(|evs| {
          events
            .insert(ent, evs)
            .unwrap();
        });
    }

    // Update the stored mouse
    self.mouse = mouse.clone();
  }
}

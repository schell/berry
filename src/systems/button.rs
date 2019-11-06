use specs::prelude::*;

use cassowary::strength::*;

use super::super::picture::Picture;
use super::super::components::*;
use super::super::UI;
use super::super::rasterizer::Rasterizer;


const PADDING_W: u32 = 4;
const PADDING_H: u32 = 4;


#[derive(Clone, Component, Debug, PartialEq)]
#[storage(HashMapStorage)]
pub struct Button {
  text_string: String,
  background: Entity,
  foreground: Entity,
  label: Entity,
  static_constraints: Entity,
  dynamic_constraints: Entity
}


impl Button {
  fn background() -> Picture {
    Picture::new()
      .set_color(0, 0, 0, 128)
      .fill_rect(0, 0, 1, 1)
  }

  fn foreground() -> Picture {
    Picture::new()
      .set_color(255, 255, 255, 255)
      .fill_rect(0, 0, 1, 1)
  }

  fn up_text(s: &str) -> Text {
    Text::new(s)
      .color(0x33, 0x33, 0x33, 255)
  }

  fn over_text(s: &str) -> Text {
    Text::new(s)
      .color(255, 0x33, 0x33, 255)
  }

  fn down_text(s: &str) -> Text {
    Text::new(s)
      .color(255, 200, 0x33, 255)
  }

  fn new_dynamic_constraints(button: Entity, foreground: Entity, label: Entity, is_down: bool) -> (Constraints<VariableX>, Constraints<VariableY>) {
    let (inset_w, inset_h) =
      if is_down {
        (PADDING_W / 2, PADDING_H / 2)
      } else {
        (0, 0)
      };
    ( Constraints(vec![
        foreground.left().is(button.left() + inset_w),
        label.left().is(button.left() + PADDING_W + inset_w),
      ]),
      Constraints(vec![
        foreground.top().is(button.top() + inset_h),
        label.top().is(button.top() + PADDING_H + inset_h)
      ])
    )
  }
}


pub struct ButtonBuilder {
  text: Option<Text>
}


impl ButtonBuilder {
  pub fn new(s: &str) -> ButtonBuilder {
    let text =
      Button::up_text(s);
    let builder =
      ButtonBuilder {
        text: Some(text)
      };
    builder
  }

  pub fn build(self, ui: &mut UI, rasterizer: &mut Rasterizer) -> Entity {
    let mut bb =
      self;
    let text =
      bb
      .text
      .take()
      .unwrap();
    let (_, tw, th) =
      rasterizer
      .get_text(&text);
    let button =
      ElementBuilder::new()
      .build(ui);
    let foreground =
      ElementBuilder::new()
      .picture(&Button::foreground())
      .name("Button foreground")
      .build(ui);
    let background =
      ElementBuilder::new()
      .picture(&Button::background())
      .name("Button background")
      .build(ui);
    let label =
      ElementBuilder::new()
      .text(&text)
      .name("Button label")
      .build(ui);
    let total_width =
      tw + PADDING_W * 3;
    let total_height =
      th + PADDING_H * 3;
    let static_constraints =
      ElementBuilder::new()
      .name("Button static constraints")
      .x_constraints(vec![
        label.width().is(button.width() - 3 * PADDING_W),
        foreground.width().is(label.width() + PADDING_W * 2),
        background.width().is(label.width() + PADDING_W * 2),
        background.left().is(button.left() + PADDING_W),
        button.width().is(total_width).with_strength(WEAK)
      ])
      .y_constraints(vec![
        label.height().is(button.height() - 3 * PADDING_H),
        foreground.height().is(label.height() + PADDING_H * 2),
        background.height().is(label.height() + PADDING_H * 2),
        background.top().is(button.top() + PADDING_H),
        button.height().is(total_height).with_strength(WEAK)
      ])
      .z_constraints(vec![
        background.z_index().is(button.z_index()),
        foreground.z_index().is(button.z_index() + 1),
        label.z_index().is(button.z_index() + 2)
      ])
      .build(ui);
    let (xs, ys) =
      Button::new_dynamic_constraints(button, foreground, label, false);
    let dynamic_constraints =
      ElementBuilder::new()
      .name("Button dynamic constraints")
      .x_constraints(xs.0)
      .y_constraints(ys.0)
      .build(ui);

    ui.update(
      button,
      Some(
        Button{
          text_string: text.text.clone(),
          background,
          foreground,
          label,
          static_constraints,
          dynamic_constraints
        }
      )
    );

    button
  }
}


pub struct ButtonSystem;


impl ButtonSystem {
  pub fn new() -> ButtonSystem {
    ButtonSystem
  }
}


impl<'a> System<'a> for ButtonSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, Button>,
    WriteStorage<'a, Constraints<VariableX>>,
    WriteStorage<'a, Constraints<VariableY>>,
    ReadStorage<'a, Events>,
    WriteStorage<'a, Text>
  );

  fn run(&mut self, (entities, buttons, mut x_constraints, mut y_constraints, events, mut texts): Self::SystemData) {
    for (ent, button, button_events) in (&entities, &buttons, &events).join() {
      let mut update_label_constraints = |is_down: bool| {
        let (xs, ys) =
          Button::new_dynamic_constraints(ent, button.foreground, button.label, is_down);
        x_constraints
          .insert(
            button.dynamic_constraints,
            xs
          )
          .unwrap();

        y_constraints
          .insert(
            button.dynamic_constraints,
            ys
          )
          .unwrap();
      };

      if button_events.0.contains(&Event::MouseOver) || button_events.0.contains(&Event::MouseUp) {
        texts
          .insert(button.label, Button::over_text(button.text_string.as_str()))
          .unwrap();
        update_label_constraints(false);
      } else if button_events.0.contains(&Event::MouseOut) {
        texts
          .insert(button.label, Button::up_text(button.text_string.as_str()))
          .unwrap();
        update_label_constraints(false);
      } else if button_events.0.contains(&Event::MouseDown) {
        texts
          .insert(button.label, Button::down_text(button.text_string.as_str()))
          .unwrap();
        update_label_constraints(true);
      }
    }
  }
}

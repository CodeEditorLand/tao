// Copyright 2014-2021 The winit contributors
// Copyright 2021-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use tao::{
	dpi::LogicalUnit,
	event::{ElementState, Event, KeyEvent, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	keyboard::Key,
	window::{WindowBuilder, WindowSizeConstraints},
};

#[allow(clippy::single_match)]
fn main() {
	env_logger::init();

	let event_loop = EventLoop::new();

	let min_width = 400.0;

	let max_width = 800.0;

	let min_height = 200.0;

	let max_height = 400.0;

      Event::WindowEvent {
        event:
          WindowEvent::KeyboardInput {
            event:
              KeyEvent {
                logical_key: Key::Character(key_str),
                state: ElementState::Released,
                ..
              },
            ..
          },
        ..
      } => match key_str {
        "e" => {
          size_constraints.min_width = size_constraints
            .min_width
            .is_none()
            .then_some(LogicalUnit::new(min_width).into());
          window.set_inner_size_constraints(size_constraints);
        }
        "f" => {
          size_constraints.max_width = size_constraints
            .max_width
            .is_none()
            .then_some(LogicalUnit::new(max_width).into());
          window.set_inner_size_constraints(size_constraints);
        }
        "p" => {
          size_constraints.min_height = size_constraints
            .min_height
            .is_none()
            .then_some(LogicalUnit::new(min_height).into());
          window.set_inner_size_constraints(size_constraints);
        }
        "v" => {
          size_constraints.max_height = size_constraints
            .max_height
            .is_none()
            .then_some(LogicalUnit::new(max_height).into());
          window.set_inner_size_constraints(size_constraints);
        }
        _ => {}
      },
      _ => (),
    }
  });
}

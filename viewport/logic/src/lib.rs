mod inner_logic;
mod input;

use crate::input::ElementKind;
use inner_logic::InnerLogic;
use input::InputMapper;
use tools::app::{
    event::Event,
    input::{ButtonKind, KeyKind},
    App, MainLoop,
};

pub struct Viewport {
    input_mapper: InputMapper,
    inner_logic: Option<InnerLogic>,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            input_mapper: Default::default(),
            inner_logic: None,
        }
    }
}

impl MainLoop for Viewport {
    fn process(&mut self, app: &mut App) {
        while let Some(event) = app.poll_event() {
            match event {
                Event::Initialized => {
                    self.input_mapper
                        .register_action("add_point", vec![ElementKind::Button(ButtonKind::Left)]);
                    self.input_mapper
                        .register_action("left", vec![ElementKind::Key(KeyKind::Left)]);
                    self.input_mapper
                        .register_action("right", vec![ElementKind::Key(KeyKind::Right)]);
                    self.input_mapper
                        .register_action("up", vec![ElementKind::Key(KeyKind::Up)]);
                    self.input_mapper
                        .register_action("down", vec![ElementKind::Key(KeyKind::Down)]);

                    self.inner_logic = Some(InnerLogic::new(app.graphics()));
                }
                Event::Resized(width, height) => {
                    if let Some(logic) = &mut self.inner_logic {
                        logic.resized(width, height);
                    }
                }
                Event::RawInput(input) => self.input_mapper.process_raw_input(input),
            };
        }

        if let Some(logic) = &mut self.inner_logic {
            logic.process(&self.input_mapper, app.graphics());
        }
    }
}

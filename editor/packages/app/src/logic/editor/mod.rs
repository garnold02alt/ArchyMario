mod tools;

use winit::event::VirtualKeyCode;

use crate::{
    data::PropInfoContainer,
    graphics::{Canvas, Graphics},
};

use self::tools::{CameraTool, Tool};

use super::{
    camera::Camera,
    elements::ElementKind,
    input::Input,
    scene::{self, Action, Scene},
};

pub struct Editor {
    mode: ElementKind,
    tool: Box<dyn Tool>,
    grid: i32,
}

impl Editor {
    pub fn init(_ctx: Context) -> Self {
        Self {
            mode: ElementKind::Solid,
            tool: Box::new(CameraTool::default()),
            grid: 100,
        }
    }

    pub fn process(&mut self, ctx: Context) {
        let new = self.tool.process(tools::Context {
            input: ctx.input,
            graphics: ctx.graphics,
            prop_infos: ctx.prop_infos,
            camera: ctx.camera,
            scene: ctx.scene,
            delta: ctx.delta,
            mode: self.mode,
            grid: self.grid,
        });

        if let Some(new) = new {
            self.tool = new;
        }

        if self.tool.can_switch() {
            for (key, mode) in [
                (VirtualKeyCode::Key1, ElementKind::Solid),
                (VirtualKeyCode::Key2, ElementKind::Face),
                (VirtualKeyCode::Key3, ElementKind::Point),
                (VirtualKeyCode::Key4, ElementKind::Prop),
            ] {
                if ctx.input.is_key_down_once(key) {
                    if self.mode != mode {
                        ctx.scene.act(
                            scene::Context {
                                graphics: ctx.graphics,
                            },
                            Action::DeselectAll(self.mode),
                        );
                        self.mode = mode;
                    }
                    break;
                }
            }
        }
    }

    pub fn render(&self, canvas: &mut Canvas) {
        self.tool.render(canvas);
    }

    pub fn mode(&self) -> ElementKind {
        self.mode
    }
}

pub struct Context<'a> {
    pub input: &'a Input,
    pub graphics: &'a Graphics,
    pub prop_infos: &'a PropInfoContainer,
    pub camera: &'a mut Camera,
    pub scene: &'a mut Scene,
    pub delta: f32,
}

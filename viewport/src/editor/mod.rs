mod camera;
mod config;
mod solid;

use std::rc::Rc;

use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    input::{InputMapper, Trigger},
    net,
    render::{LineBatch, LineFactory, LineVertex, Scene, SolidFactory, TextureBank},
};

use self::{
    camera::{SpriteCamera, WorldCamera},
    config::GRID_MAX,
    solid::{SolidEditor, SolidEditorContext},
    ActionBinding::*,
};

macro_rules! action {
    ($name:ident Key $elem:ident) => {
        (ActionBinding::$name, Trigger::Key(VirtualKeyCode::$elem))
    };

    ($name:ident Btn $elem:ident) => {
        (ActionBinding::$name, Trigger::Button(MouseButton::$elem))
    };
}

macro_rules! actions {
    ($($name:ident $ty:ident $elem:ident,)*) => {
        #[derive(Clone, Eq, PartialEq, Hash)]
        pub enum ActionBinding {
            $($name,)*
        }

        const ACTION_DEFINITIONS: &[(ActionBinding, Trigger)] = &[
            $(action!($name $ty $elem),)*
        ];
    };
}

actions! {
    // Camera movement ////////////////

    MoveCamera           Btn Right    ,
    Forward              Key W        ,
    Backward             Key S        ,
    Left                 Key A        ,
    Right                Key D        ,
    Up                   Key E        ,
    Down                 Key Q        ,

    // Global operations //////////////

    EnableMultiSelect    Key LShift   ,
    Select               Btn Left     ,
    Deselect             Key X        ,
    SelectAll            Key A        ,
    Move                 Key G        ,
    ConfirmMove          Btn Left     ,
    AbortMove            Btn Right    ,
    GridUp               Key P        ,
    GridDown             Key O        ,
    SwitchMode           Key Tab      ,

    // Solid mode /////////////////////

    SolidMode            Key Key1     ,
    FaceMode             Key Key2     ,
    VertexMode           Key Key3     ,

    // Solid manipulation /////////////

    AddSolid             Btn Left     ,
    DeleteSolid          Key Delete   ,
    CopySolid            Key C        ,

    // Face manipulation //////////////

    SetTexture           Key T        ,

    ///////////////////////////////////
}

pub struct Editor {
    mode: EditorMode,
    solid_factory: SolidFactory,
    line_factory: LineFactory,
    world_camera: WorldCamera,
    sprite_camera: SpriteCamera,
    solid_editor: SolidEditor,
    grid_subdiv: i32,
    grid: Rc<LineBatch>,
}

impl Editor {
    pub fn init(
        solid_factory: SolidFactory,
        line_factory: LineFactory,
        input: &mut InputMapper,
    ) -> Self {
        input.define_actions(ACTION_DEFINITIONS);

        let grid = line_factory.create(&generate_grid(16, 1.0));

        Self {
            mode: EditorMode::Solid,
            solid_factory,
            line_factory,
            world_camera: Default::default(),
            sprite_camera: Default::default(),
            solid_editor: Default::default(),
            grid_subdiv: 0,
            grid,
        }
    }

    pub fn process(&mut self, dt: f32, input: &InputMapper, texture_bank: &TextureBank) {
        if input.is_active_once(SwitchMode) {
            self.mode.switch();
            net::send_packet(format!(
                r#"{{ "message": "set-editor-mode", "mode": {} }}"#,
                self.mode.as_i32()
            ));
        }

        if input.is_active_once(GridUp) && self.grid_subdiv < GRID_MAX {
            self.grid_subdiv += 1;

            let grid_length = 2.0f32.powi(self.grid_subdiv);
            let grid_cell_count = (16.0 / grid_length) as i32;

            self.grid = self
                .line_factory
                .create(&generate_grid(grid_cell_count, grid_length));

            net::send_packet(format!(
                r#"{{ "message": "set-grid-size", "size": {} }}"#,
                self.grid_subdiv
            ));
        } else if input.is_active_once(GridDown) && self.grid_subdiv > GRID_MAX {
            self.grid_subdiv -= 1;

            let grid_length = 2.0f32.powi(self.grid_subdiv);
            let grid_cell_count = (16.0 / grid_length) as i32;

            self.grid = self
                .line_factory
                .create(&generate_grid(grid_cell_count, grid_length));

            net::send_packet(format!(
                r#"{{ "message": "set-grid-size", "size": {} }}"#,
                self.grid_subdiv
            ));
        }

        self.world_camera.process(dt, input);

        match self.mode {
            EditorMode::Solid => {
                self.solid_editor.process(SolidEditorContext {
                    input,
                    world_camera: &self.world_camera,
                    solid_factory: &self.solid_factory,
                    line_factory: &self.line_factory,
                    texture_bank,
                    grid_length: 2.0f32.powi(self.grid_subdiv),
                });
            }
            EditorMode::Prop => {}
        }
    }

    pub fn render(&self, scene: &mut Scene) {
        self.world_camera.render(scene);
        self.sprite_camera.render(scene);
        self.solid_editor.render(scene, &self.world_camera);
        scene.world_pass.line_batches.push(self.grid.clone());
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.world_camera.resize_viewport(width, height);
        self.sprite_camera.resize_viewport(width, height);
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
        self.solid_editor.deselect_all();
    }

    pub fn set_solid_editor_mode(&mut self, mode: i32) {
        self.solid_editor.set_mode(mode);
    }

    pub fn save_scene(&self, texture_bank: &TextureBank) {
        self.solid_editor.save_scene(texture_bank);
    }

    pub fn set_camera_speed(&mut self, speed: f32) {
        self.world_camera.set_speed(speed);
    }

    pub fn set_grid_size(&mut self, size: i32) {
        self.grid_subdiv = size;

        let grid_length = 2.0f32.powi(self.grid_subdiv);
        let grid_cell_count = (16.0 / grid_length) as i32;

        self.grid = self
            .line_factory
            .create(&generate_grid(grid_cell_count, grid_length));
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EditorMode {
    Solid,
    Prop,
}

impl EditorMode {
    fn switch(&mut self) {
        *self = match self {
            Self::Solid => Self::Prop,
            Self::Prop => Self::Solid,
        };
    }

    fn as_i32(&self) -> i32 {
        match self {
            EditorMode::Solid => 0,
            EditorMode::Prop => 1,
        }
    }
}

fn generate_grid(cell_count: i32, cell_size: f32) -> Vec<LineVertex> {
    let half_line_len = cell_count as f32 * cell_size;
    let gray = [0.1, 0.1, 0.1, 1.0];
    let red = [0.4, 0.1, 0.1, 1.0];
    let blue = [0.1, 0.1, 0.4, 1.0];

    let mut vertices = Vec::with_capacity(cell_count as usize * 8 + 4);

    vertices.push(LineVertex {
        position: [-half_line_len, 0.0, 0.0],
        color: red,
    });

    vertices.push(LineVertex {
        position: [half_line_len, 0.0, 0.0],
        color: red,
    });

    vertices.push(LineVertex {
        position: [0.0, 0.0, -half_line_len],
        color: blue,
    });

    vertices.push(LineVertex {
        position: [0.0, 0.0, half_line_len],
        color: blue,
    });

    for sign in [-1.0, 1.0] {
        for i in 1..=cell_count {
            let pos = i as f32 * cell_size * sign;

            vertices.push(LineVertex {
                position: [-half_line_len, 0.0, pos],
                color: gray,
            });

            vertices.push(LineVertex {
                position: [half_line_len, 0.0, pos],
                color: gray,
            });

            vertices.push(LineVertex {
                position: [pos, 0.0, -half_line_len],
                color: gray,
            });

            vertices.push(LineVertex {
                position: [pos, 0.0, half_line_len],
                color: gray,
            });
        }
    }

    vertices
}

mod brush;
mod camera;
mod config;

use cgmath::vec3;
use std::marker::PhantomData;
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    input::{Input, Trigger},
    render::GraphicsWorld,
};

use self::{brush::Brush, camera::Camera};

macro_rules! action {
    ($name:literal Key $elem:ident) => {
        ($name, Trigger::Key(VirtualKeyCode::$elem))
    };

    ($name:literal Btn $elem:ident) => {
        ($name, Trigger::Button(MouseButton::$elem))
    };
}

macro_rules! actions {
    ($($name:literal $ty:ident $elem:ident,)*) => {
        &[
            $(action!($name $ty $elem),)*
        ]
    };
}

pub struct Editor<I, G> {
    camera: Camera,
    brush: Brush,

    _i: PhantomData<I>,
    _g: PhantomData<G>,
}

impl<I, G> Editor<I, G>
where
    I: Input,
    G: GraphicsWorld,
{
    pub fn init(input: &mut I, gfx: &mut G) -> Self {
        input.define_actions(actions!(
            // Camera controls

            "movecam"  Btn Right ,
            "forward"  Key W     ,
            "backward" Key S     ,
            "left"     Key A     ,
            "right"    Key D     ,
            "up"       Key E     ,
            "down"     Key Q     ,

            // Editor

            "select"   Btn Left  ,
            "confirm"  Key C     ,
        ));

        gfx.update_grid(10, 1.0);

        let nodraw =
            gfx.create_texture(&image::load_from_memory(include_bytes!("res/nodraw.png")).unwrap());

        let mut brush = Brush::new(gfx, vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0), nodraw);
        brush.rebuild(gfx);

        Self {
            camera: Camera::default(),
            brush,
            _i: PhantomData,
            _g: PhantomData,
        }
    }

    pub fn process(&mut self, input: &I, gfx: &mut G) {
        self.camera.process(input, gfx);
        self.brush.draw(gfx);

        if input.is_active_once("select") {
            self.brush
                .select_point(gfx, self.camera.position(), input.mouse_pos());
        }

        if input.is_active_once("confirm") {
            let vector = {
                let forward = self.camera.forward();
                vec3(forward.x.round(), forward.y.round(), forward.z.round())
            };

            self.brush.move_selected_points(vector);
            self.brush.clear_selected_points();
            self.brush.rebuild(gfx);
        }
    }
}

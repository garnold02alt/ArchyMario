use std::rc::Rc;

use asset::GizmoID;
use cgmath::{vec2, vec3, InnerSpace, Matrix4, Vector2, Vector3, Zero};
use instant::{Duration, Instant};
use libsm64::{LevelTriangle, Mario, MarioInput, Sm64};
use winit::event::VirtualKeyCode;

use crate::graphics::{
    structures::{GizmoInstance, MarioVertex},
    Canvas, GizmoGroup, GizmoInstances, Graphics, MarioMesh, Share,
};

use super::{camera::Camera, input::Input};

pub struct Archy64 {
    game: Sm64,
    state: Option<State>,
}

impl Default for Archy64 {
    fn default() -> Self {
        Self {
            game: Sm64::new(include_bytes!("baserom.us.z64").as_slice()).unwrap(),
            state: None,
        }
    }
}

impl Archy64 {
    pub fn init(&mut self, graphics: &Graphics, geometry: &[LevelTriangle], spawn: Vector3<i16>) {
        self.game.load_level_geometry(geometry);
        self.state = Some(State::new(graphics, &self.game, spawn));
    }

    pub fn process(&mut self, ctx: Context) {
        if let Some(state) = self.state.as_mut() {
            let now = Instant::now();
            let elapsed = now - state.last_frame;

            if elapsed >= Duration::from_millis(1000 / 30) {
                state.last_frame = now;
                let mut stick = Vector2::zero();

                if ctx.input.is_key_down(VirtualKeyCode::Up) {
                    stick.y += 1.0;
                }

                if ctx.input.is_key_down(VirtualKeyCode::Down) {
                    stick.y -= 1.0;
                }

                if ctx.input.is_key_down(VirtualKeyCode::Left) {
                    stick.x += 1.0;
                }

                if ctx.input.is_key_down(VirtualKeyCode::Right) {
                    stick.x -= 1.0;
                }

                let button_a = ctx.input.is_key_down(VirtualKeyCode::Y);
                let button_b = ctx.input.is_key_down(VirtualKeyCode::C);
                let button_z = ctx.input.is_key_down(VirtualKeyCode::X);

                let cam_look = ctx.camera.position() - state.last_pos;

                let input = MarioInput {
                    stick_x: stick.x,
                    stick_y: stick.y,
                    button_a,
                    button_b,
                    button_z,
                    cam_look_x: cam_look.x,
                    cam_look_z: cam_look.z,
                };

                let mario_state = state.mario.tick(input);

                {
                    let position = vec3(
                        mario_state.position.x / 128.0,
                        mario_state.position.y / 128.0,
                        mario_state.position.z / 128.0,
                    );
                    state.last_pos = position;

                    let velocity = vec3(
                        mario_state.velocity.x / 128.0,
                        mario_state.velocity.y / 128.0,
                        mario_state.velocity.z / 128.0,
                    );

                    let angle = mario_state.face_angle;
                    let limit = velocity.magnitude2().max(stick.magnitude2());
                    ctx.camera.mario_control(position, angle, limit < 0.01);
                }

                {
                    let geometry = state.mario.geometry();

                    let mut vertices = Vec::with_capacity(2256);
                    let mut triangles = Vec::with_capacity(2256);

                    for (i, (a, b, c)) in geometry.triangles().enumerate() {
                        [a, b, c].into_iter().for_each(|v| {
                            vertices.push(MarioVertex {
                                position: vec3(v.position.x, v.position.y, v.position.z) / 128.0,
                                normal: vec3(v.normal.x, v.normal.y, v.normal.z),
                                texcoord: vec2(v.uv.x, v.uv.y),
                                color: [v.color.r, v.color.g, v.color.b],
                            });

                            let t0 = i as u16 * 3;
                            triangles.push([t0, t0 + 1, t0 + 2]);
                        });
                    }

                    ctx.graphics
                        .write_mario_mesh(&state.mesh, &vertices, &triangles);
                }
            }
        }
    }

    pub fn render(&self, canvas: &mut Canvas) {
        if let Some(state) = self.state.as_ref() {
            canvas.draw_mario(state.mesh.clone());
        }
    }

    pub fn initialized(&self) -> bool {
        self.state.is_some()
    }
}

pub struct Context<'a> {
    pub input: &'a Input,
    pub graphics: &'a Graphics,
    pub camera: &'a mut Camera,
}

struct State {
    mario: Mario<'static>,
    mesh: Rc<MarioMesh>,
    last_frame: Instant,
    last_pos: Vector3<f32>,
}

impl State {
    fn new(graphics: &Graphics, ctx: &Sm64, spawn: Vector3<i16>) -> Self {
        let ctx = unsafe { extend_lifetime(ctx) };
        Self {
            mario: ctx.create_mario(spawn.x, spawn.y, spawn.z).unwrap(),
            mesh: Rc::new(graphics.create_mario_mesh(2256, 2256)),
            last_frame: Instant::now(),
            last_pos: Vector3::zero(),
        }
    }
}

unsafe fn extend_lifetime<'a, T>(a: &'a T) -> &'static T {
    let ptr = a as *const T;
    &*ptr
}

use std::rc::Rc;

use cgmath::{vec2, vec3, Vector2, Vector3, Zero};
use instant::{Duration, Instant};
use libsm64::{LevelTriangle, Mario, MarioInput, Sm64};
use winit::event::VirtualKeyCode;

use crate::graphics::{structures::MarioVertex, Canvas, Graphics, MarioMesh};

use super::input::Input;

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

                let input = MarioInput {
                    stick_x: stick.x,
                    stick_y: stick.y,
                    ..Default::default()
                };

                state.mario.tick(input);

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
}

pub struct Context<'a> {
    pub input: &'a Input,
    pub graphics: &'a Graphics,
}

struct State {
    mario: Mario<'static>,
    mesh: Rc<MarioMesh>,
    last_frame: Instant,
}

impl State {
    fn new(graphics: &Graphics, ctx: &Sm64, spawn: Vector3<i16>) -> Self {
        let ctx = unsafe { extend_lifetime(ctx) };
        Self {
            mario: ctx.create_mario(spawn.x, spawn.y, spawn.z).unwrap(),
            mesh: Rc::new(graphics.create_mario_mesh(2256, 2256)),
            last_frame: Instant::now(),
        }
    }
}

unsafe fn extend_lifetime<'a, T>(a: &'a T) -> &'static T {
    let ptr = a as *const T;
    &*ptr
}

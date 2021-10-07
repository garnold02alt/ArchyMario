mod color;
mod gl;
mod mesh;

pub use color::Color;
pub use mesh::{Tri, Vert};

use gl::WebGL;

pub struct Graphics {
    gl: WebGL,
}

impl Default for Graphics {
    fn default() -> Self {
        let gl = WebGL::default();
        gl.enable_depth_test();
        gl.set_clear_color(Color::new(0.25, 0.25, 0.25, 1.0));

        Self { gl }
    }
}

impl Graphics {
    pub fn resize_viewport(&self, width: i32, height: i32) {
        self.gl.set_viewport_size(width, height);
    }

    pub fn begin(&self) {
        self.gl.clear();
    }
}

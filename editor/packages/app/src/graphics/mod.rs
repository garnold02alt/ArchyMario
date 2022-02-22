mod canvas;
mod renderer;

pub mod structures;

use std::rc::Rc;

use asset::TextureID;
use gpu::{BufferUsages, Gpu};
use winit::window::Window;

pub use canvas::*;
pub use renderer::Renderer;

use self::structures::{GizmoInstance, GroundVertex, LineVertex, SolidVertex, TransformTint};

pub fn init(window: &Window) -> (Renderer, Graphics) {
    let (gpu, surface) = gpu::init(window);
    let gpu = Rc::new(gpu);
    let surface = Rc::new(surface);

    {
        let (width, height) = window.inner_size().into();
        surface.configure(&gpu, width, height);
    }

    (Renderer::new(gpu.clone(), surface), Graphics { gpu })
}

pub struct Graphics {
    gpu: Rc<Gpu>,
}

impl Graphics {
    pub fn create_line_mesh(&self, descriptor: LineMeshDescriptor) -> LineMesh {
        LineMesh {
            vertices: Rc::new(
                self.gpu
                    .create_buffer(descriptor.vertices, BufferUsages::VERTEX),
            ),
        }
    }

    pub fn create_solid_mesh(&self, descriptor: SolidMeshDescriptor) -> SolidMesh {
        SolidMesh {
            texture: descriptor.texture,
            vertices: Rc::new(
                self.gpu
                    .create_buffer(descriptor.vertices, BufferUsages::VERTEX),
            ),
            triangles: Rc::new(
                self.gpu
                    .create_buffer(descriptor.triangles, BufferUsages::INDEX),
            ),
        }
    }

    pub fn create_ground_mesh(&self, descriptor: GroundMeshDescriptor) -> GroundMesh {
        GroundMesh {
            texture: descriptor.texture,
            vertices: Rc::new(
                self.gpu
                    .create_buffer(descriptor.vertices, BufferUsages::VERTEX),
            ),
            triangles: Rc::new(
                self.gpu
                    .create_buffer(descriptor.triangles, BufferUsages::INDEX),
            ),
        }
    }

    pub fn create_prop_data(&self, data: &TransformTint) -> PropData {
        PropData {
            uniform: Rc::new(self.gpu.create_uniform(data)),
        }
    }

    pub fn create_gizmo_instances(&self, len: usize) -> GizmoInstances {
        GizmoInstances {
            buffer: Rc::new(
                self.gpu
                    .create_buffer_uninit(len, BufferUsages::VERTEX | BufferUsages::COPY_DST),
            ),
            len: len as u32,
        }
    }

    pub fn write_gizmo_instances(&self, instances: &GizmoInstances, data: &[GizmoInstance]) {
        self.gpu.write_buffer(&instances.buffer, data);
    }
}

pub struct LineMeshDescriptor<'v> {
    pub vertices: &'v [LineVertex],
}

pub struct SolidMeshDescriptor<'v, 't> {
    pub texture: TextureID,
    pub vertices: &'v [SolidVertex],
    pub triangles: &'t [[u16; 3]],
}

pub struct GroundMeshDescriptor<'v, 't> {
    pub texture: TextureID,
    pub vertices: &'v [GroundVertex],
    pub triangles: &'t [[u16; 3]],
}

pub trait Share {
    fn share(&self) -> Self;
}

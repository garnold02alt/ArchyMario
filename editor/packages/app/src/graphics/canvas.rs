use std::{collections::HashMap, rc::Rc};

use asset::{GizmoID, PropID, TextureID};
use gpu::{Buffer, Uniform};

use super::{
    structures::{
        CameraMatrices, GizmoInstance, GroundVertex, LineVertex, MarioVertex, SolidVertex,
        TransformTint,
    },
    Share,
};

#[derive(Default)]
pub struct Canvas {
    pub(super) camera_matrices: CameraMatrices,
    pub(super) line_meshes: Vec<LineMesh>,
    pub(super) solids: HashMap<TextureID, HashMap<*const Buffer<SolidVertex>, Vec<u32>>>,
    pub(super) ground_meshes: Vec<GroundMesh>,
    pub(super) props: HashMap<PropID, Vec<PropData>>,
    pub(super) gizmo_groups: Vec<GizmoGroup>,
    pub(super) gizmo_groups_no_depth: Vec<GizmoGroup>,
    pub(super) grid_len: i32,
    pub(super) mario: Option<Rc<MarioMesh>>,
}

impl Canvas {
    pub fn set_camera_matrices(&mut self, matrices: CameraMatrices) {
        self.camera_matrices = matrices;
    }

    pub fn draw_lines(&mut self, line_mesh: LineMesh) {
        self.line_meshes.push(line_mesh);
    }

    pub fn draw_solid(&mut self, textures: [TextureID; 6], mesh: &SolidMesh) {
        for (i, texture) in textures.into_iter().enumerate() {
            self.solids
                .entry(texture)
                .or_default()
                .entry(as_key(&mesh.vertices))
                .or_default()
                .push(i as u32)
        }
    }

    pub fn draw_ground(&mut self, ground_mesh: GroundMesh) {
        self.ground_meshes.push(ground_mesh);
    }

    pub fn draw_prop(&mut self, instance: PropInstance) {
        self.props
            .entry(instance.prop)
            .or_default()
            .push(instance.data);
    }

    pub fn draw_gizmos(&mut self, group: GizmoGroup) {
        self.gizmo_groups.push(group);
    }

    pub fn draw_gizmos_no_depth(&mut self, group: GizmoGroup) {
        self.gizmo_groups_no_depth.push(group);
    }

    pub fn set_grid_len(&mut self, len: i32) {
        self.grid_len = len;
    }

    pub fn draw_mario(&mut self, mario: Rc<MarioMesh>) {
        self.mario = Some(mario);
    }
}

pub struct LineMesh {
    pub(super) vertices: Rc<Buffer<LineVertex>>,
}

impl Share for LineMesh {
    fn share(&self) -> Self {
        Self {
            vertices: self.vertices.clone(),
        }
    }
}

pub struct SolidMesh {
    pub(super) vertices: Buffer<SolidVertex>,
}

pub struct MarioMesh {
    pub(super) vertices: Buffer<MarioVertex>,
    pub(super) triangles: Buffer<[u16; 3]>,
}

pub struct GroundMesh {
    pub(super) texture: TextureID,
    pub(super) vertices: Rc<Buffer<GroundVertex>>,
    pub(super) triangles: Rc<Buffer<[u16; 3]>>,
}

impl Share for GroundMesh {
    fn share(&self) -> Self {
        Self {
            texture: self.texture,
            vertices: self.vertices.clone(),
            triangles: self.triangles.clone(),
        }
    }
}

pub struct PropInstance {
    pub prop: PropID,
    pub data: PropData,
}

pub struct PropData {
    pub(super) uniform: Rc<Uniform<TransformTint>>,
}

impl Share for PropData {
    fn share(&self) -> Self {
        Self {
            uniform: self.uniform.clone(),
        }
    }
}

pub struct GizmoGroup {
    pub gizmo: GizmoID,
    pub instances: GizmoInstances,
}

pub struct GizmoInstances {
    pub(super) buffer: Rc<Buffer<GizmoInstance>>,
    pub(super) len: u32,
}

impl Share for GizmoInstances {
    fn share(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            len: self.len,
        }
    }
}

fn as_key<T>(t: &T) -> *const T {
    t as *const T
}

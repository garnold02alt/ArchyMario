use crate::{
    api::camera::Camera, intersectables::triangle::Triangle,
    textures::texture_repo::TextureRepository,
};

pub mod amdl;
pub mod gltf;

pub trait Loader {
    type C: Camera;
    fn get_triangles(&self) -> &Vec<Triangle>;
    fn get_camera(&self) -> &Self::C;
}

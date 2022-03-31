use gpu::{Gpu, Pipeline, Surface};

mod gizmo;
mod ground;
mod line;
mod mario;
mod prop;
mod solid;

pub struct Pipelines {
    pub line: Pipeline,
    pub solid: Pipeline,
    pub ground: Pipeline,
    pub prop: Pipeline,
    pub gizmo: Pipeline,
    pub mario: Pipeline,
}

impl Pipelines {
    pub fn new(gpu: &Gpu, surface: &Surface) -> Self {
        Self {
            line: line::pipeline(gpu, surface),
            solid: solid::pipeline(gpu, surface),
            ground: ground::pipeline(gpu, surface),
            prop: prop::pipeline(gpu, surface),
            gizmo: gizmo::pipeline(gpu, surface),
            mario: mario::pipeline(gpu, surface),
        }
    }
}

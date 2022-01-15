use std::iter::once;

use bytemuck::Pod;
use thiserror::Error;
use wgpu::{
    Color, CommandEncoder, IndexFormat, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, SurfaceError, SurfaceTexture, TextureView,
};

use crate::{
    data::{buffer::Buffer, texture::Texture, uniform::Uniform},
    handle::GpuHandle,
    pipelines::mesh::MeshPipeline,
};

pub struct Frame {
    texture: SurfaceTexture,
    view: TextureView,
    encoder: CommandEncoder,
}

impl Frame {
    pub fn begin_pass(&mut self, clear_color: [f32; 3]) -> RenderPass {
        RenderPass {
            inner: self.encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: &self.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: clear_color[0] as f64,
                            g: clear_color[1] as f64,
                            b: clear_color[2] as f64,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            }),
        }
    }

    pub fn draw(self, gpu: &GpuHandle) {
        gpu.queue.submit(once(self.encoder.finish()));
        self.texture.present();
    }
}

impl GpuHandle {
    pub fn next_frame(&self) -> Result<Frame, NextFrameError> {
        let texture = self.surface.get_current_texture()?;
        let view = texture.texture.create_view(&Default::default());
        let encoder = self.device.create_command_encoder(&Default::default());

        Ok(Frame {
            texture,
            view,
            encoder,
        })
    }
}

#[derive(Error, Debug)]
#[error("Couldn't get next frame: {0}")]
pub struct NextFrameError(#[from] SurfaceError);

pub struct RenderPass<'a> {
    inner: wgpu::RenderPass<'a>,
}

impl<'a> RenderPass<'a> {
    pub fn set_mesh_pipeline(&mut self, pipeline: &'a MeshPipeline) {
        self.inner.set_pipeline(&pipeline.inner);
    }

    pub fn set_uniform<T: Pod>(&mut self, slot: u32, uniform: &'a Uniform<T>) {
        self.inner.set_bind_group(slot, &uniform.group, &[]);
    }

    pub fn set_texture(&mut self, texture: &'a Texture) {
        self.inner.set_bind_group(2, &texture.group, &[]);
    }

    pub fn draw_mesh<V: Pod, T: Pod>(&mut self, vertices: &'a Buffer<V>, triangles: &'a Buffer<T>) {
        self.inner.set_vertex_buffer(0, vertices.inner.slice(..));
        self.inner
            .set_index_buffer(triangles.inner.slice(..), IndexFormat::Uint16);
        self.inner
            .draw_indexed(0..triangles.len as u32 * 3, 0, 0..1);
    }
}

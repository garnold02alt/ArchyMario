

use archyrt_core::api::fragment_render::{FragmentContext, FragmentRender};

use archyrt_core::collector::raw_collector::RawCollector;
use archyrt_core::intersectables::bvh::BVH;
use archyrt_core::intersectables::sphere::Sphere;
use archyrt_core::loaders::amdl::{amdl_textures, AMDLLoader};
use archyrt_core::renderers::path_tracer::{Material, PathTracer};
use archyrt_core::renderers::solid_renderers::albedo::AlbedoRenderer;
use archyrt_core::renderers::solid_renderers::normal::NormalRenderer;
use archyrt_core::textures::texture_repo::{self, TextureRepository};
use archyrt_core::utilities::math::{Vec2};
use archyrt_core::utilities::ray::{Intersectable, Ray};
use archyrt_core::{
    api::fragment_collector::FragmentCollector, loaders::Loader, textures::TextureID, utilities::math::Vec3,
};
use image::{Rgb, RgbImage};
use rayon::prelude::*;

pub struct SamplingRenderer<Renderer: FragmentRender + Sync + Send> {
    pub inner: Renderer,
    pub samples: usize,
}

impl<Renderer: FragmentRender + Sync + Send> FragmentRender for SamplingRenderer<Renderer> {
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3 {
        (0..self.samples)
            .into_par_iter()
            .map(|_| self.inner.render_fragment(ctx, pos))
            .reduce(Vec3::default, |a, b| a + b)
            / (self.samples as f64)
    }
}

fn main() {
    println!("Load file");
    let mut repo = TextureRepository::new();
    amdl_textures::load_into(&mut repo, "../assets").unwrap();
    let skybox_id = TextureID::new(&"skybox");
    texture_repo::exr::load_into(&mut repo, "../assets", &[(skybox_id, "skybox.exr")]).unwrap();
    let skybox = Some(skybox_id);
    let loader = AMDLLoader::from_path("../assets/portal.ascn").unwrap();
    let camera = loader.get_camera();
    let object = loader.get_triangles();
    let object = BVH::from_triangles(object).unwrap();
    let sphere_intersection = object
        .intersect(Ray {
            origin: camera.position,
            direction: camera.matrix.inner[2],
        })
        .unwrap();
    let radius = 1.0;
    let _sphere = Sphere {
        origin: sphere_intersection.get_pos() + Vec3::new(0.0, radius, 0.0),
        color: Vec3::new(0.0, 0.0, 1.0),
        radius,
        material: Material::Emissive { power: 10.0 },
    };
    let _sphere2 = Sphere {
        origin: sphere_intersection.get_pos() + Vec3::new(-radius * 4.0, radius, 0.0),
        color: Vec3::new(1.0, 0.0, 0.0),
        radius,
        material: Material::Emissive { power: 10.0 },
    };
    //let object = object.union(sphere);
    //let object = object.union(sphere2);
    println!("Render");
    let renderer = PathTracer {
        skybox,
        object: &object,
        camera: &camera,
        bounces: 5,
    };
    let renderer = SamplingRenderer {
        inner: renderer,
        samples: 5,
    };
    let albedo = AlbedoRenderer {
        object: &object,
        camera: &camera,
    };
    let normal = NormalRenderer {
        object: &object,
        camera: &camera,
    };
    let collector = RawCollector {};
    let w = 1920 / 2;
    let h = 1080 / 2;
    println!("Rendering image");
    let rt_image = collector.collect(&renderer, &repo, w, h);
    let albedo_image = collector.collect(&albedo, &repo, w, h);
    let normal_image = collector.collect(&normal, &repo, w, h);
    println!("Denoising");
    let mut output: Vec<f32> = (0..rt_image.len()).into_iter().map(|_| 0f32).collect();
    let device = oidn::Device::new();
    oidn::RayTracing::new(&device)
        .srgb(false)
        .hdr(true)
        .image_dimensions(w, h)
        .albedo_normal(&albedo_image, &normal_image)
        .clean_aux(true)
        .filter(&rt_image, &mut output)
        .unwrap();
    let mut image = RgbImage::new(w as u32, h as u32);
    for (x, y, color) in image.enumerate_pixels_mut() {
        let index = (y as usize * w + x as usize) * 3;
        let r = output[index + 0].powf(1.0 / 2.2) * 255.0;
        let g = output[index + 1].powf(1.0 / 2.2) * 255.0;
        let b = output[index + 2].powf(1.0 / 2.2) * 255.0;
        let r = r.clamp(0.0, 255.0);
        let g = g.clamp(0.0, 255.0);
        let b = b.clamp(0.0, 255.0);
        let r = r as u8;
        let g = g as u8;
        let b = b as u8;
        *color = Rgb([r, g, b]);
    }
    image.save("image.png").unwrap();
}

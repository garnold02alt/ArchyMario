use std::{collections::HashMap, fs, process::Command};

use tempfile::{tempdir, tempfile};

use crate::{
    db::{Db, DbProp, DbTexture},
    model::parse_gltf,
    props::Prop,
    report::OrBail,
    textures::Texture,
};

pub fn compile(root: &str, textures: HashMap<String, Texture>, props: Vec<Prop>) {
    mkdir(root, "out/public/textures");
    mkdir(root, "out/public/props");
    mkdir(root, "out/raytracer/textures");
    mkdir(root, "out/raytracer/props");

    let mut db = Db {
        textures: Vec::new(),
        props: Vec::new(),
    };

    for (name, texture) in textures {
        save_images(root, &name, &texture);
        db.textures.push(DbTexture {
            id: texture.id,
            url: format!("{}.png", name),
            public: texture.public,
        });
    }

    for prop in props {
        save_amdl(root, &prop);
        db.props.push(DbProp {
            id: prop.id,
            url: format!("{}.amdl", prop.name),
        });
    }

    fs::write(
        format!("{}/out/db.json", root),
        serde_json::to_string_pretty(&db).or_bail("failed to serialize `db.json`"),
    )
    .or_bail("failed to save `db.json`");
}

fn mkdir(root: &str, path: &str) {
    let path = format!("{}/{}", root, path);
    OrBail::or_bail(
        fs::create_dir_all(&path),
        &format!("couldn't create directory `{}`", path),
    );
}

fn save_images(root: &str, name: &str, texture: &Texture) {
    let large = {
        let buf =
            fs::read(&texture.path).or_bail(&format!("couldn't read file `{:?}`", &texture.path));

        image::load_from_memory(&buf).or_bail(&format!("couldn't parse texture `{}`", name))
    };

    let small = large.resize_exact(256, 256, image::imageops::FilterType::CatmullRom);

    large
        .save_with_format(
            format!("{}/out/raytracer/textures/{}.png", root, name),
            image::ImageFormat::Png,
        )
        .or_bail(&format!("couldn't save texture `{}`", name));

    small
        .save_with_format(
            format!("{}/out/public/textures/{}.png", root, name),
            image::ImageFormat::Png,
        )
        .or_bail(&format!("couldn't save texture `{}`", name));
}

fn save_amdl(root: &str, prop: &Prop) {
    let temp_dir = tempdir().or_bail("couldn't create temp dir");

    let gltf = {
        let path = temp_dir.path().join(format!("{}.gltf", prop.name));
        let path = path.to_str().unwrap().to_string();

        let py = format!(
            "import bpy; bpy.ops.export_scene.gltf(filepath='{}',export_format='GLTF_EMBEDDED')",
            path,
        );

        Command::new("blender")
            .arg(prop.source.canonicalize().unwrap().as_os_str())
            .arg("-b")
            .arg("--python-expr")
            .arg(py)
            .output()
            .or_bail(&format!("blender failed at `{}`", prop.name));

        path
    };

    let amdl = parse_gltf(&gltf, prop);
    let buf = amdl.encode().unwrap();

    fs::write(
        format!("{}/out/public/props/{}.amdl", root, prop.name),
        &buf,
    )
    .or_bail(&format!("couldn't save prop `{}`", prop.name));

    fs::write(
        format!("{}/out/raytracer/props/{}.amdl", root, prop.name),
        &buf,
    )
    .or_bail(&format!("couldn't save prop `{}`", prop.name));
}

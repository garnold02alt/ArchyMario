mod comms;
mod repo;

use std::{
    fs::{self, File},
    io::BufReader,
    sync::mpsc::channel,
    time::{SystemTime, UNIX_EPOCH},
};

use app::{builtin_resources, run, FromHost, Host, Init, Resource, ResourceKind, ToHost, Winit};
use comms::AsyncStdin;
use repo::Repo;
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let (sender, receiver) = channel();

    for resource in builtin_resources() {
        sender.send(FromHost::LoadResource(resource)).unwrap();
    }

    {
        let repo: Repo = serde_json::from_reader(BufReader::new(
            File::open("../frontend/public/assets/repo.json").unwrap(),
        ))
        .unwrap();

        for texture in repo.textures {
            let buf = fs::read(format!(
                "../frontend/public/assets/textures/{}.png",
                texture.name
            ))
            .unwrap();

            sender
                .send(FromHost::LoadResource(Resource {
                    id: texture.id,
                    buf,
                    kind: ResourceKind::Texture,
                }))
                .unwrap();
        }

        for prop in repo.props {
            let buf = fs::read(format!(
                "../frontend/public/assets/props/{}.amdl",
                prop.name
            ))
            .unwrap();

            sender
                .send(FromHost::LoadResource(Resource {
                    id: prop.id,
                    buf,
                    kind: ResourceKind::Prop,
                }))
                .unwrap();
        }
    }

    let _stdin = AsyncStdin::new(sender);

    run(Init {
        winit: winit(),
        host: Box::new(NativeHost),
        receiver,
    });
}

pub struct NativeHost;

impl Host for NativeHost {
    fn callback(&self, data: ToHost) {
        match data {
            ToHost::SceneSaved(_, scene) => {
                let stamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let fname = format!("{}.ascn", stamp);
                fs::write(&fname, &scene).unwrap();
                println!("[native-runner] saving scene `{}`", fname);
            }
            ToHost::Button(button) => {
                println!("[native-runner] button feedback for {}", button);
            }
            ToHost::PointerLocked(locked) => {
                println!("[native-runner] pointer locked: {}", locked);
            }
        }
    }
}

fn winit() -> Winit {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::default()
        .with_title("Archytex")
        .build(&event_loop)
        .unwrap();

    Winit { event_loop, window }
}

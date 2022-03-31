use serde::Deserialize;

#[derive(Deserialize)]
pub struct Repo {
    pub textures: Vec<Texture>,
    pub props: Vec<Prop>,
}

#[derive(Deserialize)]
pub struct Texture {
    pub id: u32,
    pub name: String,
    pub categories: Vec<String>,
    pub emissive: Option<String>,
}

#[derive(Deserialize)]
pub struct Prop {
    pub id: u32,
    pub name: String,
    pub categories: Vec<String>,
    pub dependencies: Vec<String>,
}

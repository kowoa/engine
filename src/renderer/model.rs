use std::path::Path;

use bevy::prelude::{Vec3, Vec2};

use super::{mesh::{Mesh, Texture, Vertex, TextureType}, shader::Shader, utils};


pub struct Model {
    meshes: Vec<Mesh>,
    directory: String,
    textures_loaded: Vec<Texture>, // stores all textures loaded so far to make sure textures aren't loaded more than once
}

impl Model {
    pub fn new(filepath: &str) -> Self {
        let (meshes, directory, textures_loaded) = Self::load_model(filepath);
        Self { meshes, directory: directory.into(), textures_loaded }
    }
    
    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            unsafe { mesh.draw(shader); }
        }
    }
    
    fn load_model(filepath: &str) -> (Vec<Mesh>, &str, Vec<Texture>) {
        // load file
        let path = Path::new(filepath);
        let directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        let obj = tobj::load_obj(path, &tobj::LoadOptions {
            single_index: true,
            ..Default::default()
        });
        
        let (models, materials) = obj.expect("failed to load OBJ file");
        let materials = materials.expect("failed to load MTL file");
        
        let mut meshes = Vec::new();
        let mut textures_loaded = Vec::new();


        for model in models {
            let mesh = &model.mesh;
            
            let mut vertices = Vec::new();

            for i in &mesh.indices {
                let i = *i as usize;
                let positions = &mesh.positions;
                let normals = &mesh.normals;
                let texcoords = &mesh.texcoords;

                let p = Vec3::new(
                    positions[3*i], positions[3*i+1], positions[3*i+2]
                );
                let n = if !normals.is_empty() {
                    Vec3::new(normals[3*i], normals[3*i+1], normals[3*i+2])
                } else {
                    Vec3::ZERO
                };
                let t = if !texcoords.is_empty() {
                    Vec2::new(texcoords[2*i], 1.0 - texcoords[2*i+1])
                } else {
                    Vec2::ZERO
                };
                
                vertices.push(Vertex {
                    position: p,
                    normal: n,
                    texcoords: t,
                });
            }
            
            
            // process material
            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];
                
                // diffuse map
                if let Some(filename) = &material.diffuse_texture {
                    let filepath = format!("{}/{}", directory, filename);
                    let tex = Self::load_material_texture(&filepath, TextureType::Diffuse, &mut textures_loaded);
                    textures.push(tex);
                }
                
                // specular map
                if let Some(filename) = &material.specular_texture {
                    let filepath = format!("{}/{}", directory, filename);
                    let tex = Self::load_material_texture(&filepath, TextureType::Specular, &mut textures_loaded);
                    textures.push(tex);
                }
                
                // normal map
                if let Some(filename) = &material.normal_texture {
                    //todo!()
                }
                
                // NOTE: no height maps
            }
            
            let indices = (0..(vertices.len() as u32)).collect();
            let mesh = unsafe { Mesh::new(vertices, indices, textures) };
            meshes.push(mesh);
        }
        
        (meshes, directory, textures_loaded)
    }
    
    fn load_material_texture(filepath: &str, tex_type: TextureType, textures_loaded: &mut Vec<Texture>) -> Texture {
        let tex = textures_loaded.iter().find(|tex| tex.filepath == filepath);
        if let Some(tex) = tex {
            return tex.clone();
        }
        
        let tex = Texture {
            id: unsafe { utils::load_texture(filepath) },
            tex_type,
            filepath: filepath.into(),
        };
        textures_loaded.push(tex.clone());
        tex
    }
}
use std::{mem::size_of, ffi::c_void};

use gl::types::GLsizeiptr;
use glam::{Vec3, Vec2};

use crate::offset_of;

use super::shader::Shader;

#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoords: Vec2,
}

#[derive(Clone)]
pub struct Texture {
    pub id: u32,
    pub tex_type: TextureType,
    pub filepath: String,
}

#[derive(Clone)]
pub enum TextureType {
    Diffuse,
    Specular,
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    textures: Vec<Texture>,
    vao: u32,
    vbo: u32,
    ebo: u32,
}

impl Mesh {
    pub unsafe fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        textures: Vec<Texture>,
    ) -> Self {
        let (vao, vbo, ebo) = {
            // create objects
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);
            
            // bind vao
            gl::BindVertexArray(vao);

            // bind vbo and insert vertex data
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<Vertex>()) as GLsizeiptr,
                &vertices[0] as *const Vertex as *const c_void,
                gl::STATIC_DRAW,
            );
            
            // bind ebo and insert index data
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<u32>()) as GLsizeiptr,
                &indices[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW,
            );
            
            // set vertex attribute pointers
            let vertex_size = size_of::<Vertex>() as i32;
            // vertex positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0, 3, gl::FLOAT, gl::FALSE, vertex_size, offset_of!(Vertex, position) as *const c_void
            );
            // vertex normals
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1, 3, gl::FLOAT, gl::FALSE, vertex_size, offset_of!(Vertex, normal) as *const c_void
            ); 
            // vertex texture coordinates
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2, 2, gl::FLOAT, gl::FALSE, vertex_size, offset_of!(Vertex, texcoords) as *const c_void
            );
            
            // cleanup
            gl::BindVertexArray(0); // vao
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // vbo
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); // ebo
            
            (vao, vbo, ebo)
        };
        
        
        Self {
            vertices,
            indices,
            textures,
            vao,
            vbo,
            ebo,
        }
    }
    
    pub unsafe fn draw(&self, shader: &Shader) {
        let mut diffuse_num = 1;
        let mut specular_num = 1;

        for (i, tex) in self.textures.iter().enumerate() {
            // activate proper texture unit before binding
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);
            
            // retrieve texture number
            let (name, num) = match tex.tex_type {
                TextureType::Diffuse => {
                    diffuse_num += 1;
                    ("diffuse_tex", diffuse_num)
                },
                TextureType::Specular => {
                    specular_num += 1;
                    ("specular_tex", specular_num)
                },
            };
            
            // set sampler2D uniform and bind texture
            shader.set_int(format!("material.{name}{num}").as_str(), i as i32);
            gl::BindTexture(gl::TEXTURE_2D, tex.id);
        } 
        
        // draw mesh
        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
        
        // cleanup
        gl::BindVertexArray(0);
        gl::ActiveTexture(gl::TEXTURE0);
    }
}
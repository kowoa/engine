use bevy_ecs::system::{Commands, Res};

use super::{utils::get_gl_string, create_shader, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE, VERTEX_DATA, Renderer};

pub fn init(mut commands: Commands) {
    unsafe {
        if let Some(renderer) = get_gl_string(gl::RENDERER) {
            println!("Running on {}", renderer.to_string_lossy());
        }
        if let Some(version) = get_gl_string(gl::VERSION) {
            println!("OpenGL Version {}", version.to_string_lossy());
        }

        if let Some(shaders_version) = get_gl_string(gl::SHADING_LANGUAGE_VERSION) {
            println!("Shaders version on {}", shaders_version.to_string_lossy());
        }

        let vertex_shader = create_shader(gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
        let fragment_shader = create_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

        let program = gl::CreateProgram();

        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);

        gl::LinkProgram(program);

        gl::UseProgram(program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let mut vao = std::mem::zeroed();
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo = std::mem::zeroed();
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            VERTEX_DATA.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        let pos_attrib = gl::GetAttribLocation(program, b"position\0".as_ptr() as *const _);
        let color_attrib = gl::GetAttribLocation(program, b"color\0".as_ptr() as *const _);
        gl::VertexAttribPointer(
            pos_attrib as gl::types::GLuint,
            2,
            gl::FLOAT,
            0,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            std::ptr::null(),
        );
        gl::VertexAttribPointer(
            color_attrib as gl::types::GLuint,
            3,
            gl::FLOAT,
            0,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            (2 * std::mem::size_of::<f32>()) as *const () as *const _,
        );
        gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
        gl::EnableVertexAttribArray(color_attrib as gl::types::GLuint);
        
        commands.insert_resource(Renderer {
            program, vao, vbo
        });
    }
}

pub fn draw(render_objs: Res<Renderer>) {
    unsafe {
        gl::UseProgram(render_objs.program);

        gl::BindVertexArray(render_objs.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, render_objs.vbo);

        gl::ClearColor(0.1, 0.1, 0.1, 0.9);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}

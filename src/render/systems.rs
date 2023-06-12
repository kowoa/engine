use std::{ptr, mem::size_of, ffi::c_void, path::Path};

use bevy_ecs::{system::{Query, Res, Commands}};
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use glam::{Vec3, Mat4, Mat3};

use crate::{common::Time, window::WindowInfo};

use super::{utils::load_texture, Model, camera::Camera, RenderObjs, shader::Shader};

pub fn init(mut commands: Commands) {
    let (obj_vao, light_vao, num_elems) = unsafe {
        // vertices for positions
        let positions: [f32; 288] = [
            // positions      // normals        // texture coords
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
             0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
             0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
             0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
            -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
       
            -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
             0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
             0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
             0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
            -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
            -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
       
            -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
            -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
            -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
            -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
            -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
            -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
       
             0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
             0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
             0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
             0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
             0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
             0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
       
            -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
             0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
             0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
             0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
            -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
            -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
        
            -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
             0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
             0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
             0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
            -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
            -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
        ];
        
        // indices for position vertices
        let indices: [i32; 36] = core::array::from_fn(|i| i as i32);

        // vertex buffer object
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        
        // element buffer object
        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);
        
        // vertex array object for object
        let obj_vao = {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (positions.len() * size_of::<GLfloat>()) as GLsizeiptr,
                &positions[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<GLfloat>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW,
            );

            // link vertex attributes
            let stride = 8 * size_of::<GLfloat>() as GLsizei;
            gl::VertexAttribPointer(
                0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null()
            ); // positions
            gl::VertexAttribPointer(
                1, 3, gl::FLOAT, gl::FALSE, stride, (3 * size_of::<GLfloat>()) as *const c_void
            ); // normals
            gl::VertexAttribPointer(
                2, 2, gl::FLOAT, gl::FALSE, stride, (6 * size_of::<GLfloat>()) as *const c_void
            ); // texture coords
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);

            vao
        };

        // vertex array object for light
        let light_vao = {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (positions.len() * size_of::<GLfloat>()) as GLsizeiptr,
                &positions[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<GLfloat>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW,
            );

            // link vertex attributes
            let stride = 8 * size_of::<GLfloat>() as GLsizei;
            gl::VertexAttribPointer(
                0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null()
            ); // positions
            gl::EnableVertexAttribArray(0);

            vao
        };

        // cleanup
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        // draw in wireframe polygons
        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        
        (obj_vao, light_vao, indices.len() as u32)
    };
    
    let diffuse_map = unsafe { load_texture("assets/container2.png") };
    
    let specular_map = unsafe { load_texture("assets/container2_specular.png") };
    
    let emission_map = unsafe { load_texture("assets/matrix.jpg") };
    
    let obj_shader = Shader::new(
        "shaders/object.vert",
        "shaders/object.frag",
    );
    
    let light_shader = Shader::new(
        "shaders/light.vert",
        "shaders/light.frag",
    );
    
    let model = Model::new("assets/backpack/backpack.obj");
    
    commands.insert_resource(RenderObjs {
        obj_vao,
        light_vao,
        obj_shader,
        light_shader,
        num_elems,
        diffuse_map,
        specular_map,
        emission_map,
        model,
    });
}


pub fn draw(
    cam_qry: Query<&Camera>,
    render_objs: Res<RenderObjs>,
    window_params: Res<WindowInfo>,
    time: Res<Time>,
) {
    let cam = cam_qry.single();
    
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        //gl::Enable(gl::CULL_FACE);
        //gl::CullFace(gl::BACK);
        //gl::FrontFace(gl::CW);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        //gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        
        let point_light_positions = [
            Vec3::new(0.7, 0.2, 2.0),
            Vec3::new(2.3, -3.3, -4.0),
            Vec3::new(-4.0, 2.0, -12.0),
            Vec3::new(0.0, 0.0, -3.0),
        ];

        { // draw the light cubes for point lights
            let shader = &render_objs.light_shader;
            shader.activate();
            gl::BindVertexArray(render_objs.light_vao);
            
            // vertex shader uniforms
            let view = cam.get_view_mat();
            let proj = Mat4::perspective_rh(
                45.0f32.to_radians(),
                window_params.width as f32 / window_params.height as f32,
                0.1,
                100.0,
            );
            shader.set_mat4("view", view);
            shader.set_mat4("proj", proj);
            
            for pos in point_light_positions {
                let model = Mat4::from_translation(pos) * Mat4::from_scale(Vec3::new(0.25, 0.25, 0.25));
                shader.set_mat4("model", model);
                gl::DrawElements(gl::TRIANGLES, render_objs.num_elems as i32, gl::UNSIGNED_INT, ptr::null());
            }
        }
        

        { // draw the object cube(s)
            // bind textures
            let shader = &render_objs.obj_shader;
            shader.activate();
            
            // vertex shader uniforms
            let model = Mat4::from_translation(Vec3::ZERO);
            let view = cam.get_view_mat();
            let proj = Mat4::perspective_rh(
                cam.zoom.to_radians(),
                window_params.width as f32 / window_params.height as f32,
                0.1,
                100.0,
            );
            let normal_mat = {
                let mat = (view * model)
                .inverse()
                .transpose();
                // cast Matrix4 to Matrix3
                Mat3::from_mat4(mat)
            };
            shader.set_mat4("model", model);
            shader.set_mat4("view", view);
            shader.set_mat4("proj", proj);
            shader.set_mat3("normal_mat", normal_mat);
            
            // fragment shader uniforms
            // material textures are handled by the mesh's draw method
            shader.set_float("material.shininess", 64.0);
            
            shader.set_float("time", time.current);

            // lights
            shader.set_vec3("dir_light.direction", -0.2, -1.0, -0.3);
            shader.set_vec3("dir_light.ambient", 0.05, 0.05, 0.05);
            shader.set_vec3("dir_light.diffuse", 0.4, 0.4, 0.4);
            shader.set_vec3("dir_light.specular", 0.5, 0.5, 0.5);
            
            for (i, pos) in point_light_positions.iter().enumerate() {
                shader.set_vec3(format!("point_lights[{i}].position").as_str(), pos.x, pos.y, pos.z);
                shader.set_vec3(format!("point_lights[{i}].ambient").as_str(), 0.05, 0.05, 0.05);
                shader.set_vec3(format!("point_lights[{i}].diffuse").as_str(), 0.8, 0.8, 0.8);
                shader.set_vec3(format!("point_lights[{i}].specular").as_str(), 1.0, 1.0, 1.0);
                shader.set_float(format!("point_lights[{i}].att_constant").as_str(), 1.0);
                shader.set_float(format!("point_lights[{i}].att_linear").as_str(), 0.09);
                shader.set_float(format!("point_lights[{i}].att_quadratic").as_str(), 0.032);
            }

            shader.set_vec3("spot_light.position", 0.0, 0.0, 0.0);
            shader.set_vec3("spot_light.direction", 0.0, 0.0, -1.0);
            shader.set_float("spot_light.cutoff_angle_cos", 12.5f32.to_radians().cos());
            shader.set_float("spot_light.outer_cutoff_angle_cos", 15.0f32.to_radians().cos());
            shader.set_vec3("spot_light.ambient", 0.0, 0.0, 0.0);
            shader.set_vec3("spot_light.diffuse", 1.0, 1.0, 1.0);
            shader.set_vec3("spot_light.specular", 1.0, 1.0, 1.0);
            shader.set_float("spot_light.att_constant", 1.0);
            shader.set_float("spot_light.att_linear", 0.09);
            shader.set_float("spot_light.att_quadratic", 0.032);
            
            render_objs.model.draw(shader);
        }
    }
}
use std::{path::Path, ffi::{c_void, CStr}};

use image::DynamicImage;

pub unsafe fn load_texture(filepath: &str) -> u32 {
    let mut texture = 0;
    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture);
    
    // load image
    let img = image::open(Path::new(filepath))
        .expect("failed to load texture");
    let format = match &img {
        DynamicImage::ImageLuma8(_) => gl::RED,
        DynamicImage::ImageLumaA8(_) => gl::RG,
        DynamicImage::ImageRgb8(_) => gl::RGB,
        DynamicImage::ImageRgba8(_) => gl::RGBA,
        other => {
            println!("no format found for {:?}", other);
            gl::RGB
        }
    };
    let img = img.flipv(); // flip loaded texture on the y-axis
    let data = img.as_bytes();
    
    // create texture
    gl::TexImage2D(gl::TEXTURE_2D,
        0,
        format as i32,
        img.width() as i32,
        img.height() as i32,
        0,
        format,
        gl::UNSIGNED_BYTE,
        data.as_ptr() as *const u8 as *const c_void
    );

    // set texture wrapping params
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S,
        if format == gl::RGBA { gl::CLAMP_TO_EDGE } else { gl::REPEAT } as i32
    );
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_R,
        if format == gl::RGBA { gl::CLAMP_TO_EDGE } else { gl::REPEAT } as i32
    );

    // set texture filtering params
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    gl::GenerateMipmap(gl::TEXTURE_2D);

    // cleanup
    gl::BindTexture(gl::TEXTURE_2D, 0);
        
    texture
}

pub fn get_gl_string(variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl::GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

/// Get offset to struct member, similar to `offset_of` in C/C++
/// From https://stackoverflow.com/questions/40310483/how-to-get-pointer-offset-in-bytes/40310851#40310851
#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(std::ptr::null() as *const $ty)).$field as *const _ as usize
    }
}
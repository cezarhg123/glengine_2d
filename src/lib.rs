#![allow(unused)] // only for dev, not for release

pub mod gl;
pub mod shader;
pub mod vao;
pub mod buffer;
pub mod uniform;
pub mod texture;


pub use gl::load_with;
pub use shader::Shader;
pub use vao::VAO;


/// forcably changes `cloned` to true
fn change_clone_status(cloned: &bool) {
    let cloned = cloned as *const bool;
    let cloned = cloned.cast_mut();
    unsafe {
        *cloned = true;
    }
}

pub fn draw_arrays(count: u32) {
    unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, count as i32);
    }
}

pub fn draw_element(count: u32) {
    unsafe {
        gl::DrawElements(gl::TRIANGLES, count as i32, gl::UNSIGNED_INT, 0 as *const _);
    }
}

pub fn clear(r: f32, g: f32, b: f32) {
    unsafe {
        gl::ClearColor(r, g, b, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

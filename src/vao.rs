use crate::{gl, change_clone_status};

#[derive(Debug)]
pub struct VAO {
    id: u32,
    cloned: bool
}

impl VAO {
    pub fn new() -> VAO {
        let mut id = 0;
        
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        VAO {
            id,
            cloned: false
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn set_attrib(&self, layout: u32, components: u32, type_: u32, size_in_bytes: u32, offset_in_bytes: u32) {
        unsafe {
            gl::VertexAttribPointer(layout, components as i32, type_, gl::FALSE, size_in_bytes as i32, offset_in_bytes as *const _);
            gl::EnableVertexAttribArray(layout);
        }
    }
}

impl Clone for VAO {
    fn clone(&self) -> Self {
        change_clone_status(&self.cloned);

        Self {
            id: self.id,
            cloned: false
        }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        if !self.cloned {
            unsafe {
                gl::DeleteVertexArrays(1, &self.id);
            }
        }
    }
}

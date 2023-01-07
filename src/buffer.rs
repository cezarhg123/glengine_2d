use std::mem::size_of_val;

use crate::{gl, change_clone_status};

#[derive(Debug)]
pub struct Buffer {
    type_: u32,
    id: u32,
    cloned: bool
}

impl Buffer {
    pub fn new(buffer_type: u32) -> Buffer {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Buffer {
            type_: buffer_type,
            id,
            cloned: false
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.type_, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.type_, 0);
        }
    }

    pub fn set_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(self.type_, size_of_val(data) as isize, data.as_ptr().cast(), gl::STATIC_DRAW);
        }
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        change_clone_status(&self.cloned);
        Self {
            type_: self.type_,
            id: self.id,
            cloned: false
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if !self.cloned {
            unsafe {
                gl::DeleteBuffers(1, &self.id);
            }
        }
    }
}

use std::io::Cursor;

use image::DynamicImage;

use crate::{change_clone_status, gl};

/// Can be created with `from()`
#[derive(Debug)]
pub struct Texture {
    image: DynamicImage,
    id: u32,
    cloned: bool
}

impl Texture {
    fn new(image: DynamicImage) -> Texture {
        let mut id = 0;
        
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, image.width() as i32, image.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, image.as_bytes().as_ptr().cast());
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture {
            image,
            id,
            cloned: false
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl From<&str> for Texture {
    fn from(path: &str) -> Self {
        let image = image::load(
            Cursor::new(std::fs::read(path).unwrap()),
            image::ImageFormat::Png
        ).unwrap().flipv();

        Self::new(image)
    }
}

impl From<DynamicImage> for Texture {
    /// Flips image vertically because OpenGL weird
    fn from(image: DynamicImage) -> Self {
        let image = image.flipv();

        Self::new(image)
    }
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        change_clone_status(&self.cloned);

        Self {
            image: self.image.clone(),
            id: self.id,
            cloned: false
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if !self.cloned {
            unsafe {
                gl::DeleteTextures(1, &self.id);
            }
        }
    }
}

use crate::{gl, change_clone_status, uniform::Uniform};

#[derive(Debug)]
pub struct Shader {
    id: u32,
    cloned: bool
}

impl Shader {
    pub fn new(vertex_source: &str, frag_source: &str) -> Shader {
        let mut id = 0;
        
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &(vertex_source.as_ptr().cast()), &(vertex_source.len().try_into().unwrap()));
            gl::CompileShader(vertex_shader);
            
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &(frag_source.as_ptr().cast()), &(frag_source.len().try_into().unwrap()));
            gl::CompileShader(fragment_shader);

            id = gl::CreateProgram();
            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);
            gl::LinkProgram(id);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Shader {
            id,
            cloned: false
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_uniform(&self, uniform: Uniform) {
        unsafe {
            use crate::uniform::UniformType::*;
            match uniform.uniform_type {
                Int(i) => {
                    gl::Uniform1i(
                        gl::GetUniformLocation(self.id, uniform.name.as_ptr().cast()),
                        i
                    );
                }
                Float(f) => {
                    gl::Uniform1f(
                        gl::GetUniformLocation(self.id, uniform.name.as_ptr().cast()),
                        f
                    );
                }
                Vec2(f1, f2) => {
                    gl::Uniform2f(gl::GetUniformLocation(self.id, uniform.name.as_ptr().cast()), f1, f2);
                }
                Vec3(f1, f2, f3) => {
                    gl::Uniform3f(gl::GetUniformLocation(self.id, uniform.name.as_ptr().cast()), f1, f2, f3);
                }
                Vec4(f1, f2, f3, f4) => {
                    gl::Uniform4f(gl::GetUniformLocation(self.id, uniform.name.as_ptr().cast()), f1, f2, f3, f4);
                }
            }
        }
    }
}

impl Clone for Shader {
    fn clone(&self) -> Self {
        change_clone_status(&self.cloned);
        Self {
            id: self.id,
            cloned: false
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        if !self.cloned {
            unsafe {
                gl::DeleteProgram(self.id);
            }
        }
    }
}

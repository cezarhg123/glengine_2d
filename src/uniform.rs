
/// use `uniform()` function to create instead
#[derive(Debug, Clone, PartialEq)]
pub struct Uniform {
    pub name: String,
    pub uniform_type: UniformType
}

/// official way of creating `Uniform`. appends \0 to the end of `name`
pub fn uniform(name: &str, uniform_type: UniformType) -> Uniform {
    let name = name.to_string() + "\0";
    Uniform {
        name,
        uniform_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UniformType {
    Int(i32),
    Float(f32),
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    Vec4(f32, f32, f32, f32)
}
struct Vertex(f32, f32);
struct VertexUV(f32, f32, f32, f32);

#[test]
fn color_no_ebo() {
    use std::mem::size_of;
    use glfw::Context;
    use crate::{gl, Shader, VAO, buffer::Buffer, draw_arrays, draw_element, texture::Texture, uniform::{Uniform, uniform, UniformType}};

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(800, 800, "Color Test", glfw::WindowMode::Windowed).unwrap();
    window.set_all_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));

    let shader = Shader::new(include_str!("../shaders/color.vert"), include_str!("../shaders/color.frag"));

    let vao = VAO::new();
    vao.bind();

    let vbo = Buffer::new(gl::ARRAY_BUFFER);
    vbo.bind();
    vbo.set_data([
        Vertex(-0.5, -0.5),
        Vertex(0.5, -0.5),
        Vertex(0.0, 0.5)
    ].as_slice());

    vao.set_attrib(0, 2, gl::FLOAT, size_of::<Vertex>() as u32, 0);

    vao.unbind();
    vbo.unbind();
    
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        shader.use_program();
        vao.bind();
        draw_arrays(3);

        window.swap_buffers();
        glfw.poll_events();

        for (_, _event) in glfw::flush_messages(&events) {
        }
    }
}

#[test]
fn color_ebo() {
    use std::mem::size_of;
    use glfw::Context;
    use crate::{gl, Shader, VAO, buffer::Buffer, draw_arrays, draw_element, texture::Texture, uniform::{Uniform, uniform, UniformType}};

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(800, 800, "Color With EBO Test", glfw::WindowMode::Windowed).unwrap();
    window.set_all_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));

    let shader = Shader::new(include_str!("../shaders/color.vert"), include_str!("../shaders/color.frag"));

    let vao = VAO::new();
    vao.bind();

    let vbo = Buffer::new(gl::ARRAY_BUFFER);
    vbo.bind();
    vbo.set_data([
        Vertex(-0.5, -0.5),
        Vertex(-0.5, 0.5),
        Vertex(0.5, 0.5),
        Vertex(0.5, -0.5),
    ].as_slice());
    
    let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
    ebo.bind();
    ebo.set_data([
        0, 2, 1,
        0, 3, 2
    ].as_slice());

    vao.set_attrib(0, 2, gl::FLOAT, size_of::<Vertex>() as u32, 0);

    vao.unbind();
    vbo.unbind();
    ebo.unbind();
    
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        shader.use_program();
        vao.bind();
        draw_element(6);

        window.swap_buffers();
        glfw.poll_events();

        for (_, _event) in glfw::flush_messages(&events) {
        }
    }
}

#[test]
fn image() {
    use std::mem::size_of;
    use glfw::Context;
    use crate::{gl, Shader, VAO, buffer::Buffer, draw_arrays, draw_element, texture::Texture, uniform::{Uniform, uniform, UniformType}};

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(800, 800, "Color With EBO Test", glfw::WindowMode::Windowed).unwrap();
    window.set_all_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));

    let shader = Shader::new(include_str!("../shaders/image.vert"), include_str!("../shaders/image.frag"));

    let vao = VAO::new();
    vao.bind();

    let vbo = Buffer::new(gl::ARRAY_BUFFER);
    vbo.bind();
    vbo.set_data([
        VertexUV(-0.5, -0.5, 0.0, 0.0),
        VertexUV(-0.5, 0.5, 0.0, 1.0),
        VertexUV(0.5, 0.5, 1.0, 1.0),
        VertexUV(0.5, -0.5, 1.0, 0.0),
    ].as_slice());
    
    let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
    ebo.bind();
    ebo.set_data([
        0, 2, 1,
        0, 3, 2
    ].as_slice());

    vao.set_attrib(0, 2, gl::FLOAT, size_of::<VertexUV>() as u32, 0);
    vao.set_attrib(1, 2, gl::FLOAT, size_of::<VertexUV>() as u32, 8);

    
    vao.unbind();
    vbo.unbind();
    ebo.unbind();

    let tex = Texture::from("textures/test.png");
    shader.use_program();
    shader.set_uniform(uniform("tex", UniformType::Int(gl::TEXTURE0 as i32)));

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        shader.use_program();
        vao.bind();
        tex.bind();
        draw_element(6);

        window.swap_buffers();
        glfw.poll_events();

        for (_, _event) in glfw::flush_messages(&events) {
        }
    }
}

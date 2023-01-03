use glfw::Window;

include!("bindings.rs");

pub fn load_gl(window: &mut Window) {
    load_with(|s| window.get_proc_address(s));
}

use glfw::{Action, Context, Key};
use rust_animation::play::Play;
use rust_animation::stage::Stage;
use rust_animation::actor::LayoutMode;
use std::sync::mpsc::Receiver;

use crate::actor::ActorEvent;

pub async fn launch_koch() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(1920, 1080, "Image Viewer", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut play = Play::new("Animation test".to_string());
    play.initialize();
    let mut stage = Stage::new("stage".to_string(), 1920, 1080, LayoutMode::UserDefine, Some(Box::new(ActorEvent::new())));
    stage.set_visible(true);

    stage.set_needs_layout();
    play.add_stage(stage);

    while !window.should_close() {
        process_events(&mut window, &events, &mut play);
        play.render();
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, play: &mut Play) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(key, _, action, _) => {
                match (key, action) {
                    (Key::Escape, Action::Press) => window.set_should_close(true),
                    (Key::Up, Action::Press) => play.handle_input(unsafe { ::std::mem::transmute(Key::Up) }),
                    (Key::Down, Action::Press) => play.handle_input(unsafe { ::std::mem::transmute(Key::Down) }),
                    (Key::Left, Action::Press) => play.handle_input(unsafe { ::std::mem::transmute(Key::Left) }),
                    (Key::Right, Action::Press) => play.handle_input(unsafe { ::std::mem::transmute(Key::Right) }),
                    (Key::Enter, Action::Press) => play.handle_input(unsafe { ::std::mem::transmute(Key::Enter) }),
                    (Key::Space, Action::Press) => play.handle_input(unsafe { ::std::mem::transmute(Key::Space) }),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
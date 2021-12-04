use winit::{event_loop::EventLoop, window::WindowBuilder};

pub struct Window(pub winit::window::Window);

impl Window {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("Fornjot")
            .with_maximized(true)
            .with_decorations(true)
            .with_transparent(false)
            .build(event_loop)
            .unwrap();

        Self(window)
    }
}

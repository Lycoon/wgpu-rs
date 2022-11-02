use winit::platform::windows::WindowBuilderExtWindows;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Theme, WindowBuilder},
};

fn main() {
    let event_loop: EventLoop<()> = EventLoop::new();

    /* Window settings */
    let _window = WindowBuilder::new()
        .with_title("Trunkee")
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)
        .unwrap();
        
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

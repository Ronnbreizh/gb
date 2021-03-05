use gb::Gameboy;

//fn main() {
//    use glium::{glutin, Surface};
//
//    let event_loop = glutin::event_loop::EventLoop::new();
//    let wb = glutin::window::WindowBuilder::new();
//    let cb = glutin::ContextBuilder::new();
//    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
//
//    event_loop.run(move |ev, _, control_flow| {
//        // Clear
//        let mut target = display.draw();
//        target.clear_color(0.0, 0.0, 1.0, 1.0);
//        target.finish().unwrap();
//
//        // Next time update
//        let next_frame_time = std::time::Instant::now() +
//            std::time::Duration::from_nanos(16_666_667);
//
//        // On update
//        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
//        // event :
//        match ev {
//            glutin::event::Event::WindowEvent { event, .. } => match event {
//                glutin::event::WindowEvent::CloseRequested => {
//                    *control_flow = glutin::event_loop::ControlFlow::Exit;
//                    return;
//                },
//                glutin::event::WindowEvent::ReceivedCharacter(c) => println!("Char : {}", c),
//                _ => return,
//            },
//            _ => (),
//        }
//    });
//}


fn main() {
    // load ROM
    let mut gameboy = Gameboy::new();

    gameboy.run();
}

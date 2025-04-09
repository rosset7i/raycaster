use consts::{
    FRAGMENT_SHADER_SRC, VERTEX_SHADER,
    errors::WINDOW_ERROR,
    window::{HEIGHT, WIDTH},
};
use glium::{
    Program, Surface, VertexBuffer,
    backend::glutin::SimpleWindowBuilder,
    index::{NoIndices, PrimitiveType},
    uniform,
    winit::{
        event::{Event, WindowEvent},
        event_loop::EventLoop,
    },
};
use vertexes::draw;

mod consts;
mod vertexes;

fn main() {
    let event_loop = EventLoop::builder()
        .build()
        .unwrap_or_else(|err| panic!("{} {}", WINDOW_ERROR, err));

    let (window, display) = SimpleWindowBuilder::new()
        .with_inner_size(WIDTH, HEIGHT)
        .build(&event_loop);

    let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER_SRC, None).unwrap();
    let indices = NoIndices(PrimitiveType::TrianglesList);

    let mut t: f32 = 0.0;
    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    t += 0.01;
                    let x_off = t.sin() * 0.5;

                    let drawing = draw();
                    let vertex_buffer = VertexBuffer::new(&display, &drawing).unwrap();
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);
                    target
                        .draw(
                            &vertex_buffer,
                            indices,
                            &program,
                            &uniform! {x: x_off},
                            &Default::default(),
                        )
                        .unwrap();
                    target.finish().unwrap();
                }
                _ => (),
            },
            Event::AboutToWait => window.request_redraw(),
            _ => (),
        };
    });
}

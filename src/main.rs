use glium::{
    Program, Surface, VertexBuffer,
    backend::glutin::SimpleWindowBuilder,
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    uniforms::EmptyUniforms,
    winit::{
        event::{Event, WindowEvent},
        event_loop::EventLoop,
    },
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

fn main() {
    let event_loop = EventLoop::builder().build().unwrap();
    let (_window, display) = SimpleWindowBuilder::new()
        .with_inner_size(WIDTH, HEIGHT)
        .build(&event_loop);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let triangle = draw();
    let indices = NoIndices(PrimitiveType::LineLoop);
    let vertex_buffer = VertexBuffer::new(&display, &triangle).unwrap();

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);
                    target
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &EmptyUniforms,
                            &Default::default(),
                        )
                        .unwrap();
                    target.finish().unwrap();
                }
                _ => (),
            },
            _ => (),
        };
    });
}

fn draw() -> Vec<Vertex> {
    let vertex_1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex_2 = Vertex {
        position: [-0.5, 0.5],
    };
    let vertex_3 = Vertex {
        position: [0.5, 0.5],
    };
    let vertex_4 = Vertex {
        position: [0.5, -0.5],
    };

    vec![vertex_1, vertex_2, vertex_3, vertex_4]
}

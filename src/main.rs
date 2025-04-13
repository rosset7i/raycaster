use std::f32::consts::PI;

use consts::{
    FRAGMENT_SHADER_SRC, VERTEX_SHADER,
    errors::{BUFFER_ERROR, DRAWING_ERROR, PROGRAM_ERROR, WINDOW_ERROR},
    window::{HEIGHT, WIDTH},
};
use glam::Mat4;
use glium::{
    Display, Program, Surface, VertexBuffer,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    index::{NoIndices, PrimitiveType},
    uniform,
    winit::{
        event::{ElementState, Event, WindowEvent},
        event_loop::{ActiveEventLoop, EventLoop},
        keyboard::{KeyCode, PhysicalKey},
    },
};
use vertexes::{Vertex, draw_map, draw_player};

mod consts;
mod vertexes;

fn main() {
    //TODO: This should be struct,
    //Position should be a Enum
    let mut position_x: f32 = 500.0;
    let mut position_y: f32 = 500.0;
    let mut player_angle: f32 = 0.0;
    let mut position_delta_x: f32 = player_angle.cos() * 5.0;
    let mut position_delta_y: f32 = player_angle.sin() * 5.0;

    let event_loop = EventLoop::builder()
        .build()
        .unwrap_or_else(|err| panic!("{WINDOW_ERROR} {err}"));

    let (window, display) = SimpleWindowBuilder::new()
        .with_inner_size(WIDTH, HEIGHT)
        .build(&event_loop);

    let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER_SRC, None)
        .unwrap_or_else(|err| panic!("{PROGRAM_ERROR} {err}"));

    let indices = NoIndices(PrimitiveType::TrianglesList);
    let line_indices = NoIndices(PrimitiveType::LinesList);

    let ortho = Mat4::orthographic_rh_gl(0.0, WIDTH as f32, HEIGHT as f32, 0.0, -1.0, 1.0);

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => handle_window_event(
                event,
                window_target,
                &display,
                indices,
                line_indices,
                &program,
                &mut position_x,
                &mut position_y,
                ortho,
                &mut position_delta_x,
                &mut position_delta_y,
                &mut player_angle,
            ),
            Event::AboutToWait => window.request_redraw(),
            _ => (),
        };
    });
}

fn handle_window_event(
    event: WindowEvent,
    window_target: &ActiveEventLoop,
    display: &Display<WindowSurface>,
    indices: NoIndices,
    line_indices: NoIndices,
    program: &Program,
    position_x: &mut f32,
    position_y: &mut f32,
    ortho: Mat4,
    positon_delta_x: &mut f32,
    positon_delta_y: &mut f32,
    angle: &mut f32,
) {
    match event {
        WindowEvent::CloseRequested => window_target.exit(),
        WindowEvent::RedrawRequested => redraw(
            display,
            indices,
            line_indices,
            program,
            position_x,
            position_y,
            ortho,
            positon_delta_x,
            positon_delta_y,
        ),
        WindowEvent::KeyboardInput { event, .. } => {
            if event.state != ElementState::Pressed {
                return;
            }

            if let PhysicalKey::Code(keycode) = event.physical_key {
                match keycode {
                    KeyCode::KeyW => {
                        *position_y += *positon_delta_y;
                        *position_x -= *positon_delta_x
                    }
                    KeyCode::KeyS => {
                        *position_y -= *positon_delta_y;
                        *position_x += *positon_delta_x
                    }
                    KeyCode::KeyA => {
                        *angle += 0.1;
                        if *angle > 2.0 * PI {
                            *angle -= 2.0 * PI;
                        }
                        *positon_delta_x = angle.cos() * 5.0;
                        *positon_delta_y = angle.sin() * 5.0;
                    }
                    KeyCode::KeyD => {
                        *angle -= 0.1;
                        if *angle < 0.0 {
                            *angle += 2.0 * PI;
                        }
                        *positon_delta_x = angle.cos() * 5.0;
                        *positon_delta_y = angle.sin() * 5.0;
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }
}

fn redraw(
    display: &Display<WindowSurface>,
    indices: NoIndices,
    //TODO: Os indices podem ficar estaticos
    line_indices: NoIndices,
    program: &Program,
    pos_x: &f32,
    pos_y: &f32,
    ortho: Mat4,
    position_delta_x: &mut f32,
    position_delta_y: &mut f32,
) {
    let mut player = draw_player(*pos_x, *pos_y);
    let mut map = draw_map();
    map.append(&mut player);

    let camera_draw = vec![
        Vertex {
            position: [*pos_x, *pos_y],
            color: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [
                *pos_x + *position_delta_x * 5.0,
                *pos_y + *position_delta_y * 5.0,
            ],
            color: [0.0, 0.0, 1.0],
        },
    ];
    let vertex_buffer =
        VertexBuffer::new(display, &map).unwrap_or_else(|err| panic!("{BUFFER_ERROR} {err}"));
    let camera_line_buffer = VertexBuffer::new(display, &camera_draw)
        .unwrap_or_else(|err| panic!("{BUFFER_ERROR} {err}"));

    let mut target = display.draw();
    target.clear_color(0.3, 0.3, 0.3, 1.0);

    let uniforms = uniform! {
        matrix: ortho.to_cols_array_2d()
    };

    target
        .draw(
            &vertex_buffer,
            indices,
            &program,
            &uniforms,
            &Default::default(),
        )
        .unwrap_or_else(|err| panic!("{DRAWING_ERROR} {err}"));

    target
        .draw(
            &camera_line_buffer,
            &line_indices,
            &program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();

    target.finish().unwrap();
}

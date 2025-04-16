use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

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
use movement::{Direction, PlayerPosition, Point};
use vertexes::{Vertex, draw_map, draw_player};

mod consts;
mod movement;
mod vertexes;

static TRIANGLE_INDICES: NoIndices = NoIndices(PrimitiveType::TrianglesList);
static LINE_INDICES: NoIndices = NoIndices(PrimitiveType::LinesList);

fn main() {
    let start_position = Point::from(500.0, 500.0);
    let mut player_position = PlayerPosition::new(start_position, 0.0);

    let event_loop = EventLoop::builder()
        .build()
        .unwrap_or_else(|err| panic!("{WINDOW_ERROR} {err}"));

    let (window, display) = SimpleWindowBuilder::new()
        .with_inner_size(WIDTH, HEIGHT)
        .build(&event_loop);

    let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER_SRC, None)
        .unwrap_or_else(|err| panic!("{PROGRAM_ERROR} {err}"));

    let ortho = Mat4::orthographic_rh_gl(0.0, WIDTH as f32, HEIGHT as f32, 0.0, -1.0, 1.0);

    let mut pressed_keys: HashSet<KeyCode> = HashSet::new();

    let mut last_frame = Instant::now();
    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => handle_window_event(
                event,
                window_target,
                &display,
                &program,
                ortho,
                &mut player_position,
                &mut pressed_keys,
                &mut last_frame,
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
    program: &Program,
    ortho: Mat4,
    player_position: &mut PlayerPosition,
    pressed_keys: &mut HashSet<KeyCode>,
    last_frame: &mut Instant,
) {
    match event {
        WindowEvent::CloseRequested => window_target.exit(),
        WindowEvent::RedrawRequested => redraw(
            display,
            program,
            ortho,
            player_position,
            pressed_keys,
            last_frame,
        ),
        WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
            PhysicalKey::Code(keycode) => match event.state {
                ElementState::Pressed => {
                    pressed_keys.insert(keycode);
                }
                ElementState::Released => {
                    pressed_keys.remove(&keycode);
                }
            },
            _ => (),
        },
        _ => (),
    }
}

fn redraw(
    display: &Display<WindowSurface>,
    program: &Program,
    ortho: Mat4,
    player_position: &mut PlayerPosition,
    pressed_keys: &mut HashSet<KeyCode>,
    last_frame: &mut Instant,
) {
    let now = Instant::now();

    if now.duration_since(*last_frame) >= Duration::from_millis(16) {
        *last_frame = now;

        for key in &*pressed_keys {
            match key {
                KeyCode::KeyW => player_position.move_up(),
                KeyCode::KeyS => player_position.move_down(),
                KeyCode::KeyA => player_position.rotate(Direction::Left),
                KeyCode::KeyD => player_position.rotate(Direction::Right),
                _ => (),
            }
        }
    }

    let player_coordinates = &player_position.coordinates;

    let mut player = draw_player(player_coordinates.x, player_coordinates.y);
    let mut map = draw_map();
    map.append(&mut player);

    let camera_line_position = player_position.get_camera_line_position();
    let camera_draw = vec![
        Vertex {
            position: [player_position.coordinates.x, player_position.coordinates.y],
            color: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [camera_line_position.x, camera_line_position.y],
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
            TRIANGLE_INDICES,
            program,
            &uniforms,
            &Default::default(),
        )
        .unwrap_or_else(|err| panic!("{DRAWING_ERROR} {err}"));

    target
        .draw(
            &camera_line_buffer,
            LINE_INDICES,
            program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();

    target.finish().unwrap();
}

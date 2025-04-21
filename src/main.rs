use std::time::{Duration, Instant};

use consts::{
    FRAGMENT_SHADER_SRC, START_CIRCUNFERENCE, VERTEX_SHADER,
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
use vertexes::{draw_map, draw_player, draw_rays};

mod consts;
mod movement;
mod vertexes;

static TRIANGLE_INDICES: NoIndices = NoIndices(PrimitiveType::TrianglesList);
static LINE_INDICES: NoIndices = NoIndices(PrimitiveType::LinesList);

fn main() {
    let start_position = Point::from(500.0, 500.0);
    let mut player_position = PlayerPosition::new(start_position, START_CIRCUNFERENCE);

    let event_loop = EventLoop::builder()
        .build()
        .unwrap_or_else(|err| panic!("{WINDOW_ERROR} {err}"));

    let (window, display) = SimpleWindowBuilder::new()
        .with_inner_size(WIDTH, HEIGHT)
        .build(&event_loop);

    let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER_SRC, None)
        .unwrap_or_else(|err| panic!("{PROGRAM_ERROR} {err}"));

    let ortho = Mat4::orthographic_rh_gl(0.0, WIDTH as f32, HEIGHT as f32, 0.0, -1.0, 1.0);

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
    last_frame: &mut Instant,
) {
    match event {
        WindowEvent::CloseRequested => window_target.exit(),
        WindowEvent::RedrawRequested => {
            redraw(display, program, ortho, player_position, last_frame)
        }
        WindowEvent::KeyboardInput { event, .. } => {
            if let PhysicalKey::Code(keycode) = event.physical_key {
                if let ElementState::Pressed = event.state {
                    player_position.pressed_keys.insert(keycode);
                } else {
                    player_position.pressed_keys.remove(&keycode);
                }
            }
        }
        _ => (),
    }
}

fn redraw(
    display: &Display<WindowSurface>,
    program: &Program,
    ortho: Mat4,
    player_position: &mut PlayerPosition,
    last_frame: &mut Instant,
) {
    let now = Instant::now();

    if now.duration_since(*last_frame) >= Duration::from_millis(16) {
        *last_frame = now;

        for key in &player_position.pressed_keys.to_owned() {
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

    let rays = draw_rays(player_position);

    let vertex_buffer =
        VertexBuffer::new(display, &map).unwrap_or_else(|err| panic!("{BUFFER_ERROR} {err}"));
    let ray_buffer =
        VertexBuffer::new(display, &rays).unwrap_or_else(|err| panic!("{BUFFER_ERROR} {err}"));

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
            &ray_buffer,
            LINE_INDICES,
            program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();

    target.finish().unwrap();
}

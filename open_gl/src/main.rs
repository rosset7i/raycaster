use std::{
    sync::OnceLock,
    thread::sleep,
    time::{Duration, Instant},
};

use canva::{VertexBase, get_map_buffer, get_player_buffer, get_rays_buffer};
use casting::{
    consts::{WINDOW_HEIGHT, WINDOW_WIDTH},
    map::Map,
    math::START_CIRCUNFERENCE,
    player::{Direction, PlayerPosition, Point},
    raycasting::{get_map_vertices, get_player_vertices, get_ray_vertices},
};
use consts::{DRAWING_ERROR, FRAGMENT_SHADER_SRC, PROGRAM_ERROR, VERTEX_SHADER, WINDOW_ERROR};
use glam::Mat4;
use glium::{
    Display, Program, Surface,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    index::{NoIndices, PrimitiveType},
    uniform,
    uniforms::{EmptyUniforms, UniformsStorage},
    winit::{
        event::{ElementState, Event, WindowEvent},
        event_loop::{ActiveEventLoop, EventLoop},
        keyboard::{KeyCode, PhysicalKey},
    },
};
use state::{AppState, AppStateBuilder};

mod canva;
mod consts;
mod state;

static TRIANGLE_INDICES: NoIndices = NoIndices(PrimitiveType::TrianglesList);
static LINE_INDICES: NoIndices = NoIndices(PrimitiveType::LinesList);
static UNIFORM: OnceLock<UniformsStorage<'_, [[f32; 4]; 4], EmptyUniforms>> = OnceLock::new();
const FRAME_RATE: Duration = Duration::from_millis(16);

fn main() {
    let event_loop = EventLoop::builder()
        .build()
        .unwrap_or_else(|err| panic!("{WINDOW_ERROR} {err}"));

    let (window, display) = SimpleWindowBuilder::new()
        .with_inner_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_title("Raycaster")
        .build(&event_loop);

    let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER_SRC, None)
        .unwrap_or_else(|err| panic!("{PROGRAM_ERROR} {err}"));

    let mut app_state = initialize_state(display, program);

    let _ = UNIFORM.set(uniform! {
        matrix: Mat4::orthographic_rh_gl(
            0.0,
            WINDOW_WIDTH as f32,
            WINDOW_HEIGHT as f32,
            0.0,
            -1.0,
            1.0,
        ).to_cols_array_2d()
    });

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => {
                handle_window_event(event, window_target, &mut app_state)
            }
            Event::AboutToWait => window.request_redraw(),
            _ => (),
        };
    });
}

fn initialize_state(display: Display<WindowSurface>, program: Program) -> AppState {
    let player_position = PlayerPosition::new(Point::from(500.0, 500.0), 30.0f32.to_radians());

    let map = Map::new();

    let rays_buffer = get_rays_buffer(&display, get_ray_vertices(&player_position, &map));

    let map_buffer = get_map_buffer(&display, get_map_vertices(&map));

    let player_buffer = get_player_buffer(&display, get_player_vertices(&player_position));

    AppStateBuilder::new(display, program)
        .with_position(player_position)
        .with_map(map)
        .with_map_buffer(map_buffer)
        .with_player_buffer(player_buffer)
        .with_rays_buffer(rays_buffer)
        .build()
        .unwrap()
}

fn handle_window_event(
    event: WindowEvent,
    window_target: &ActiveEventLoop,
    app_state: &mut AppState,
) {
    match event {
        WindowEvent::CloseRequested => window_target.exit(),
        WindowEvent::RedrawRequested => redraw(app_state),
        WindowEvent::KeyboardInput { event, .. } => {
            if let PhysicalKey::Code(keycode) = event.physical_key {
                if let ElementState::Pressed = event.state {
                    app_state.pressed_keys.insert(keycode);
                } else {
                    app_state.pressed_keys.remove(&keycode);
                }
            }
        }
        _ => (),
    }
}

fn redraw(app_state: &mut AppState) {
    let now = Instant::now();
    let elapsed = now.duration_since(app_state.last_frame);

    if elapsed < FRAME_RATE {
        sleep(FRAME_RATE - elapsed);
    }

    app_state.last_frame = now;

    for key in app_state.pressed_keys.iter() {
        match key {
            KeyCode::KeyW => app_state.player_position.move_up(),
            KeyCode::KeyS => app_state.player_position.move_down(),
            KeyCode::KeyA => app_state.player_position.rotate(Direction::Left),
            KeyCode::KeyD => app_state.player_position.rotate(Direction::Right),
            _ => (),
        }
    }

    let player_vertices = get_player_vertices(&app_state.player_position);
    app_state
        .player_buffer
        .write(player_vertices.as_gpu_vertices().as_slice());

    let rays_vertices = get_ray_vertices(&app_state.player_position, &app_state.map);
    app_state
        .rays_buffer
        .write(rays_vertices.as_gpu_vertices().as_slice());

    let mut target = app_state.display.draw();
    target.clear_color(0.3, 0.3, 0.3, 1.0);
    target
        .draw(
            &app_state.map_buffer,
            TRIANGLE_INDICES,
            &app_state.program,
            UNIFORM.get().unwrap(),
            &Default::default(),
        )
        .unwrap_or_else(|err| panic!("{DRAWING_ERROR} {err}"));

    target
        .draw(
            &app_state.player_buffer,
            TRIANGLE_INDICES,
            &app_state.program,
            UNIFORM.get().unwrap(),
            &Default::default(),
        )
        .unwrap_or_else(|err| panic!("{DRAWING_ERROR} {err}"));

    target
        .draw(
            &app_state.rays_buffer,
            LINE_INDICES,
            &app_state.program,
            UNIFORM.get().unwrap(),
            &Default::default(),
        )
        .unwrap_or_else(|err| panic!("{DRAWING_ERROR} {err}"));

    target.finish().unwrap();
}

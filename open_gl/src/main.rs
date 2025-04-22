use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use casting::{
    consts::{WINDOW_HEIGHT, WINDOW_WIDTH},
    map::Map,
    math::START_CIRCUNFERENCE,
    player::{Direction, PlayerPosition, Point},
    raycasting::{Vertex, draw_map, draw_player, draw_rays},
};
use consts::{
    BUFFER_ERROR, DRAWING_ERROR, FRAGMENT_SHADER_SRC, PROGRAM_ERROR, VERTEX_SHADER, WINDOW_ERROR,
};
use glam::Mat4;
use glium::{
    Display, Program, Surface, VertexBuffer,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    uniform,
    winit::{
        event::{ElementState, Event, WindowEvent},
        event_loop::{ActiveEventLoop, EventLoop},
        keyboard::{KeyCode, PhysicalKey},
    },
};

mod consts;

static TRIANGLE_INDICES: NoIndices = NoIndices(PrimitiveType::TrianglesList);
static LINE_INDICES: NoIndices = NoIndices(PrimitiveType::LinesList);

#[derive(Debug, Copy, Clone)]
pub struct GpuVertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

pub trait VertexBase {
    fn as_gpu_vertex(&self) -> Vec<GpuVertex>;
}

impl VertexBase for Vec<Vertex> {
    fn as_gpu_vertex(&self) -> Vec<GpuVertex> {
        self.iter()
            .map(|x| GpuVertex {
                position: x.position,
                color: x.color,
            })
            .collect()
    }
}

implement_vertex!(GpuVertex, position, color);

fn main() {
    let start_position = Point::from(500.0, 500.0);
    let mut player_position = PlayerPosition::new(start_position, START_CIRCUNFERENCE);

    let event_loop = EventLoop::builder()
        .build()
        .unwrap_or_else(|err| panic!("{WINDOW_ERROR} {err}"));

    let (window, display) = SimpleWindowBuilder::new()
        .with_inner_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .build(&event_loop);

    let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER_SRC, None)
        .unwrap_or_else(|err| panic!("{PROGRAM_ERROR} {err}"));

    let ortho = Mat4::orthographic_rh_gl(
        0.0,
        WINDOW_WIDTH as f32,
        WINDOW_HEIGHT as f32,
        0.0,
        -1.0,
        1.0,
    );

    let mut last_frame = Instant::now();
    let mut pressed_keys: HashSet<KeyCode> = HashSet::new();

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
                &mut pressed_keys,
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
    pressed_keys: &mut HashSet<KeyCode>,
) {
    match event {
        WindowEvent::CloseRequested => window_target.exit(),
        WindowEvent::RedrawRequested => redraw(
            display,
            program,
            ortho,
            player_position,
            last_frame,
            pressed_keys,
        ),
        WindowEvent::KeyboardInput { event, .. } => {
            if let PhysicalKey::Code(keycode) = event.physical_key {
                if let ElementState::Pressed = event.state {
                    pressed_keys.insert(keycode);
                } else {
                    pressed_keys.remove(&keycode);
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
    pressed_keys: &mut HashSet<KeyCode>,
) {
    let now = Instant::now();

    if now.duration_since(*last_frame) >= Duration::from_millis(16) {
        *last_frame = now;

        for key in pressed_keys.iter() {
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

    let map = Map::new();

    let mut player = draw_player(player_coordinates.x, player_coordinates.y);
    let mut map_vertex = draw_map(&map);
    map_vertex.append(&mut player);

    let rays = draw_rays(player_position, &map);

    let vertex_buffer = VertexBuffer::new(display, &map_vertex.as_gpu_vertex())
        .unwrap_or_else(|err| panic!("{BUFFER_ERROR} {err}"));
    let ray_buffer = VertexBuffer::new(display, &rays.as_gpu_vertex())
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
            &ray_buffer,
            LINE_INDICES,
            program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();

    target.finish().unwrap();
}

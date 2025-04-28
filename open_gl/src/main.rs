use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use canva::{get_map_buffer, get_player_buffer, get_rays_buffer};
use casting::{
    consts::{WINDOW_HEIGHT, WINDOW_WIDTH},
    map::Map,
    math::START_CIRCUNFERENCE,
    player::{Direction, PlayerPosition, Point},
    raycasting::{
        Vertex, get_map_vertex_coordinates, get_player_vertex_coordinates,
        get_ray_vertex_coordinates,
    },
};
use consts::{DRAWING_ERROR, FRAGMENT_SHADER_SRC, PROGRAM_ERROR, VERTEX_SHADER, WINDOW_ERROR};
use glam::Mat4;
use glium::{
    Display, Program, Surface, VertexBuffer,
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    uniform,
    uniforms::{EmptyUniforms, UniformsStorage},
    winit::{
        event::{ElementState, Event, WindowEvent},
        event_loop::{ActiveEventLoop, EventLoop},
        keyboard::{KeyCode, PhysicalKey},
    },
};

mod canva;
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

    let map = Map::new();

    let uniforms = uniform! {
        matrix: ortho.to_cols_array_2d()
    };

    let map_buffer = get_map_buffer(&display, get_map_vertex_coordinates(&map));
    let player_buffer = get_player_buffer(
        &display,
        get_player_vertex_coordinates(player_position.coordinates.x, player_position.coordinates.y),
    );
    let rays_buffer = get_rays_buffer(&display, get_ray_vertex_coordinates(&player_position, &map));

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
                &uniforms,
                &mut player_position,
                &mut last_frame,
                &mut pressed_keys,
                &map_buffer,
                &rays_buffer,
                &player_buffer,
                &map,
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
    uniforms: &UniformsStorage<'_, [[f32; 4]; 4], EmptyUniforms>,
    player_position: &mut PlayerPosition,
    last_frame: &mut Instant,
    pressed_keys: &mut HashSet<KeyCode>,
    map_buffer: &VertexBuffer<GpuVertex>,
    rays_buffer: &VertexBuffer<GpuVertex>,
    player_buffer: &VertexBuffer<GpuVertex>,
    map: &Map,
) {
    match event {
        WindowEvent::CloseRequested => window_target.exit(),
        WindowEvent::RedrawRequested => redraw(
            display,
            program,
            uniforms,
            player_position,
            last_frame,
            pressed_keys,
            map_buffer,
            rays_buffer,
            player_buffer,
            map,
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
    uniforms: &UniformsStorage<'_, [[f32; 4]; 4], EmptyUniforms>,
    player_position: &mut PlayerPosition,
    last_frame: &mut Instant,
    pressed_keys: &mut HashSet<KeyCode>,
    map_buffer: &VertexBuffer<GpuVertex>,
    rays_buffer: &VertexBuffer<GpuVertex>,
    player_buffer: &VertexBuffer<GpuVertex>,
    map: &Map,
) {
    let now = Instant::now();

    if now.duration_since(*last_frame) < Duration::from_millis(16) {
        return;
    }

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

    let player_coordinates = &player_position.coordinates;

    let player_vertex_coordinates =
        get_player_vertex_coordinates(player_coordinates.x, player_coordinates.y);
    player_buffer.write(player_vertex_coordinates.as_gpu_vertex().as_slice());

    let rays_vertex_coordinates = get_ray_vertex_coordinates(&player_position, map);
    rays_buffer.write(rays_vertex_coordinates.as_gpu_vertex().as_slice());

    let mut target = display.draw();
    target.clear_color(0.3, 0.3, 0.3, 1.0);
    target
        .draw(
            map_buffer,
            TRIANGLE_INDICES,
            program,
            uniforms,
            &Default::default(),
        )
        .unwrap_or_else(|err| panic!("{DRAWING_ERROR} {err}"));

    target
        .draw(
            player_buffer,
            TRIANGLE_INDICES,
            program,
            uniforms,
            &Default::default(),
        )
        .unwrap();

    target
        .draw(
            rays_buffer,
            LINE_INDICES,
            program,
            uniforms,
            &Default::default(),
        )
        .unwrap();

    target.finish().unwrap();
}

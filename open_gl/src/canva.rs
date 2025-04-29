use crate::consts;
use casting::raycasting::Vertex;
use consts::BUFFER_ERROR;
use glium::{Display, VertexBuffer, glutin::surface::WindowSurface, implement_vertex};

#[derive(Debug, Copy, Clone)]
pub struct GpuVertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

pub trait VertexBase {
    fn as_gpu_vertices(&self) -> Vec<GpuVertex>;
}

impl VertexBase for Vec<Vertex> {
    fn as_gpu_vertices(&self) -> Vec<GpuVertex> {
        self.iter()
            .map(|x| GpuVertex {
                position: x.position,
                color: x.color,
            })
            .collect()
    }
}

implement_vertex!(GpuVertex, position, color);

pub fn get_map_buffer(
    display: &Display<WindowSurface>,
    map_coordinates: Vec<Vertex>,
) -> VertexBuffer<GpuVertex> {
    VertexBuffer::new(display, &map_coordinates.as_gpu_vertices())
        .unwrap_or_else(|err| panic!("{BUFFER_ERROR} {err}"))
}

pub fn get_rays_buffer(
    display: &Display<WindowSurface>,
    ray_coordinates: Vec<Vertex>,
) -> VertexBuffer<GpuVertex> {
    VertexBuffer::dynamic(display, &ray_coordinates.as_gpu_vertices())
        .unwrap_or_else(|err| panic!("{BUFFER_ERROR} {err}"))
}

pub fn get_player_buffer(
    display: &Display<WindowSurface>,
    player_coordinates: Vec<Vertex>,
) -> VertexBuffer<GpuVertex> {
    VertexBuffer::dynamic(display, &player_coordinates.as_gpu_vertices())
        .unwrap_or_else(|err| panic!("{BUFFER_ERROR} {err}"))
}

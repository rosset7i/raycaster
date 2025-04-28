use crate::{GpuVertex, VertexBase, consts};
use casting::raycasting::Vertex;
use consts::BUFFER_ERROR;
use glium::{Display, VertexBuffer, glutin::surface::WindowSurface};

pub fn get_map_buffer(
    display: &Display<WindowSurface>,
    map_coordinates: Vec<Vertex>,
) -> VertexBuffer<GpuVertex> {
    VertexBuffer::new(display, &map_coordinates.as_gpu_vertex())
        .unwrap_or_else(|err| panic!("{BUFFER_ERROR} {err}"))
}

pub fn get_rays_buffer(
    display: &Display<WindowSurface>,
    ray_coordinates: Vec<Vertex>,
) -> VertexBuffer<GpuVertex> {
    VertexBuffer::dynamic(display, &ray_coordinates.as_gpu_vertex())
        .unwrap_or_else(|err| panic!("{BUFFER_ERROR} {err}"))
}

pub fn get_player_buffer(
    display: &Display<WindowSurface>,
    player_coordinates: Vec<Vertex>,
) -> VertexBuffer<GpuVertex> {
    VertexBuffer::dynamic(display, &player_coordinates.as_gpu_vertex())
        .unwrap_or_else(|err| panic!("{BUFFER_ERROR} {err}"))
}

use std::{collections::HashSet, time::Instant};

use canva::GpuVertex;
use casting::{map::Map, player::PlayerPosition};
use glium::{
    Display, Program, VertexBuffer, glutin::surface::WindowSurface, winit::keyboard::KeyCode,
};

use crate::canva;

pub struct AppState {
    pub player_position: PlayerPosition,
    pub display: Display<WindowSurface>,
    pub program: Program,
    pub map_buffer: VertexBuffer<GpuVertex>,
    pub player_buffer: VertexBuffer<GpuVertex>,
    pub rays_buffer: VertexBuffer<GpuVertex>,
    pub pressed_keys: HashSet<KeyCode>,
    pub last_frame: Instant,
    pub map: Map,
}

pub struct AppStateBuilder {
    player_position: PlayerPosition,
    display: Display<WindowSurface>,
    program: Program,
    map_buffer: Option<VertexBuffer<GpuVertex>>,
    player_buffer: Option<VertexBuffer<GpuVertex>>,
    rays_buffer: Option<VertexBuffer<GpuVertex>>,
    map: Option<Map>,
}

impl AppStateBuilder {
    pub fn new(display: Display<WindowSurface>, program: Program) -> Self {
        Self {
            player_position: PlayerPosition::default(),
            display,
            program,
            map_buffer: None,
            player_buffer: None,
            rays_buffer: None,
            map: None,
        }
    }

    pub fn with_position(mut self, player_position: PlayerPosition) -> Self {
        self.player_position = player_position;
        self
    }

    pub fn with_map_buffer(mut self, map_buffer: VertexBuffer<GpuVertex>) -> Self {
        self.map_buffer = Some(map_buffer);
        self
    }

    pub fn with_player_buffer(mut self, player_buffer: VertexBuffer<GpuVertex>) -> Self {
        self.player_buffer = Some(player_buffer);
        self
    }

    pub fn with_rays_buffer(mut self, rays_buffer: VertexBuffer<GpuVertex>) -> Self {
        self.rays_buffer = Some(rays_buffer);
        self
    }

    pub fn with_map(mut self, map: Map) -> Self {
        self.map = Some(map);
        self
    }

    pub fn build(self) -> Result<AppState, String> {
        Ok(AppState {
            player_position: self.player_position,
            display: self.display,
            program: self.program,
            map_buffer: self
                .map_buffer
                .ok_or_else(|| "Missing map buffer".to_string())?,
            player_buffer: self
                .player_buffer
                .ok_or_else(|| "Missing player buffer".to_string())?,
            rays_buffer: self
                .rays_buffer
                .ok_or_else(|| "Missing rays buffer".to_string())?,
            pressed_keys: HashSet::new(),
            last_frame: Instant::now(),
            map: self.map.ok_or_else(|| "Missing map".to_string())?,
        })
    }
}

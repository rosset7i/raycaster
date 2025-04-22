use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

const MAP: [[u32; 10 as usize]; 11 as usize] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

pub struct Map {
    pub tiles: Vec<Vec<u32>>,
    pub length_x: u32,
    pub length_y: u32,
}

//TODO: Read from file, Calculate DEPTH_OF_FIELD when loading map
impl Map {
    pub fn new() -> Map {
        Map {
            tiles: MAP.into_iter().map(|x| x.to_vec()).collect(),
            length_x: MAP[0].len() as u32,
            length_y: MAP.len() as u32,
        }
    }

    pub fn get_tile_size(&self) -> (f32, f32) {
        (
            (WINDOW_WIDTH / self.length_x) as f32,
            (WINDOW_HEIGHT / self.length_y) as f32,
        )
    }
}

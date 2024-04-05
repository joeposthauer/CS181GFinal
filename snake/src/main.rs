use frenderer::{
    input::{Input, Key},
    sprites::{Camera2D, SheetRegion, Transform},
    wgpu, Renderer,
};

#[derive(Clone, Copy, Debug)]
struct TileData {
    solid: bool,
    sheet_region: SheetRegion,
}

enum EntityType {
    Snake,
    Food,
}

struct Game {
    started: bool,
}

fn main() {
    println!("Ayelet - Change test");
}

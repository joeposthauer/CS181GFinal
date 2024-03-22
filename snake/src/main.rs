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

fn main() {
    println!("Hello, world!");
}

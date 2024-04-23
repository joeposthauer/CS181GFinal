// use assets_manager::{asset::Png, AssetCache};
use frenderer::{
    input::{Input, Key},
    sprites::{Camera2D, SheetRegion, Transform},
    wgpu, Immediate, Renderer,
};

use engine::{grid::Grid, *};
use engine::{level::Level, *};

const TILE_SZ: usize = 4;
const W: usize = 120;
const H: usize = 120;
const DT: f32 = 1.0 / 60.0;

fn main() {
    println!("Hello, world!");
}

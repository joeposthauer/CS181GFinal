// use assets_manager::{asset::Png, AssetCache};
use frenderer::{
    input::{Input, Key},
    sprites::{Camera2D, SheetRegion, Transform},
    wgpu, Immediate, Renderer,
};

use std::collections::VecDeque;

use engine::{grid::Grid, *};
use engine::{level::Level, *};

const TILE_SZ: usize = 4;
const W: usize = 120;
const H: usize = 120;
const DT: f32 = 1.0 / 60.0;


pub enum EntityType {
    Gold,
    Silver,
    Rock,
}
struct Claw {
    dir: Dir,
    body: VecDeque<Vec2>,
}

impl Claw {
    pub fn transform(&self, index: usize) -> Transform {
        Transform {
            x: self.body.get(index).unwrap().x,
            y: self.body.get(index).unwrap().y,
            w: 4,
            h: 4,
            rot: 0.0,
        }
    }
}

struct Object {
    pos: Vec2,
    e_type: EntityType,
}

impl Object {
    pub fn transform(&self) -> Transform {
        Transform {
            x: self.pos.x,
            y: self.pos.y,
            w: 4,
            h: 4,
            rot: 0.0,
        }
    }
}





// we need to define where in tilesheet we are representing each of these
const CLAW: [SheetRegion; 1] = [SheetRegion::rect(533, 39, 4, 4)];
const GOLD: [SheetRegion; 1] = [SheetRegion::rect(190, 345, 4, 4)];
const SILVER: [SheetRegion; 1] = [SheetRegion::rect(190, 345, 4, 4)];
const ROCK: [SheetRegion; 1] = [SheetRegion::rect(190, 345, 4, 4)];

fn main() {
    println!("Hello, world!");
}

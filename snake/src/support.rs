use frenderer::{
    input::{Input, Key},
    sprites::{Camera2D, SheetRegion, Transform},
    wgpu, Renderer,
};
use std::collections::HashMap;
use std::str::FromStr;


const FOOD: [SheetRegion; 4] = [
    SheetRegion::rect(533 + 16 * 2, 39, 16, 16),
    SheetRegion::rect(533 + 16, 39, 16, 16),
    SheetRegion::rect(533, 39, 16, 16),
    SheetRegion::rect(533 + 16 * 3, 39, 16, 16),
];

const SNAKE: [SheetRegion; 4] = [
    SheetRegion::rect(533 + 16 * 2, 39, 16, 16),
    SheetRegion::rect(533 + 16, 39, 16, 16),
    SheetRegion::rect(533, 39, 16, 16),
    SheetRegion::rect(533 + 16 * 3, 39, 16, 16),
];

#[derive(Clone, Copy, Debug)]
pub struct TileData {
    solid: bool,
    sheet_region: SheetRegion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn to_vec2(self) -> Vec2 {
        match self {
            Dir::Up => Vec2 { x: 0.0, y: 1.0 },
            Dir::Right => Vec2 { x: 1.0, y: 0.0 },
            Dir::Down => Vec2 { x: 0.0, y: -1.0 },
            Dir::Left => Vec2 { x: -1.0, y: 0.0 },
        }
    }
}

const TILE_SZ: usize = 4;
const W: usize = 320;
const H: usize = 240;

pub mod level;
pub mod grid;

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub struct Tileset {
    tiles: Vec<TileData>,
}

pub enum EntityType {
    Snake,
    Food,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub fn overlap(&self, other: Rect) -> Option<Vec2> {
        let x_overlap =
            (self.x + self.w as f32).min(other.x + other.w as f32) - self.x.max(other.x);
        let y_overlap =
            (self.y + self.h as f32).min(other.y + other.h as f32) - self.y.max(other.y);
        if x_overlap >= 0.0 && y_overlap >= 0.0 {
            // This will return the magnitude of overlap in each axis.
            Some(Vec2 {
                x: x_overlap,
                y: y_overlap,
            })
        } else {
            None
        }
    }
    pub fn origin(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.w == 0 || self.h == 0
    }
}

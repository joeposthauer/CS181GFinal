use frenderer::{
    input::{Input, Key},
    sprites::{Camera2D, SheetRegion, Transform},
    wgpu, Renderer,
};

#[derive(Clone, Copy, Debug)]
pub struct TileData {
    solid: bool,
    sheet_region: SheetRegion,
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub struct Grid<T> {
    width: usize,
    height: usize,
    storage: Box<[T]>,
}

pub struct Tileset {
    tiles: Vec<TileData>,
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

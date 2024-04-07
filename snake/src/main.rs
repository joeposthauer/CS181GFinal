use assets_manager::{asset::Png, AssetCache};
use frenderer::{
    input::{Input, Key},
    sprites::{Camera2D, SheetRegion, Transform},
    wgpu, Renderer,
};
use std::str::FromStr;

mod support;
use support::*;

struct Game {
    started: bool,
    snake: Vec<Vec2>,
}

const TILE_SZ: usize = 4;
//change as needed
const W: usize = 320;
const H: usize = 240;

fn main() {
    println!("Ayelet - Change test");
}

impl Game {
    fn new(renderer: &mut Renderer, cache: &AssetCache) -> Self {
        let tile_handle = cache
            .load::<Png>("texture")
            .expect("Couldn't load tilesheet img");
        let tile_img = tile_handle.read().0.to_rgba8();
        let tile_tex = renderer.create_array_texture(
            &[&tile_img],
            wgpu::TextureFormat::Rgba8UnormSrgb,
            tile_img.dimensions(),
            Some("tiles-sprites"),
        );
        let levels = vec![Level::from_str(
            &cache
                .load::<String>("level")
                .expect("Couldn't access level.txt")
                .read(),
        )];
        let current_level = 0;
        let camera = Camera2D {
            screen_pos: [0.0, 0.0],
            screen_size: [W as f32, H as f32],
        };
        // let sprite_estimate =
        //     levels[current_level].sprite_count() + levels[current_level].starts().len();
        // renderer.sprite_group_add(
        //     &tile_tex,
        //     vec![Transform::ZERO; sprite_estimate],
        //     vec![SheetRegion::ZERO; sprite_estimate],
        //     camera,
        // );
        // let player_start = *levels[current_level]
        //     .starts()
        //     .iter()
        //     .find(|(t, _)| *t == EntityType::Player)
        //     .map(|(_, ploc)| ploc)
        //     .expect("Start level doesn't put the player anywhere");
        let mut game = Game {
            started: true,
            snake: vec![Vec2{x:0.0,y:0.0,}],
        };
        game
    }
}

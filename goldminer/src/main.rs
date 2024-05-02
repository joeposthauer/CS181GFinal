use assets_manager::{asset::Png, AssetCache};
use frenderer::{
    input::{Input, Key},
    sprites::{Camera2D, SheetRegion, Transform},
    wgpu, Immediate, Renderer,
};

// Ayelet - I cannot run this line. It seems to step from differences in hardware that I guess my machine
// doesnt' support. I changed it to the following two lines which seems to solve the issue for me.
// use std::{arch::aarch64::float32x2_t, collections::VecDeque}; -Joe ok!

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::float32x2_t;
use std::{collections::VecDeque, intrinsics::cosf64};

// use std::collections::VecDeque;

use engine::{grid::Grid, *};
use engine::{level::Level, *};

const TILE_SZ: usize = 8;
const W: usize = 240; //was 120 - should be 240, 8 * 30, no?
const H: usize = 240; //was 120 - should be 240, 8 * 30, no?
const DT: f32 = 1.0 / 60.0;
const CLAW_ROT_VEL: f32 = 0.1;
const CHAIN_SIZE: f32 = 8.0;

struct Game {
    claw: Claw,
    score: usize,
    current_level: Level, // Ayelet - was usize, changed to level
    levels: Vec<Level>,
    entities: Vec<Object>,
    timer: usize,
    frame_counter: usize,
    move_interval: usize,
}
struct Claw {
    dir: f32,
    body: VecDeque<Vec2>,
    is_deployed: bool,
    velo_dir: bool,
    claw_dir: bool,
}

impl Claw {
    pub fn transform(&self) -> Transform {
        Transform {
            x: self.body.get(0).unwrap().x,
            y: self.body.get(0).unwrap().y,
            w: 8,
            h: 8,
            rot: self.dir,
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
            w: 8,
            h: 8,
            rot: 0.0,
        }
    }

    pub fn uv(&self) -> SheetRegion {
        match self.e_type {
            EntityType::Gold => GOLD[0],
            EntityType::Silver => SILVER[0],
            EntityType::Rock => ROCK[0],
            EntityType::Gem => GEM[0],
            _ => panic!("can't draw this type"),
        }
        .with_depth(1)
    }
}

struct Contact {
    rect_a: Rect,
    index_a: usize,
    rect_b: Rect,
    index_b: usize,
    overlap: Vec2,
}

// Claw 68 260 128 64
// Chain 1 262 64 64
// Rock 134 65 64 64
// Gold 1 131 64 64
// Silver 1 197 64 64
// Gem 68 131 64 64
// Bottom background  1 65 64 64
// Top Background 68 65 64 64

// 64 by 64 coordinated, related to Goldminer_tilesheet_trasparent
// const CLAW: [SheetRegion; 1] = [SheetRegion::rect(68, 260, 32, 64)];
// const GOLD: [SheetRegion; 1] = [SheetRegion::rect(1, 131, 32, 32)];
// const SILVER: [SheetRegion; 1] = [SheetRegion::rect(1, 197, 32, 32)];
// const ROCK: [SheetRegion; 1] = [SheetRegion::rect(134, 65, 32, 32)];
// const GEM: [SheetRegion; 1] = [SheetRegion::rect(68, 131, 32, 32)];
// const CHAIN: [SheetRegion; 1] = [SheetRegion::rect(1, 262, 32, 32)];

// 8 by 8 coordinates, related to Goldminer_tilesheet1
const CLAW: [SheetRegion; 1] = [SheetRegion::rect(1, 71, 8, 16)];
const GOLD: [SheetRegion; 1] = [SheetRegion::rect(1, 17, 8, 8)];
const SILVER: [SheetRegion; 1] = [SheetRegion::rect(1, 26, 8, 8)];
const ROCK: [SheetRegion; 1] = [SheetRegion::rect(1, 35, 8, 8)];
const GEM: [SheetRegion; 1] = [SheetRegion::rect(1, 53, 8, 8)];
const CHAIN: [SheetRegion; 1] = [SheetRegion::rect(1, 80, 8, 8)];

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let source =
        assets_manager::source::FileSystem::new("content").expect("Couldn't load resources");
    #[cfg(target_arch = "wasm32")]
    let source = assets_manager::source::Embedded::from(assets_manager::source::embed!("content"));
    let cache = assets_manager::AssetCache::with_source(source);

    let drv = frenderer::Driver::new(
        winit::window::WindowBuilder::new()
            .with_title("test")
            .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0)),
        Some((W as u32, H as u32)),
    );

    let mut input = Input::default();

    let mut now = frenderer::clock::Instant::now();
    let mut acc = 0.0;
    drv.run_event_loop::<(), _>(
        move |window, frend| {
            let mut frend = Immediate::new(frend);
            let game = Game::new(&mut frend, cache);
            (window, game, frend)
        },
        move |event, target, (window, ref mut game, ref mut frend)| {
            use winit::event::{Event, WindowEvent};
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    target.exit();
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    if !frend.gpu().is_web() {
                        frend.resize_surface(size.width, size.height);
                    }
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    let elapsed = now.elapsed().as_secs_f32();
                    // You can add the time snapping/death spiral prevention stuff here if you want.
                    // I'm not using it here to keep the starter code small.
                    acc += elapsed;
                    now = std::time::Instant::now();
                    // While we have time to spend
                    while acc >= DT {
                        // simulate a frame
                        acc -= DT;
                        game.simulate(&input, DT);
                        input.next_frame();
                    }
                    game.render(frend);
                    frend.render();
                    window.request_redraw();
                }
                event => {
                    input.process_input_event(&event);
                }
            }
        },
    )
    .expect("event loop error");
}

impl Game {
    fn new(renderer: &mut Immediate, cache: AssetCache) -> Self {
        let tile_handle = cache
            .load::<Png>("Goldminer_tilesheet1")
            .expect("Couldn't load tilesheet img");
        let tile_img = tile_handle.read().0.to_rgba8();
        let tile_tex = renderer.create_array_texture(
            &[&tile_img],
            wgpu::TextureFormat::Rgba8UnormSrgb,
            tile_img.dimensions(),
            Some("tiles-sprites"),
        );
        // Ayelet: Changed this to be only one level
        let level = engine::level::Level::from_str(
            &cache
                .load::<String>("level")
                .expect("Couldn't access level.txt")
                .read(),
        );
        // let current_level = 0; // For future if we want to add more levels?
        let camera = Camera2D {
            screen_pos: [0.0, 0.0],
            screen_size: [W as f32, H as f32],
        };
        let sprite_estimate = level.sprite_count() + level.starts().len();
        renderer.sprite_group_add(
            &tile_tex,
            vec![Transform::ZERO; sprite_estimate],
            vec![SheetRegion::ZERO; sprite_estimate],
            camera,
        );
        let mut claw_body: VecDeque<Vec2> = VecDeque::new();
        claw_body.push_back(Vec2 { x: 200.0, y: 250.0 });
        let mut entities: Vec<Object> = vec![];
        entities.push(Object {
            pos: Vec2 { x: 200.0, y: 150.0 },
            e_type: EntityType::Rock,
        });
        entities.push(Object {
            pos: Vec2 { x: 150.0, y: 150.0 },
            e_type: EntityType::Gem,
        });
        entities.push(Object {
            pos: Vec2 { x: 100.0, y: 200.0 },
            e_type: EntityType::Gold,
        });
        let mut game = Game {
            claw: Claw {
                dir: 0.0,
                body: claw_body,
                is_deployed: false,
                velo_dir: false,
                claw_dir: true,
            },
            score: 0,
            current_level: level,
            levels: vec![],
            entities: entities,
            timer: 30,
            frame_counter: 0,
            move_interval: 5,
        };
        game
    }

    fn render(&mut self, frend: &mut Immediate) {
        self.current_level.render_immediate(frend);
        frend.draw_sprite(0, self.claw.transform(), CLAW[0]);
        for obj in self.entities.iter() {
            match obj.e_type {
                EntityType::Gold => frend.draw_sprite(0, obj.transform(), GOLD[0]),
                EntityType::Silver => frend.draw_sprite(0, obj.transform(), SILVER[0]),
                EntityType::Rock => frend.draw_sprite(0, obj.transform(), ROCK[0]),
                EntityType::Gem => frend.draw_sprite(0, obj.transform(), GEM[0]),
                EntityType::Snake => continue,
                EntityType::Food => continue,
                EntityType::Claw => continue,
            }
        }
    }

    fn simulate(&mut self, input: &Input, dt: f32) {
        self.frame_counter += 1;
        if self.frame_counter >= self.move_interval {
            if input.is_key_down(Key::Space) && self.claw.is_deployed != true {
                self.claw.is_deployed = true;
            }
            // rotate claw
            if self.claw.is_deployed == false {
                if self.claw.velo_dir == true {
                    if self.claw.dir > 1.0 {
                        self.claw.velo_dir = !self.claw.velo_dir;
                    } else {
                        self.claw.dir += CLAW_ROT_VEL;
                    }
                } else {
                    if self.claw.dir < 1.0 {
                        self.claw.velo_dir = !self.claw.velo_dir;
                    } else {
                        self.claw.dir -= CLAW_ROT_VEL;
                    }
                }
            }

            // move claw
            if self.claw.is_deployed == true {
                // shoot claw
                if self.claw.claw_dir == true {
                    let curr_x = self.claw.body.front().unwrap().x;
                    let curr_y = self.claw.body.front().unwrap().y;
                    let new_x: f32 = curr_x + CHAIN_SIZE*f32::cos(self.claw.dir);
                    let new_y: f32 = curr_y + CHAIN_SIZE*f32::sin(self.claw.dir);  
                    self.claw.body.push_front(*self.claw.body.back().unwrap());
                } 
                else 
                // retract claw
                {
                    self.claw.body.pop_back();
                }
            }

            // change claw direction when claw gets outside map 
            if self.claw.body.get(0).unwrap().x < 0.0
                || self.claw.body.get(0).unwrap().y < 0.0
                || self.claw.body.get(0).unwrap().x >= W as f32
                || self.claw.body.get(0).unwrap().y >= H as f32
            {
                self.claw.claw_dir = !self.claw.claw_dir;
            } 
            // change claw direction if collision
            for entity in self.entities.iter() {
                if self.claw.body.contains(&entity.pos) {
                    self.claw.claw_dir = !self.claw.claw_dir;
                }
            }
            self.frame_counter = 0;
        }
    }

    fn gather_contact(a_rects: &[Rect], b_rects: &[Rect], contacts_list: &mut Vec<Contact>) {
        for (i, a_rect) in a_rects.iter().enumerate() {
            for (j, b_rect) in b_rects.iter().enumerate() {
                if let Some(overlap) = a_rect.overlap(*b_rect) {
                    contacts_list.push(Contact {
                        index_a: i,
                        rect_a: *a_rect,
                        index_b: j,
                        rect_b: *b_rect,
                        overlap: overlap,
                    })
                }
            }
        }
    }

    fn compute_displacement(a: Rect, b: Rect) -> Vec2 {
        let Some(mut overlap) = a.overlap(b) else {
            return Vec2 { x: 0.0, y: 0.0 };
        };
        if overlap.y < overlap.x {
            overlap.x = 0.0;
        } else {
            overlap.y = 0.0;
        }
        if a.x < b.x {
            overlap.x *= -1.0;
        }
        if a.y < b.y {
            overlap.y *= -1.0;
        }
        return overlap;
    }
}

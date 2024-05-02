use assets_manager::{asset::Png, AssetCache};
use frenderer::{
    input::{Input, Key},
    sprites::{Camera2D, SheetRegion, Transform},
    wgpu, Immediate, Renderer,
};
use wgpu::naga::back::msl::EntryPointError;

// Ayelet - I cannot run this line. It seems to step from differences in hardware that I guess my machine
// doesnt' support. I changed it to the following two lines which seems to solve the issue for me.
// use std::{arch::aarch64::float32x2_t, collections::VecDeque}; -Joe ok!

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::float32x2_t;
use std::collections::VecDeque;

// use std::collections::VecDeque;

use engine::{grid::Grid, *};
use engine::{level::Level, *};

const TILE_SZ: usize = 8;
const W: usize = 240;
const H: usize = 240;
pub const PI: f32 = 3.14159265358979323846264338327950288_f32; // 3.1415926535897931f64
const DT: f32 = 1.0 / 60.0;
const CLAW_ROT_VEL: f32 = 0.1;
const CHAIN_SIZE: f32 = 8.0;

struct Game {
    claw: Claw,
    score: usize,
    current_level: Level,
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
            x: self.body[0].x,
            y: self.body[0].y,
            w: 8,
            h: 16,
            rot: self.dir,
        }
    }

    pub fn to_rect(self) -> Rect {
        Rect {
            x: self.body[0].x - (TILE_SZ as f32 / 2.0),
            y: self.body[0].y - (TILE_SZ as f32 / 2.0),
            h: TILE_SZ as u16,
            w: TILE_SZ as u16, 
        }
    }

    pub fn chain_transform(&self, index: usize) -> Transform {
        Transform {
            x: self.body.get(index).unwrap().x,
            y: self.body.get(index).unwrap().y,
            w: 8,
            h: 8,
            rot: self.dir,
        }
    }
}

struct Object {
    pos: Vec2,
    e_type: EntityType,
    is_moving: bool,
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

    pub fn to_rect(self) -> Rect {
        Rect {
            x: self.pos.x - (TILE_SZ as f32 / 2.0),
            y: self.pos.y - (TILE_SZ as f32 / 2.0),
            h: TILE_SZ as u16,
            w: TILE_SZ as u16, 
        }
    }
}

struct Contact {
    rect_a: Rect,
    index_a: usize,
    rect_b: Rect,
    index_b: usize,
    overlap: Vec2,
}

// 8 by 8 coordinates, related to Goldminer_tilesheet1
const CLAW: [SheetRegion; 1] = [SheetRegion::rect(1, 56, 8, 16)];
const GOLD: [SheetRegion; 1] = [SheetRegion::rect(1, 10, 8, 8)];
const SILVER: [SheetRegion; 1] = [SheetRegion::rect(1, 19, 8, 8)];
const ROCK: [SheetRegion; 1] = [SheetRegion::rect(1, 27, 8, 8)];
const GEM: [SheetRegion; 1] = [SheetRegion::rect(1, 47, 8, 8)];
const CHAIN: [SheetRegion; 1] = [SheetRegion::rect(1, 74, 8, 8)];

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
            TILE_SZ,
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
        claw_body.push_back(Vec2 {
            x: TILE_SZ as f32 * 15.0,
            y: TILE_SZ as f32 * 25.0,
        });
        let mut entities: Vec<Object> = vec![];
        for (etype, pos) in level.starts().iter() {
            match etype {
                EntityType::Claw => {}
                EntityType::Rock => entities.push(Object {
                    pos: *pos,
                    e_type: EntityType::Rock,
                    is_moving: false,
                }),
                EntityType::Gem => entities.push(Object {
                    pos: *pos,
                    e_type: EntityType::Gem,
                    is_moving: false,
                }),
                EntityType::Gold => entities.push(Object {
                    pos: *pos,
                    e_type: EntityType::Gold,
                    is_moving: false,
                }),
                EntityType::Silver => entities.push(Object {
                    pos: *pos,
                    e_type: EntityType::Silver,
                    is_moving: false,
                }),
                EntityType::Snake => {}
                EntityType::Food => {} // EntityType::Door(_rm, _x, _y) => {}
                                       // EntityType::Enemy => self.enemies.push(Pos {
                                       //     pos: *pos,
                                       //     dir: Dir::S,
                                       // }),
            }
        }
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

        for (i, chain) in self.claw.body.iter().enumerate() {
            if i == 0 {
                continue;
            }
            frend.draw_sprite(0, self.claw.chain_transform(i), CHAIN[0]);
        }
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
                    if self.claw.dir < -1.0 {
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
                    let new_x: f32 = curr_x + CHAIN_SIZE * f32::cos(self.claw.dir - PI / 2.0);
                    let new_y: f32 = curr_y + CHAIN_SIZE * f32::sin(self.claw.dir - PI / 2.0);
                    self.claw.body.push_front(Vec2 { x: new_x, y: new_y });
                } else
                // retract claw
                {
                    if self.claw.body.len() > 1 {
                        self.claw.body.pop_front();
                    } else {
                        self.claw.is_deployed = false;
                        self.claw.claw_dir = true;
                    }
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
            for entity in self.entities.iter_mut() {
                if self.claw.body.contains(&entity.pos) {
                    self.claw.claw_dir = !self.claw.claw_dir;
                    entity.is_moving = true;
                }
            }

            for entity in self.entities.iter_mut() {
                if entity.is_moving == true {}
            }

            self.frame_counter = 0;
        }
        let mut object_contacts: Vec<Contact> = Vec::new();
        let mut object_to_remove: Vec<Contact> = Vec::new();
        let object_rects: Vec<Rect> = self.entities.iter().map(|&pos| pos.to_rect()).collect();
        let claw_rect: Rect = self.claw.to_rect();

        
        gather_contact(&claw_rect, &object_rects, &mut object_contacts);
    

    // fn gather_contact(a_rects: &[Rect], b_rects: &[Rect], contacts_list: &mut Vec<Contact>) {
    //     for (i, a_rect) in a_rects.iter().enumerate() {
    //         for (j, b_rect) in b_rects.iter().enumerate() {
    //             if let Some(overlap) = a_rect.overlap(*b_rect) {
    //                 contacts_list.push(Contact {
    //                     index_a: i,
    //                     rect_a: *a_rect,
    //                     index_b: j,
    //                     rect_b: *b_rect,
    //                     overlap: overlap,
    //                 })
    //             }
    //         }
    //     }
    // }
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


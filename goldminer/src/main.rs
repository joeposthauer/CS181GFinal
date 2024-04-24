use assets_manager::{asset::Png, AssetCache};
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

struct Game {
    claw: Claw,
    score: usize,
    current_level: usize,
    levels: Vec<Level>,
    entities: Vec<EntityType>,
    timer: usize,
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

struct Contact {
    rect_a: Rect,
    index_a: usize,
    rect_b: Rect,
    index_b: usize,
    overlap: Vec2,
}



// we need to define where in tilesheet we are representing each of these
const CLAW: [SheetRegion; 1] = [SheetRegion::rect(533, 39, 4, 4)];
const GOLD: [SheetRegion; 1] = [SheetRegion::rect(190, 345, 4, 4)];
const SILVER: [SheetRegion; 1] = [SheetRegion::rect(190, 345, 4, 4)];
const ROCK: [SheetRegion; 1] = [SheetRegion::rect(190, 345, 4, 4)];

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
            .load::<Png>("tilesheet")
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
        let mut snake_body: VecDeque<Vec2> = VecDeque::new();
        // for i in 0i8..5 {
        //     let i = f32::from(i);
        //     snake_body.push_back(Vec2 {
        //         x: 100.0 + i * 4.0,
        //         y: 100.0,
        //     })
        // }
        // let mut game = Game {
        //     started: true,
        //     snake: Snake {
        //         dir: (Dir::Right),
        //         body: (snake_body),
        //     },
        //     apple: Apple {
        //         pos: Vec2 { x: 50.0, y: 50.0 },
        //     },
        //     level: level,
        //     frame_counter: 0,
        //     move_interval: 5,
        // };
        // game
    }

    fn render(&mut self, frend: &mut Immediate) {
        self.level.render_immediate(frend);
        frend.draw_sprite(0, self.apple.transform(), FOOD[0]);
        let mut count: usize = 0;
        frend.draw_sprite(0, self.snake.transform(count), SNAKE[0]);
        for vector2 in self.snake.body.iter() {
            if (count == 0) {
                count = count + 1;
                continue;
            };
            frend.draw_sprite(0, self.snake.transform(count), SNAKE[1]);
            count = count + 1;
        }
    }

    fn simulate(&mut self, input: &Input, dt: f32) {
        self.frame_counter += 1;
        if self.frame_counter >= self.move_interval {
            if input.is_key_down(Key::ArrowLeft) && self.snake.dir != Dir::Right {
                self.snake.dir = Dir::Left;
            } 
            let head_pos = self
                .snake
                .body
                .front()
                .expect("Snake body is empty")
                .clone();
            let new_head_pos = head_pos + self.snake.dir.to_vec2();
            // coliision with the wall - restart game
            if new_head_pos.x < 0.0
                || new_head_pos.y < 0.0
                || new_head_pos.x >= W as f32
                || new_head_pos.y >= H as f32
            {
                self.restart();
                return;
            }

            if self.snake.body.contains(&new_head_pos) {
                self.restart();
                return;
            }
            if new_head_pos == self.apple.pos {
                // 3 times to growth is a more noticlable
                self.snake.body.push_front(new_head_pos);
                self.snake.body.push_front(new_head_pos);
                self.snake.body.push_front(new_head_pos);
                self.relocate_apple();
            } else {
                self.snake.body.push_front(new_head_pos);
                self.snake.body.pop_back();
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
use assets_manager::{asset::Png, AssetCache};
use frenderer::{
    input::{Input, Key},
    sprites::{Camera2D, SheetRegion, Transform},
    wgpu, Renderer,
};
use rand::Rng;
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
const DT: f32 = 1.0 / 60.0;

struct Contact {
    rect_a: Rect,
    index_a: usize,
    rect_b: Rect,
    index_b: usize,
    overlap: Vec2,
}

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
        move |window, mut frend| {
            let game = Game::new(&mut frend, &cache);
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
                    if !frend.gpu.is_web() {
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
            snake: vec![Vec2 { x: 0.0, y: 0.0 }],
        };
        game
    }
    fn render(&mut self, frend: &mut Renderer) {
        // make this exactly as big as we need
        // frend.sprite_group_resize(0, self.sprite_count());

        // let sprites_used = self.level().render_into(frend, 0);
        // let (sprite_posns, sprite_gfx) = frend.sprites_mut(0, sprites_used..);

        // for (enemy, (trf, uv)) in self
        //     .enemies
        //     .iter()
        //     .zip(sprite_posns.iter_mut().zip(sprite_gfx.iter_mut()))
        // {
        //     *trf = Transform {
        //         w: TILE_SZ as u16,
        //         h: TILE_SZ as u16,
        //         x: enemy.pos.x,
        //         y: enemy.pos.y,
        //         rot: 0.0,
        //     };
        //     *uv = ENEMY[enemy.dir as usize];
        // }
        // let sprite_posns = &mut sprite_posns[self.enemies.len()..];
        // let sprite_gfx = &mut sprite_gfx[self.enemies.len()..];
        // sprite_posns[0] = Transform {
        //     w: TILE_SZ as u16,
        //     h: TILE_SZ as u16,
        //     x: self.player.pos.x,
        //     y: self.player.pos.y,
        //     rot: 0.0,
        // };
        // sprite_gfx[0] = PLAYER[self.player.dir as usize].with_depth(1);
        // if self.attack_area.is_empty() {
        //     sprite_posns[1] = Transform::ZERO;
        // } else {
        //     let (w, h) = match self.player.dir {
        //         Dir::N | Dir::S => (16, 8),
        //         _ => (8, 16),
        //     };
        //     let delta = self.player.dir.to_vec2() * 7.0;
        //     sprite_posns[1] = Transform {
        //         w,
        //         h,
        //         x: self.player.pos.x + delta.x,
        //         y: self.player.pos.y + delta.y,
        //         rot: 0.0,
        //     };
        // }
        // sprite_gfx[1] = PLAYER_ATK[self.player.dir as usize].with_depth(0);

        // let mut heart_x = 10.0; // Start 10 pixels from the left edge
        // let heart_y = 10.0; // 10 pixels from the top edge

        // let sprite_posns = &mut sprite_posns[2..];
        // let sprite_gfx = &mut sprite_gfx[2..];

        // for i in 0..3 {
        //     sprite_posns[i] = (Transform {
        //         w: HEART.w as u16, // Width of heart sprite
        //         h: HEART.h as u16, // Height of heart sprite
        //         x: heart_x + (i * 10) as f32,
        //         y: heart_y,
        //         rot: 0.0, // No rotation
        //     });
        //     sprite_gfx[i] = HEART;
        // }
    }

    fn simulate(&mut self, input: &Input, dt: f32) {
        // if self.attack_timer > 0.0 {
        //     self.attack_timer -= dt;
        // }
        // if self.knockback_timer > 0.0 {
        //     self.knockback_timer -= dt;
        // }
        // Added
        // for contact in player_enemy_contacts {
        //     if let Some(overlap) = self.player.to_rect().overlap(self.enemies[contact.index_b].rect()) {
        //         if self.player.knockback_timer <= 0.0 {
        //             // set the knockback timer
        //             //  reduce life
        //         }
        //     }
        // }
        // End adding
        let mut dx = input.key_axis(Key::ArrowLeft, Key::ArrowRight) * DT;
        // now down means -y and up means +y!  beware!
        let mut dy = input.key_axis(Key::ArrowDown, Key::ArrowUp) * DT;
        // let attacking = !self.attack_area.is_empty();
        // let knockback = self.knockback_timer > 0.0;
        // if attacking {
        //     // while attacking we can't move
        //     dx = 0.0;
        //     dy = 0.0;
        // } else if knockback {
        //     // during knockback we move but don't turn around
        //     let delta = self.player.dir.to_vec2();
        //     dx = -delta.x * KNOCKBACK_SPEED * dt;
        //     dy = -delta.y * KNOCKBACK_SPEED * dt;
        // } else {
        //     // not attacking, no knockback, do normal movement
        //     if dx > 0.0 {
        //         self.player.dir = Dir::E;
        //     }
        //     if dx < 0.0 {
        //         self.player.dir = Dir::W;
        //     }
        //     if dy > 0.0 {
        //         self.player.dir = Dir::N;
        //     }
        //     if dy < 0.0 {
        //         self.player.dir = Dir::S;
        //     }
        // }
        // if self.attack_timer <= 0.0 && input.is_key_pressed(Key::Space) {
        //     // TODO POINT: compute the attack area's center based on the player's position and facing and some offset
        //     // For the spritesheet provided, the attack is placed 8px "forwards" from the player.
        //     let direction_vector = self.player.dir.to_vec2();
        //     // Assuming the attack extends 16 pixels from the player's center and is 16x16 in size.
        //     // Adjust values as needed.
        //     let attack_center_offset = 8.0;
        //     let attack_size = Vec2 { x: 16.0, y: 16.0 };
        //     let attack_center = self.player.pos + (direction_vector * attack_center_offset);

        //     self.attack_area = Rect {
        //         x: attack_center.x - attack_size.x / 2.0,
        //         y: attack_center.y - attack_size.y / 2.0,
        //         w: attack_size.x as u16,
        //         h: attack_size.y as u16,
        //     };

        //     self.attack_timer = ATTACK_MAX_TIME;
        // } else if self.attack_timer <= ATTACK_COOLDOWN_TIME {
        //     // "turn off" the attack, but the cooldown is still going
        //     self.attack_area = Rect {
        //         x: 0.0,
        //         y: 0.0,
        //         w: 0,
        //         h: 0,
        //     };
        // }

        // let dest = self.player.pos + Vec2 { x: dx, y: dy };
        // self.player.pos = dest;

        let mut rng = rand::thread_rng();
        // for enemy in self.enemies.iter_mut() {
        //     if rng.gen_bool(0.05) {
        //         enemy.dir = match rng.gen_range(0..4) {
        //             0 => Dir::N,
        //             1 => Dir::E,
        //             2 => Dir::S,
        //             3 => Dir::W,
        //             _ => panic!(),
        //         };
        //     }
        //     enemy.pos += enemy.dir.to_vec2() * ENEMY_SPEED * dt;
        // }

        // function to gather tile-entity contacts

        // WILL NEED THIS
        // pub fn gather_tile_contacts(rects: &[Rect], level: &Level, contacts: &mut Vec<Contact>) {
        //     for (i, rect) in rects.iter().enumerate() {
        //         for (tr, _) in level.tiles_within(*rect).filter(|(_tr, td)| td.solid) {
        //             if let Some(overlap) = rect.overlap(tr) {
        //                 contacts.push(Contact {
        //                     index_a: i,
        //                     rect_a: *rect,
        //                     index_b: 0,
        //                     rect_b: tr,
        //                     overlap: overlap,
        //                 })
        //             }
        //         }
        //     }
        // }
        // function to gather entity-entity contacts
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

        //create empty vecs for different contacts
        let mut em_contacts: Vec<Contact> = Vec::new();
        let mut pl_contacts: Vec<Contact> = Vec::new();
        let mut pl_en_contacts: Vec<Contact> = Vec::new();
        let mut enemies_to_remove: Vec<usize> = Vec::new();
        // Go from vec of enemies to vec of rectangles
        // let em_rects: Vec<Rect> = self.enemies.iter().map(|&pos| pos.to_rect()).collect();
        // // Get player's Rectengle
        // let player_rect: Rect = self.player.to_rect();

        // // gather enemy-tile contacts
        // gather_tile_contacts(&em_rects, &self.level(), &mut em_contacts);

        // // gather player-tile contacts
        // gather_tile_contacts(&[player_rect], &self.level(), &mut pl_contacts);

        // // gather player attack area-enemy contacts
        // gather_contact(
        //     &[player_rect, self.attack_area],
        //     &em_rects,
        //     &mut pl_en_contacts,
        // );

        // // sort enemy contacts vector
        // em_contacts.sort_by(|a, b| {
        //     b.overlap.mag_sq().partial_cmp(&a.overlap.mag_sq()).unwrap()
        //     // _or(std::cmp::Ordering::Equal)
        // });
        // // sort player contacts vector
        // em_contacts.sort_by(|a, b| b.overlap.mag_sq().partial_cmp(&a.overlap.mag_sq()).unwrap());
        // // sort player-enemy contacts vector
        // pl_en_contacts.sort_by(|a, b| b.overlap.mag_sq().partial_cmp(&a.overlap.mag_sq()).unwrap());

        // // deal with enemy-tile contact
        // for c in em_contacts.drain(..) {
        //     let displacement: Vec2 =
        //         compute_displacement(self.enemies[c.index_a].to_rect(), c.rect_b);
        //     self.enemies[c.index_a].pos += displacement;
        // }

        // // deal with player-tile contact
        // for c in pl_contacts.drain(..) {
        //     let displacement: Vec2 = compute_displacement(self.player.to_rect(), c.rect_b);
        //     self.player.pos += displacement;
        // }

        // for c in pl_en_contacts.drain(..) {
        //     if attacking && !enemies_to_remove.contains(&c.index_b) {
        //         enemies_to_remove.push(c.index_b);
        //     }
        // }
        // enemies_to_remove.sort();
        // for index in enemies_to_remove.iter().rev() {
        //     self.enemies.swap_remove(*index);
        // }

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
}

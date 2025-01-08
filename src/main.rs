use std::{
    f32::consts::{PI, TAU},
    path::Path,
};

use macroquad::{
    prelude::*,
    rand::{self, gen_range},
    ui::{root_ui, widgets::Window, Skin},
};

const DEBUG: bool = false;

const BACKGROUND_COLOR: Color = BLACK;
const STAR_COLORS: [Color; 3] = [WHITE, LIGHTGRAY, GRAY];
const STAR_NUM: usize = 600;
const STAR_MAX_SIZE: f32 = 1.2;

const INITIAL_LIVES: usize = 3;
const HEART_VERTICIES: [Vec2; 8] = [
    Vec2::new(0.0, -0.5),
    Vec2::new(0.5, -1.0),
    Vec2::new(1.0, -0.5),
    Vec2::new(1.0, -0.25),
    Vec2::new(0.0, 1.0),
    Vec2::new(-1.0, -0.25),
    Vec2::new(-1.0, -0.5),
    Vec2::new(-0.5, -1.0),
];
const HEART_RADIUS: f32 = 16.0;
const LIFE_SCORE: usize = 10_000;

const SHIP_COLOR: Color = SKYBLUE;
const SHIP_WIDTH: f32 = 22.0;
const SHIP_HEIGHT: f32 = 28.0;
const SHIP_COLLISION_RADIUS: f32 = 10.0;
const SHIP_ROTATION_SPEED: f32 = 0.4 * TAU;
const SHIP_MAX_SPEED: f32 = 80.0;
const SHIP_ACCELERATION: f32 = 200.0;
const SHIP_DRAG: f32 = 0.02;
const SHIP_HYPERSPACE_FREQUENCY: f32 = 2.0;
const SHIP_HYPERSPACE_MIN_DISTANCE: f32 = 100.0;
const SHIP_HYPERSPACE_SPEED: f32 = 300.0;
const SHIP_SHIELD_TIME: f32 = 3.0;
const SHIP_SHIELD_COLOR: Color = BLUE;

const SMALL_ASTEROID_SIZE: f32 = 12.0;
const SMALL_ASTEROID_SPEED: f32 = 130.0;
const SMALL_ASTEROID_SCORE: usize = 100;

const MEDIUM_ASTEROID_SIZE: f32 = 20.0;
const MEDIUM_ASTEROID_SPEED: f32 = 75.0;
const MEDIUM_ASTEROID_SCORE: usize = 50;

const LARGE_ASTEROID_SIZE: f32 = 40.0;
const LARGE_ASTEROID_SPEED: f32 = 40.0;
const LARGE_ASTEROID_SCORE: usize = 20;

const ASTEROID_SPLIT_ANGLE: f32 = PI / 6.0;
const ASTEROID_MIN_SPAWN_RATE: f32 = 0.5;
const ASTEROID_INITIAL_MAX_SPAWN_RATE: f32 = 5.0;
const ASTEROID_SPAWN_DECREASE_FACTOR: f32 = 0.001;

const PARTICLE_SIZE: f32 = 5.0;

const ASTEROID_PARTICLE_SPAWN: usize = 3;
const ASTEROID_PARTICLE_TTL: f32 = 1.5;
const ASTEROID_PARTICLE_COLOR: Color = MAROON;
const ASTEROID_PARTICLE_SPEED: f32 = 40.0;

const ASTEROID_COLOR: Color = DARKGRAY;
const ASTEROID_MIN_VERTICIES: usize = 8;
const ASTEROID_MAX_VERTICIES: usize = 12;
const ASTEROID_MIN_RADIUS: f32 = 0.8;
const ASTEROID_MAX_RADIUS: f32 = 1.1;

const LARGE_SAUCER_SIZE: f32 = 25.0;
const LARGE_SAUCER_SPEED: f32 = 32.0;
const LARGE_SAUCER_SCORE: usize = 200;

const SMALL_SAUCER_SIZE: f32 = 15.0;
const SMALL_SAUCER_SPEED: f32 = 50.0;
const SMALL_SAUCER_SCORE: usize = 1000;

const SAUCER_SMALL_SCORE_THRESHOLD: usize = 10_000;
const SAUCER_SMALL_MAX_PROBABILTY: f32 = 0.8;
const SAUCER_COLOR: Color = DARKPURPLE;
const SAUCER_SPAWN_RATE: f32 = 10.0;
const SAUCER_MAX: usize = 3;
const SAUCER_MAX_PER_WAVE: usize = 5;
const SAUCER_BULLET_FREQUENCY: f32 = 2.0;
const SAUCER_BULLET_COLOR: Color = PURPLE;
const SAUCER_BULLET_TTL: f32 = 3.0;
const SAUCER_VERTICIES: [Vec2; 10] = [
    Vec2::new(1.1, 0.2),
    Vec2::new(0.4, 0.7),
    Vec2::new(-0.4, 0.7),
    Vec2::new(-1.1, 0.2),
    Vec2::new(-0.3, -0.2),
    Vec2::new(-0.2, -0.7),
    Vec2::new(0.2, -0.7),
    Vec2::new(0.3, -0.2),
    Vec2::new(1.1, 0.2),
    Vec2::new(-1.1, 0.2),
];

const SAUCER_PARTICLE_SPAWN: usize = 3;
const SAUCER_PARTICLE_TTL: f32 = 1.5;
const SAUCER_PARTICLE_COLOR: Color = DARKPURPLE;
const SAUCER_PARTICLE_SPEED: f32 = 40.0;

const BULLET_COLOR: Color = LIME;
const BULLET_SIZE: f32 = 5.0;
const BULLET_SPEED: f32 = 150.0;
const BULLET_FREQUENCY: f32 = 0.2;
const MAX_BULLETS: usize = 4;

#[derive(Default)]
struct Translation {
    from: Vec2,
    to: Vec2,
    duration: f32,
    current_time: f32,
}

impl Translation {
    fn get(&self) -> Vec2 {
        self.from + (self.to - self.from) * (self.current_time / self.duration)
    }
}

enum ShipState {
    Normal,
    Hyperdrive,
    Shielded,
}

impl ShipState {
    fn is_translating(&self) -> bool {
        matches!(self, Self::Hyperdrive)
    }

    fn is_invincible(&self) -> bool {
        !matches!(self, Self::Normal)
    }
}

struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
    acc: Vec2,
    color: Color,
    state: ShipState,
    current_translation: Translation,
    shield_time: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            pos: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            rot: 0.0,
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            color: SHIP_COLOR,
            state: ShipState::Normal,
            current_translation: Translation {
                ..Default::default()
            },
            shield_time: 0.0,
        }
    }
}

impl Ship {
    fn get_tri(&self) -> (Vec2, Vec2, Vec2) {
        (
            // Ship Nose
            Vec2::new(
                self.pos.x + self.rot.sin() * (SHIP_HEIGHT - SHIP_COLLISION_RADIUS),
                self.pos.y - self.rot.cos() * (SHIP_HEIGHT - SHIP_COLLISION_RADIUS),
            ),
            // Left Base
            Vec2::new(
                self.pos.x
                    - self.rot.cos() * SHIP_WIDTH / 2.
                    - self.rot.sin() * SHIP_COLLISION_RADIUS,
                self.pos.y - self.rot.sin() * SHIP_WIDTH / 2.
                    + self.rot.cos() * SHIP_COLLISION_RADIUS,
            ),
            // Right Base
            Vec2::new(
                self.pos.x + self.rot.cos() * SHIP_WIDTH / 2.
                    - self.rot.sin() * SHIP_COLLISION_RADIUS,
                self.pos.y
                    + self.rot.sin() * SHIP_WIDTH / 2.
                    + self.rot.cos() * SHIP_COLLISION_RADIUS,
            ),
        )
    }

    fn get_unit_direction(&self) -> Vec2 {
        Vec2::new(self.rot.sin(), -self.rot.cos())
    }
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    collided: bool,
}

enum AsteroidType {
    Small,
    Medium,
    Large,
}

impl AsteroidType {
    fn size(&self) -> f32 {
        match self {
            AsteroidType::Small => SMALL_ASTEROID_SIZE,
            AsteroidType::Medium => MEDIUM_ASTEROID_SIZE,
            AsteroidType::Large => LARGE_ASTEROID_SIZE,
        }
    }

    fn speed(&self) -> f32 {
        match self {
            AsteroidType::Small => SMALL_ASTEROID_SPEED,
            AsteroidType::Medium => MEDIUM_ASTEROID_SPEED,
            AsteroidType::Large => LARGE_ASTEROID_SPEED,
        }
    }
    fn score(&self) -> usize {
        match self {
            AsteroidType::Small => SMALL_ASTEROID_SCORE,
            AsteroidType::Medium => MEDIUM_ASTEROID_SCORE,
            AsteroidType::Large => LARGE_ASTEROID_SCORE,
        }
    }
}

struct Asteroid {
    size: AsteroidType,
    pos: Vec2,
    vel: Vec2,
    collided: bool,
    verticies: Vec<Vec2>,
}

impl Default for Asteroid {
    fn default() -> Self {
        Self {
            size: AsteroidType::Large,
            pos: random_screen_edge_position(),
            vel: random_unit_vector() * AsteroidType::Large.speed(),
            collided: false,
            verticies: generate_asteroid_vertices(),
        }
    }
}

impl Asteroid {
    fn split(&self) -> Option<Vec<Asteroid>> {
        if let AsteroidType::Small = self.size {
            return None;
        };

        let vel = self.vel.normalize();

        let split_velocities = vec![
            Vec2::new(
                vel.x * ASTEROID_SPLIT_ANGLE.cos() - vel.y * ASTEROID_SPLIT_ANGLE.sin(),
                vel.x * ASTEROID_SPLIT_ANGLE.sin() + vel.y * ASTEROID_SPLIT_ANGLE.cos(),
            ),
            Vec2::new(
                vel.x * ASTEROID_SPLIT_ANGLE.cos() + vel.y * ASTEROID_SPLIT_ANGLE.sin(),
                -vel.x * ASTEROID_SPLIT_ANGLE.sin() + vel.y * ASTEROID_SPLIT_ANGLE.cos(),
            ),
        ];

        match self.size {
            AsteroidType::Large => Some(
                split_velocities
                    .into_iter()
                    .map(|v| Asteroid {
                        size: AsteroidType::Medium,
                        pos: self.pos,
                        vel: v * AsteroidType::Medium.speed(),
                        ..Default::default()
                    })
                    .collect(),
            ),
            AsteroidType::Medium => Some(
                split_velocities
                    .into_iter()
                    .map(|v| Asteroid {
                        size: AsteroidType::Small,
                        pos: self.pos,
                        vel: v * AsteroidType::Small.speed(),
                        ..Default::default()
                    })
                    .collect(),
            ),
            _ => None,
        }
    }
}

fn generate_asteroid_vertices() -> Vec<Vec2> {
    let num_vertices = rand::gen_range(ASTEROID_MIN_VERTICIES, ASTEROID_MAX_VERTICIES);
    (0..num_vertices)
        .map(|v| {
            let a = rand::gen_range(
                (v as f32) * TAU / (num_vertices as f32),
                ((v + 1) as f32) * TAU / (num_vertices as f32),
            );
            let r = rand::gen_range(ASTEROID_MIN_RADIUS, ASTEROID_MAX_RADIUS);
            Vec2::new(r * a.cos(), r * a.sin())
        })
        .collect()
}

enum SaucerSize {
    Large,
    Small,
}

impl SaucerSize {
    fn from_score(score: usize) -> Self {
        let prob = if score >= SAUCER_SMALL_SCORE_THRESHOLD {
            SAUCER_SMALL_MAX_PROBABILTY
        } else {
            SAUCER_SMALL_MAX_PROBABILTY * (score as f32) / (SAUCER_SMALL_SCORE_THRESHOLD as f32)
        };
        if rand::gen_range(0.0, 1.0) < prob {
            Self::Small
        } else {
            Self::Large
        }
    }

    fn size(&self) -> f32 {
        match self {
            Self::Large => LARGE_SAUCER_SIZE,
            Self::Small => SMALL_SAUCER_SIZE,
        }
    }

    fn speed(&self) -> f32 {
        match self {
            Self::Large => LARGE_SAUCER_SPEED,
            Self::Small => SMALL_SAUCER_SPEED,
        }
    }

    fn score(&self) -> usize {
        match self {
            Self::Large => LARGE_SAUCER_SCORE,
            Self::Small => SMALL_SAUCER_SCORE,
        }
    }
}

struct Saucer {
    size: SaucerSize,
    pos: Vec2,
    vel: Vec2,
    last_shot: f32,
    collided: bool,
}

impl Saucer {
    fn new(size: SaucerSize) -> Saucer {
        let speed = size.speed();
        Self {
            size,
            pos: random_screen_edge_position(),
            vel: random_unit_vector() * speed,
            last_shot: 0.0,
            collided: false,
        }
    }

    fn shoot(&self, ship: &Ship) -> Bullet {
        match self.size {
            SaucerSize::Large => Bullet {
                pos: self.pos,
                vel: random_unit_vector() * BULLET_SPEED,
                collided: false,
            },
            SaucerSize::Small => {
                let prediction_offset = ship.get_unit_direction() * ship.vel.length();
                let target = (ship.pos + prediction_offset - self.pos).normalize();

                Bullet {
                    pos: self.pos,
                    vel: target * BULLET_SPEED,
                    collided: false,
                }
            }
        }
    }
}

struct Particle {
    color: Color,
    ttl: f32,
    time: f32,
    pos: Vec2,
    vel: Vec2,
    size: f32,
}

fn wrap_screen(pos: &mut Vec2) {
    if pos.x < 0.0 {
        pos.x = screen_width();
    } else if pos.x > screen_width() {
        pos.x = 0.0;
    }
    if pos.y < 0.0 {
        pos.y = screen_height();
    } else if pos.y > screen_height() {
        pos.y = 0.0;
    }
}

fn draw_centered_text(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let text_center = get_text_center(text, None, font_size as u16, 1.0, 0.0);
    draw_text(text, x - text_center.x, y - text_center.y, font_size, color);
}

fn random_unit_vector() -> Vec2 {
    let rot = rand::gen_range(0.0, TAU);
    Vec2::new(rot.cos(), rot.sin())
}

fn random_screen_position() -> Vec2 {
    Vec2::new(
        rand::gen_range(0.0, screen_width()),
        gen_range(0.0, screen_height()),
    )
}

fn random_screen_edge_position() -> Vec2 {
    let side: i32 = rand::gen_range(0, 4);

    match side {
        // TOP
        0 => Vec2::new(rand::gen_range(0.0, screen_width()), 0.0),
        // RIGHT
        1 => Vec2::new(screen_width(), rand::gen_range(0.0, screen_height())),
        // BOTTOM
        2 => Vec2::new(rand::gen_range(0.0, screen_width()), screen_height()),
        // LEFT
        _ => Vec2::new(0.0, rand::gen_range(0.0, screen_height())),
    }
}

fn draw_asteroid(a: &Asteroid) {
    a.verticies
        .iter()
        .zip(a.verticies.iter().cycle().skip(1))
        .for_each(|(v1, v2)| {
            draw_line(
                a.pos.x + a.size.size() * v1.x,
                a.pos.y + a.size.size() * v1.y,
                a.pos.x + a.size.size() * v2.x,
                a.pos.y + a.size.size() * v2.y,
                2.0,
                ASTEROID_COLOR,
            );
        });
}

fn draw_saucer(s: &Saucer) {
    SAUCER_VERTICIES
        .iter()
        .zip(SAUCER_VERTICIES.iter().cycle().skip(1))
        .for_each(|(v1, v2)| {
            draw_line(
                s.pos.x + s.size.size() * v1.x,
                s.pos.y + s.size.size() * v1.y,
                s.pos.x + s.size.size() * v2.x,
                s.pos.y + s.size.size() * v2.y,
                2.0,
                SAUCER_COLOR,
            );
        });
}

fn draw_heart(p: Vec2) {
    HEART_VERTICIES
        .iter()
        .zip(HEART_VERTICIES.iter().cycle().skip(1))
        .for_each(|(v1, v2)| {
            draw_line(
                p.x + HEART_RADIUS * v1.x,
                p.y + HEART_RADIUS * v1.y,
                p.x + HEART_RADIUS * v2.x,
                p.y + HEART_RADIUS * v2.y,
                HEART_RADIUS / 6.0,
                RED,
            );
        });
}

#[derive(Default)]
struct Game {
    ship: Ship,
    last_hyperspace: f32,
    lives: usize,
    lives_awarded: usize,

    bullets: Vec<Bullet>,
    last_bullet: f32,

    asteroids: Vec<Asteroid>,
    asteroid_wave: usize,
    asteroids_spawned_in_wave: usize,
    last_asteroid: f32,
    max_asteroid_spawn_rate: f32,
    next_asteroid_spawn_rate: f32,

    saucers: Vec<Saucer>,
    last_saucer: f32,
    saucers_spawned_in_wave: usize,

    // bullet, time alive
    saucer_bullets: Vec<(Bullet, f32)>,

    game_over: bool,
    score: usize,

    particles: Vec<Particle>,

    frame: usize,
}

impl Game {
    fn get_wave_asteroid_amount(&self) -> usize {
        self.asteroid_wave * 2 + 2
    }
}

#[macroquad::main("Asteroids")]
async fn main() {
    let mut game = Game {
        lives: INITIAL_LIVES,
        max_asteroid_spawn_rate: ASTEROID_INITIAL_MAX_SPAWN_RATE,
        ..Default::default()
    };

    // use the small variance in start up time to seed the random number generator
    let time = (get_time() * 100_000_000_000.0) as u64;
    rand::srand(time);

    let star_map: Vec<(Vec2, f32)> = (0..STAR_NUM)
        .map(|_| {
            (
                Vec2::new(rand::gen_range(0.0, 1.0), rand::gen_range(0.0, 1.0)),
                rand::gen_range(0.5, STAR_MAX_SIZE),
            )
        })
        .collect();

    loop {
        if DEBUG && is_key_down(KeyCode::Escape) {
            break;
        }

        if game.game_over {
            clear_background(MAROON);

            draw_centered_text(
                &format!("Final Score: {}", game.score),
                screen_width() / 2.0,
                screen_height() / 2.0,
                48.0,
                BLACK,
            );

            if root_ui().button(
                Vec2::new(screen_width() / 2.0, screen_height() / 2.0 + 24.0),
                "Restart?",
            ) {
                game = Game {
                    lives: INITIAL_LIVES,
                    max_asteroid_spawn_rate: ASTEROID_INITIAL_MAX_SPAWN_RATE,
                    ..Default::default()
                };
            }

            next_frame().await;
            continue;
        }

        let delta_t = get_frame_time();
        game.frame += 1;

        // Asteroid Spawning
        game.last_asteroid += delta_t;
        if game.asteroids_spawned_in_wave < game.get_wave_asteroid_amount()
            && game.last_asteroid >= game.next_asteroid_spawn_rate
        {
            game.last_asteroid = 0.0;
            game.asteroids_spawned_in_wave += 1;
            game.next_asteroid_spawn_rate =
                rand::gen_range(ASTEROID_MIN_SPAWN_RATE, game.max_asteroid_spawn_rate);
            game.asteroids.push(Asteroid::default());
        }

        game.max_asteroid_spawn_rate -= (game.max_asteroid_spawn_rate - ASTEROID_MIN_SPAWN_RATE)
            * delta_t
            * ASTEROID_SPAWN_DECREASE_FACTOR;

        // Saucer Spawning

        game.last_saucer += delta_t;
        if game.saucers_spawned_in_wave < SAUCER_MAX_PER_WAVE
            && game.saucers.len() < SAUCER_MAX
            && game.last_saucer > SAUCER_SPAWN_RATE
        {
            game.last_saucer = 0.0;
            game.saucers_spawned_in_wave += 1;
            game.saucers
                .push(Saucer::new(SaucerSize::from_score(game.score)));
        }

        // Ship Logic

        game.last_bullet += delta_t;
        game.last_hyperspace += delta_t;

        if let ShipState::Shielded = game.ship.state {
            game.ship.shield_time += delta_t;
            if game.ship.shield_time > SHIP_SHIELD_TIME {
                game.ship.state = ShipState::Normal;
            }
        }

        if !game.ship.state.is_translating() {
            if is_key_down(KeyCode::A) {
                game.ship.rot -= SHIP_ROTATION_SPEED * delta_t;
            }
            if is_key_down(KeyCode::D) {
                game.ship.rot += SHIP_ROTATION_SPEED * delta_t;
            }

            if is_key_down(KeyCode::W) {
                game.ship.acc = game.ship.get_unit_direction() * SHIP_ACCELERATION;
                if game.frame % 5 == 0 {
                    game.particles.push(Particle {
                        color: ORANGE,
                        ttl: 0.2,
                        time: 0.0,
                        pos: game.ship.pos
                            - (game.ship.get_unit_direction() * SHIP_COLLISION_RADIUS * 0.7),
                        vel: Vec2::ZERO,
                        size: SHIP_WIDTH * 0.3,
                    });
                }
            } else {
                game.ship.acc = -SHIP_DRAG * game.ship.vel * game.ship.vel.length();
            }

            game.ship.vel += game.ship.acc * delta_t;

            if game.ship.vel.length() > SHIP_MAX_SPEED {
                game.ship.vel = game.ship.vel.normalize() * SHIP_MAX_SPEED;
            }

            game.ship.pos += game.ship.vel * delta_t;

            if is_key_down(KeyCode::Space)
                && game.last_bullet > BULLET_FREQUENCY
                && game.bullets.len() < MAX_BULLETS
            {
                game.last_bullet = 0.0;
                game.bullets.push(Bullet {
                    pos: game.ship.pos,
                    vel: game.ship.get_unit_direction() * BULLET_SPEED,
                    collided: false,
                });
            }

            if is_key_pressed(KeyCode::LeftShift)
                && game.last_hyperspace > SHIP_HYPERSPACE_FREQUENCY
            {
                game.last_hyperspace = 0.0;
                loop {
                    let pos = random_screen_position();
                    let dist = game.ship.pos.distance(pos);
                    if dist > SHIP_HYPERSPACE_MIN_DISTANCE {
                        game.ship.current_translation = Translation {
                            from: game.ship.pos,
                            to: pos,
                            duration: dist / SHIP_HYPERSPACE_SPEED,
                            ..Default::default()
                        };
                        game.ship.state = ShipState::Hyperdrive;
                        break;
                    }
                }
            }
        } else {
            game.ship.pos = game.ship.current_translation.get();
            game.ship.color.a =
                0.5 + ((game.ship.current_translation.current_time * 20.0).cos() as f32) * 0.5;

            game.ship.current_translation.current_time += delta_t;

            if game.ship.current_translation.current_time > game.ship.current_translation.duration {
                game.ship.state = ShipState::Normal;
                game.ship.color = SHIP_COLOR;
            }
        }

        wrap_screen(&mut game.ship.pos);

        // Game logic

        game.particles.iter_mut().for_each(|p| {
            p.pos += p.vel * delta_t;
            p.size = PARTICLE_SIZE * (1.0 - p.time / p.ttl);
            p.color.a = 1.0 - p.time / p.ttl;
            p.time += delta_t;
        });
        game.particles.retain(|p| p.time < p.ttl);

        game.bullets.iter_mut().for_each(|b| {
            b.pos += b.vel * delta_t;
            wrap_screen(&mut b.pos);
        });

        game.asteroids.iter_mut().for_each(|a| {
            a.pos += a.vel * delta_t;
            wrap_screen(&mut a.pos);
        });

        game.asteroids.iter_mut().for_each(|a| {
            game.bullets.iter_mut().for_each(|b| {
                let collided = a.pos.distance(b.pos).abs() < a.size.size() + BULLET_SIZE;
                a.collided = a.collided || collided;
                b.collided = b.collided || collided;
            });
        });

        game.saucers.iter_mut().for_each(|s| {
            s.pos += s.vel * delta_t;
            wrap_screen(&mut s.pos);
        });

        game.saucers.iter_mut().for_each(|s| {
            game.bullets.iter_mut().for_each(|b| {
                let collided = s.pos.distance(b.pos).abs() < s.size.size() + BULLET_SIZE;
                s.collided = s.collided || collided;
                b.collided = b.collided || collided;
            });
        });

        game.saucers.iter_mut().for_each(|s| {
            s.last_shot += delta_t;
            if s.last_shot > SAUCER_BULLET_FREQUENCY {
                s.last_shot = 0.0;
                let bullet = s.shoot(&game.ship);
                game.saucer_bullets.push((bullet, 0.0));
            }
        });

        game.saucer_bullets.iter_mut().for_each(|(b, t)| {
            b.pos += b.vel * delta_t;
            wrap_screen(&mut b.pos);

            *t += delta_t;
        });

        let mut ship_hit = false;

        game.asteroids.iter_mut().for_each(|a| {
            let collided = !game.ship.state.is_invincible()
                && game.ship.pos.distance(a.pos) < SHIP_COLLISION_RADIUS + a.size.size();
            ship_hit = ship_hit || collided;
            a.collided = a.collided || collided;
        });

        game.saucers.iter_mut().for_each(|s| {
            let collided = !game.ship.state.is_invincible()
                && game.ship.pos.distance(s.pos) < SHIP_COLLISION_RADIUS + s.size.size();
            ship_hit = ship_hit || collided;
            s.collided = s.collided || collided;
        });

        game.saucer_bullets.iter_mut().for_each(|(b, _)| {
            let collided = !game.ship.state.is_invincible()
                && game.ship.pos.distance(b.pos) < SHIP_COLLISION_RADIUS + BULLET_SIZE;
            ship_hit = ship_hit || collided;
            b.collided = b.collided || collided;
        });

        if ship_hit {
            game.ship.state = ShipState::Shielded;
            game.ship.shield_time = 0.0;
            if game.lives == 0 {
                game.game_over = true;
            } else {
                game.lives -= 1;
            }
        }

        let mut new_asteroid_particles = game
            .asteroids
            .iter()
            .filter(|a| a.collided)
            .flat_map(|a| {
                (0..ASTEROID_PARTICLE_SPAWN)
                    .map(|_| Particle {
                        color: ASTEROID_PARTICLE_COLOR,
                        ttl: ASTEROID_PARTICLE_TTL,
                        time: 0.0,
                        pos: a.pos,
                        vel: random_unit_vector() * ASTEROID_PARTICLE_SPEED,
                        size: PARTICLE_SIZE,
                    })
                    .collect::<Vec<Particle>>()
            })
            .collect();
        game.particles.append(&mut new_asteroid_particles);

        let mut new_saucer_particles = game
            .saucers
            .iter()
            .filter(|s| s.collided)
            .flat_map(|s| {
                (0..SAUCER_PARTICLE_SPAWN)
                    .map(|_| Particle {
                        color: SAUCER_PARTICLE_COLOR,
                        ttl: SAUCER_PARTICLE_TTL,
                        time: 0.0,
                        pos: s.pos,
                        vel: random_unit_vector() * SAUCER_PARTICLE_SPEED,
                        size: PARTICLE_SIZE,
                    })
                    .collect::<Vec<Particle>>()
            })
            .collect();
        game.particles.append(&mut new_saucer_particles);

        game.score += game
            .asteroids
            .iter()
            .filter(|a| a.collided)
            .map(|a| a.size.score())
            .sum::<usize>();
        game.score += game
            .saucers
            .iter()
            .filter(|s| s.collided)
            .map(|s| s.size.score())
            .sum::<usize>();
        if (game.score / LIFE_SCORE) > game.lives_awarded {
            game.lives += 1;
            game.lives_awarded += 1;
        }

        let mut new_asteroids: Vec<Asteroid> = game
            .asteroids
            .iter()
            .filter(|a| a.collided)
            .flat_map(|a| a.split())
            .flatten()
            .collect();

        game.asteroids.retain(|a| !a.collided);
        game.asteroids.append(&mut new_asteroids);
        game.bullets.retain(|b| !b.collided);
        game.saucers.retain(|s| !s.collided);
        game.saucer_bullets
            .retain(|(b, t)| !(b.collided || *t > SAUCER_BULLET_TTL));

        if game.frame % 10 == 0 {
            game.bullets.iter().for_each(|b| {
                game.particles.push(Particle {
                    color: BULLET_COLOR,
                    ttl: 0.2,
                    time: 0.0,
                    vel: Vec2::ZERO,
                    pos: b.pos,
                    size: BULLET_SIZE,
                });
            });
            game.saucer_bullets.iter().for_each(|(b, _)| {
                game.particles.push(Particle {
                    color: SAUCER_BULLET_COLOR,
                    ttl: 0.2,
                    time: 0.0,
                    vel: Vec2::ZERO,
                    pos: b.pos,
                    size: BULLET_SIZE,
                });
            });
        }

        if game.asteroids_spawned_in_wave == game.get_wave_asteroid_amount()
            && game.asteroids.len() == 0
        {
            game.asteroid_wave += 1;
            game.asteroids_spawned_in_wave = 0;
            game.saucers_spawned_in_wave = 0;
        }

        // Display
        clear_background(BACKGROUND_COLOR);
        star_map.iter().for_each(|(p, r)| {
            draw_circle(
                p.x * screen_width(),
                p.y * screen_height(),
                *r,
                STAR_COLORS[rand::gen_range(0, STAR_COLORS.len())],
            );
        });

        // Game
        game.particles
            .iter()
            .for_each(|p| draw_circle(p.pos.x, p.pos.y, p.size, p.color));

        let (v1, v2, v3) = game.ship.get_tri();
        draw_triangle(v1, v2, v3, game.ship.color);
        match game.ship.state {
            ShipState::Hyperdrive => draw_circle_lines(
                game.ship.current_translation.to.x,
                game.ship.current_translation.to.y,
                SHIP_COLLISION_RADIUS * 0.5,
                2.0,
                RED,
            ),
            ShipState::Shielded => {
                let mut shield_color = SHIP_SHIELD_COLOR;
                shield_color.a = 0.5 + (game.ship.shield_time * 20.0).cos() * 0.5;
                draw_circle_lines(
                    game.ship.pos.x,
                    game.ship.pos.y,
                    SHIP_COLLISION_RADIUS * 1.5,
                    3.0,
                    shield_color,
                )
            }
            _ => (),
        };

        game.asteroids.iter().for_each(draw_asteroid);

        game.saucers.iter().for_each(draw_saucer);

        game.bullets
            .iter()
            .for_each(|b| draw_circle(b.pos.x, b.pos.y, BULLET_SIZE, BULLET_COLOR));

        game.saucer_bullets
            .iter()
            .for_each(|(b, _)| draw_circle(b.pos.x, b.pos.y, BULLET_SIZE, SAUCER_BULLET_COLOR));

        // UI
        draw_centered_text(
            &format!("Score: {}", game.score),
            screen_width() / 2.0,
            24.0,
            32.0,
            WHITE,
        );

        for i in 0..game.lives {
            let x = if game.lives == 1 {
                0.0
            } else {
                ((2.0 * (i as f32)) / ((game.lives as f32) - 1.0)) - 1.0
            };
            draw_heart(Vec2::new(
                screen_width() / 2.0 + x * HEART_RADIUS * (game.lives as f32),
                60.0,
            ));
        }

        let (height, hyperspace_bar_colour) = if game.last_hyperspace > SHIP_HYPERSPACE_FREQUENCY {
            (30.0, GREEN)
        } else {
            (
                30.0 * game.last_hyperspace / SHIP_HYPERSPACE_FREQUENCY,
                YELLOW,
            )
        };
        draw_rectangle(
            screen_width() - 50.0,
            screen_height() - 20.0 - height,
            20.0,
            height,
            hyperspace_bar_colour,
        );
        draw_centered_text(
            "Hyperspace",
            screen_width() - 40.0,
            screen_height() - 10.0,
            16.0,
            WHITE,
        );

        // Debug last to draw on top
        if DEBUG {
            // Ship Ppsition
            draw_circle(game.ship.pos.x, game.ship.pos.y, 1.0, RED);
            // Ship collision
            draw_circle_lines(
                game.ship.pos.x,
                game.ship.pos.y,
                SHIP_COLLISION_RADIUS,
                1.0,
                RED,
            );
            // Ship velocity
            draw_line(
                game.ship.pos.x,
                game.ship.pos.y,
                game.ship.pos.x + game.ship.vel.x,
                game.ship.pos.y + game.ship.vel.y,
                2.0,
                BLUE,
            );
            // Ship acceleration
            draw_line(
                game.ship.pos.x,
                game.ship.pos.y,
                game.ship.pos.x + game.ship.acc.x * SHIP_MAX_SPEED / SHIP_ACCELERATION,
                game.ship.pos.y + game.ship.acc.y * SHIP_MAX_SPEED / SHIP_ACCELERATION,
                2.0,
                RED,
            );
            // Asteroid spawning info
            draw_text(
                &format!(
                    "Wave: {}({}). Spawned {} Asteroids, {} Saucers. Next Asteroid spawn: {:.2} (max {:.2})",
                    game.asteroid_wave, game.asteroids_spawned_in_wave, game.asteroids_spawned_in_wave, game.saucers_spawned_in_wave, game.next_asteroid_spawn_rate, game.max_asteroid_spawn_rate
                ),
                5.0,
                screen_height() - 10.0,
                16.0,
                RED,
            );
            // FPS
            draw_text(&format!("FPS: {}", get_fps()), 5.0, 20.0, 16.0, RED);
            // center lines
            draw_line(
                0.0,
                screen_height() / 2.0,
                screen_width(),
                screen_height() / 2.0,
                1.0,
                RED,
            );
            draw_line(
                screen_width() / 2.0,
                0.0,
                screen_width() / 2.0,
                screen_height(),
                1.0,
                RED,
            );
        }

        next_frame().await;
    }
}

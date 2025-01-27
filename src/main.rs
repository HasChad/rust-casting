use macroquad::prelude::*;

mod app_settings;
mod inits;

use app_settings::*;
use inits::*;

const FOV: i32 = 120;
const RAY_COUNT: i32 = FOV / 40; // fov / real ray count
const COLUMN_SIZE: i32 = WIDHT / RAY_COUNT;

struct Player {
    degree: f32,
    pos: Vec2,
    rays: Vec<Ray>,
}

impl Player {
    fn fmove_player(&mut self, direction: f32) {
        let forward = Vec2::new(
            self.degree.to_radians().cos(),
            self.degree.to_radians().sin(),
        )
        .normalize();

        self.pos += forward * direction * get_frame_time() * 200.0;
    }

    fn draw(&self) {
        let direction = vec2(
            self.degree.to_radians().cos(),
            self.degree.to_radians().sin(),
        );

        draw_line(
            self.pos.x,
            self.pos.y,
            self.pos.x + direction.x * 100.,
            self.pos.y + direction.y * 100.,
            3.0,
            RED,
        );
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player {
        degree: 0.0,
        pos: vec2(250.0, 250.0),
        rays: vec![],
    };
    let walls: Vec<Wall> = vec![
        Wall {
            a: vec2(200., 200.),
            b: vec2(200., 400.),
        },
        Wall {
            a: vec2(400., 400.),
            b: vec2(400., 200.),
        },
        Wall {
            a: vec2(200., 400.),
            b: vec2(400., 400.),
        },
        Wall {
            a: vec2(400., 200.),
            b: vec2(200., 200.),
        },
    ];

    for degree in -20..20 {
        player.rays.push(Ray::new(degree as f32 * 3.0, player.pos));
    }

    // ! Game Loop
    loop {
        player.rays = vec![];
        for degree in -20..20 {
            player
                .rays
                .push(Ray::new(player.degree + degree as f32 * 3.0, player.pos));
        }

        // head rotate
        if is_key_down(KeyCode::Left) {
            player.degree -= 100.0 * get_frame_time();
        }
        if is_key_down(KeyCode::Right) {
            player.degree += 100.0 * get_frame_time();
        }

        // player movement
        if is_key_down(KeyCode::Up) {
            player.fmove_player(1.0);
        }
        if is_key_down(KeyCode::Down) {
            player.fmove_player(-1.0);
        }

        // ! draw
        clear_background(BLACK);

        for ray in player.rays.iter_mut() {
            ray.check_wall(&walls);
            ray.draw();
        }

        for wall in walls.iter() {
            wall.draw();
        }

        player.draw();

        // draw_circle(pos.x, pos.y, 5., RED);

        next_frame().await
    }
}

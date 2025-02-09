use macroquad::prelude::*;

mod app_settings;
mod inits;

use app_settings::*;
use inits::*;

const RAY_COUNT: i32 = 120;
const FOV: f32 = 80.0 / RAY_COUNT as f32;

enum GameMode {
    FirstPerson,
    MiniMap,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_mode = GameMode::FirstPerson;
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

    for degree in (RAY_COUNT / -2)..(RAY_COUNT / 2) {
        player.rays.push(Ray::new(degree as f32 * FOV, player.pos));
    }

    // ! Game Loop
    loop {
        player.rays = vec![];
        for degree in (RAY_COUNT / -2)..(RAY_COUNT / 2) {
            player
                .rays
                .push(Ray::new(player.degree + degree as f32 * FOV, player.pos));
        }

        // head rotate
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            player.degree -= 100.0 * get_frame_time();
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            player.degree += 100.0 * get_frame_time();
        }

        // degree correction
        if player.degree < -0.0 {
            player.degree += 360.0;
        } else if player.degree > 360.0 {
            player.degree -= 360.0;
        }

        // player movement
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            player.fmove_player(1.0);
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            player.fmove_player(-1.0);
        }

        // gamemode changer
        if is_key_pressed(KeyCode::Tab) {
            match game_mode {
                GameMode::FirstPerson => game_mode = GameMode::MiniMap,
                GameMode::MiniMap => game_mode = GameMode::FirstPerson,
            }
        }

        for ray in player.rays.iter_mut() {
            ray.check_wall(&walls);
        }

        // ! draw
        match game_mode {
            GameMode::FirstPerson => {
                draw_rectangle(0.0, 0., screen_width(), screen_height() / 2.0, DARKBLUE);
                draw_rectangle(
                    0.0,
                    screen_height() / 2.0,
                    screen_width(),
                    screen_height() / 2.0,
                    DARKGREEN,
                );

                for (num, ray) in player.rays.iter_mut().enumerate() {
                    ray.draw_column(num, player.degree);
                }
            }
            GameMode::MiniMap => {
                clear_background(BLACK);

                for wall in walls.iter() {
                    wall.draw();
                }

                for ray in player.rays.iter_mut() {
                    ray.draw();
                }

                player.draw();
            }
        }

        next_frame().await
    }
}

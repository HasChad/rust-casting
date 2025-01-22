use macroquad::prelude::*;

mod app_settings;
mod inits;

use app_settings::*;
use inits::*;

const FOV: i32 = 120;
const RAY_COUNT: i32 = 40;
const COLUMN_SIZE: i32 = WIDHT / RAY_COUNT;

#[macroquad::main(window_conf)]
async fn main() {
    let mut rays: Vec<Ray> = vec![];
    let mut walls: Vec<Wall> = vec![
        Wall {
            a: vec2(100., 100.),
            b: vec2(100., 300.),
        },
        Wall {
            a: vec2(100., 300.),
            b: vec2(300., 300.),
        },
        Wall {
            a: vec2(300., 300.),
            b: vec2(300., 100.),
        },
        Wall {
            a: vec2(300., 100.),
            b: vec2(100., 100.),
        },
        Wall {
            a: vec2(300., 100.),
            b: vec2(100., 300.),
        },
    ];

    for degree in (0..360).step_by(360) {
        rays.push(Ray::new(degree));
    }

    // for _ in 0..5 {
    //     walls.push(Wall::new());
    // }

    // ! Game Loop
    loop {
        let (mouse_xpos, mouse_ypos) = mouse_position();

        if is_key_pressed(KeyCode::Space) {
            for wall in walls.iter_mut() {
                *wall = Wall::new();
            }
        }

        // ! draw
        clear_background(BLACK);

        for ray in rays.iter_mut() {
            ray.pos = vec2(mouse_xpos, mouse_ypos);
            ray.check_wall(&walls);
            ray.draw();
        }

        for wall in walls.iter() {
            wall.draw();
        }

        next_frame().await
    }
}

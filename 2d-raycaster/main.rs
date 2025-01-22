use macroquad::prelude::*;

mod app_settings;
mod inits;

use app_settings::*;
use inits::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut rays: Vec<Ray> = vec![];
    let mut walls: Vec<Wall> = vec![];

    for degree in (0..360).step_by(3) {
        rays.push(Ray::new(degree));
    }

    for _ in 0..5 {
        walls.push(Wall::new());
    }

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

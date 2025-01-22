use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

const RAY_LENGTH: f32 = 1000.0;

pub struct Ray {
    pub pos: Vec2,
    pub end: Vec2,
    pub dir: Vec2,
}

impl Ray {
    pub fn new(degree: i32) -> Ray {
        let direction = vec2(deg_to_rad(degree).cos(), deg_to_rad(degree).sin());

        info!("{}", direction);

        Ray {
            pos: vec2(0.0, 0.0),
            end: vec2(direction.x * RAY_LENGTH, direction.y * RAY_LENGTH),
            dir: direction,
        }
    }

    pub fn draw(&self) {
        draw_line(
            self.pos.x,
            self.pos.y,
            self.pos.x + self.end.x,
            self.pos.y + self.end.y,
            1.0,
            WHITE,
        );
    }

    pub fn check_wall(&mut self, walls: &[Wall]) {
        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection

        let mut min_distance = f32::INFINITY;
        let mut point = Vec2::ZERO;

        for wall in walls.iter() {
            let x1 = wall.a.x;
            let y1 = wall.a.y;
            let x2 = wall.b.x;
            let y2 = wall.b.y;

            let x3 = self.pos.x;
            let y3 = self.pos.y;
            let x4 = self.pos.x + self.dir.x;
            let y4 = self.pos.y + self.dir.y;

            let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
            if den == 0.0 {
                return;
            }

            let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
            let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

            if t > 0.0 && t < 1.0 && u > 0.0 {
                let pt = Vec2 {
                    x: x1 + t * (x2 - x1),
                    y: y1 + t * (y2 - y1),
                };

                let distance = ((pt.x - self.pos.x) * (pt.x - self.pos.x)
                    + (pt.y - self.pos.y) * (pt.y - self.pos.y))
                    .sqrt();

                if distance < min_distance {
                    min_distance = distance;
                    point = pt;
                }
            }
        }

        if min_distance != f32::INFINITY {
            self.end = point - self.pos;
        } else {
            self.end = vec2(self.dir.x * RAY_LENGTH, self.dir.y * RAY_LENGTH);
        }
    }
}

pub struct Wall {
    pub a: Vec2,
    pub b: Vec2,
}

impl Wall {
    pub fn new() -> Wall {
        let mut rng = thread_rng();

        Wall {
            a: Vec2 {
                x: rng.gen_range(0.0..screen_width()),
                y: rng.gen_range(0.0..screen_height()),
            },
            b: Vec2 {
                x: rng.gen_range(0.0..screen_width()),
                y: rng.gen_range(0.0..screen_height()),
            },
        }
    }

    pub fn draw(&self) {
        draw_line(self.a.x, self.a.y, self.b.x, self.b.y, 1.0, YELLOW);
    }
}

fn deg_to_rad(value: i32) -> f32 {
    value as f32 * core::f32::consts::TAU / 360.0
}

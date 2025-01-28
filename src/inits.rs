use macroquad::prelude::*;

const RAY_LENGTH: f32 = 1000.0;

pub struct Ray {
    pub pos: Vec2,
    pub end: Vec2,
    pub dir: Vec2,
}

impl Ray {
    pub fn new(degree: f32, pos: Vec2) -> Ray {
        let direction = vec2(degree.to_radians().cos(), degree.to_radians().sin());

        Ray {
            pos,
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
                continue;
            }

            let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
            let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

            if (0.0..=1.0).contains(&t) && u > 0.0 {
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

pub struct Player {
    pub degree: f32,
    pub pos: Vec2,
    pub rays: Vec<Ray>,
}

impl Player {
    pub fn fmove_player(&mut self, direction: f32) {
        let forward = Vec2::new(
            self.degree.to_radians().cos(),
            self.degree.to_radians().sin(),
        )
        .normalize();

        self.pos += forward * direction * get_frame_time() * 200.0;
    }

    pub fn draw(&self) {
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

pub struct Wall {
    pub a: Vec2,
    pub b: Vec2,
}

impl Wall {
    pub fn draw(&self) {
        draw_line(self.a.x, self.a.y, self.b.x, self.b.y, 1.0, YELLOW);
    }

    /*
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
    */
}

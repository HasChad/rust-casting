use macroquad::prelude::*;

use crate::COLUMN_WIDTH;

const RAY_LENGTH: f32 = 1000.0;

pub struct Ray {
    pub degree: f32,
    pub pos: Vec2,
    pub end: Option<Vec2>,
    pub dir: Vec2,
}

impl Ray {
    pub fn new(degree: f32, pos: Vec2) -> Ray {
        let direction = vec2(degree.to_radians().cos(), degree.to_radians().sin());

        Ray {
            degree,
            pos,
            end: Some(vec2(direction.x * RAY_LENGTH, direction.y * RAY_LENGTH)),
            dir: direction,
        }
    }

    pub fn draw(&self) {
        if let Some(end_point) = self.end {
            draw_line(
                self.pos.x,
                self.pos.y,
                self.pos.x + end_point.x,
                self.pos.y + end_point.y,
                1.0,
                WHITE,
            );
        } else {
            draw_line(
                self.pos.x,
                self.pos.y,
                self.pos.x + self.dir.x * RAY_LENGTH,
                self.pos.y + self.dir.y * RAY_LENGTH,
                1.0,
                WHITE,
            );
        }
    }

    pub fn draw_column(&self, line_count: usize, player_angle: f32) {
        if let Some(end_point) = self.end {
            let mut angle = self.degree - player_angle;
            // info!("angle = {}", angle);
            // info!("angle rad = {}", angle.to_radians());
            // info!("wid = {}", COLUMN_WIDTH);

            let x1 = self.pos.x;
            let y1 = self.pos.y;
            let x2 = self.pos.x + end_point.x;
            let y2 = self.pos.y + end_point.y;

            let real_distance =
                ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt() * angle.to_radians().cos();

            let distance = (RAY_LENGTH - real_distance) / RAY_LENGTH;
            let column_height = distance * screen_height();

            if column_height > 0. {
                let color = Color::from_rgba(
                    (distance * 255.0) as u8,
                    (distance * 255.0) as u8,
                    (distance * 255.0) as u8,
                    255,
                );

                draw_rectangle(
                    line_count as f32 * COLUMN_WIDTH,
                    240. - column_height / 2.,
                    COLUMN_WIDTH,
                    column_height,
                    color,
                );
            }
        }
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
            self.end = Some(point - self.pos);
        } else {
            self.end = None;
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

        draw_circle(self.pos.x, self.pos.y, 5.0, RED);
        draw_line(
            self.pos.x,
            self.pos.y,
            self.pos.x + direction.x * 10.,
            self.pos.y + direction.y * 10.,
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
}

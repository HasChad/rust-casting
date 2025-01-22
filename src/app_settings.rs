use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "RayCaster2D".into(),
        icon: None,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

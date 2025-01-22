use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "RayCaster2D".into(),
        icon: None,
        window_width: 640,
        window_height: 480,
        ..Default::default()
    }
}

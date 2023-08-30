use opencv::{
    prelude::*,
    core::Rect,
};

pub struct frame {
    raw: Mat
}
impl frame {
    pub fn to_matrix(&self) -> Mat {
        self.raw
    }
}

pub struct detect_ops {
    scale_factor: f64,
    min_neighbors: i32,
    min_width: i32,
    min_height: i32,
    max_width: i32,
    max_height: i32
}

pub struct area {
    x: i64,
    y: i64,
    width: i64,
    height: i64,
}
impl area {
    pub fn to_rect(&self) -> Rect{
        opencv::opencv::hub::core::cv_rect<i32> {

        }
    }
}
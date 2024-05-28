use chrono::{DateTime, Local};

pub struct Measurement {
    pub depth: u32,
    pub temp: f32,
    pub light: f32,
    pub salinity: f32,
    pub time: DateTime<Local>,
}

#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let i:f64 = speed as f64;
    return if i >= 0.0 && i <= 4.0 {
        i * 221.0
    } else if i >= 5.0 && i <= 8.0 {
        i * 221.0 * 0.9
    } else {
        i * 221.0 * 0.77
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let per_hour:f64 = production_rate_per_hour(speed);
    (per_hour/60.0) as u32
}


fn main() {
    println!("{}", working_items_per_minute(6));
}

#![allow(unused)]

use std::cmp::max;

pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    max(0, 40 - actual_minutes_in_oven)
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    2 * number_of_layers
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
   actual_minutes_in_oven + preparation_time_in_minutes(number_of_layers)
}


fn main() {
    println!("{}",elapsed_time_in_minutes(3, 30));
}

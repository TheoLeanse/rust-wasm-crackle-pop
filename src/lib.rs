extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn crackle_pop(limit: i32) {
    for number in 1..limit+1 {
        println!("{}", get_result(number));
    };
}

#[wasm_bindgen]
pub fn get_result (x: i32) -> String {
    if is_divisible_by(3, x) && is_divisible_by(5, x) {
        return String::from("CracklePop");
    } else  if is_divisible_by(3, x) {
        return String::from("Crackle");
    } else if is_divisible_by(5, x) {
        return String::from("Pop");
    } else {
        return x.to_string();
    };
}

fn is_divisible_by(x: i32, y: i32) -> bool {
    return y % x == 0;
}

fn main() {
    for number in 1..101 {
        println!("{}", crackle_pop(number));
    };
}

fn crackle_pop (x: i32) -> String {
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

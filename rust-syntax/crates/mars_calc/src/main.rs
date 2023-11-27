use std::io;

fn main() {
    let mut input = String::new();

    if let Ok(_) = io::stdin().read_line(&mut input) {
        borrow_string(&input);
        own_string(input);
    }
}

fn borrow_string(s: &String) {
    println!("{s}");
}

fn own_string(s: String) {
    println!("{s}");
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

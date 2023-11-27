use std::io;

fn main() {
    println!("Please Entry your weight (kg): ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    let weight = input
        .trim()
        .parse::<f32>()
        .expect("Failed pares input to number!");

    let mars_weight = calculate_weight_on_mars(weight);

    println!("The weight on mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

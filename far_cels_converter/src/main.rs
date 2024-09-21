use std::env;

fn main() {
    let temp: f32 = env::args()
        .nth(1)
        .expect("Provide temperature as first argument")
        .parse()
        .expect("Temperature must be a number");

    let unit = env::args()
        .nth(2)
        .expect("Provide unit (Celcius/Farenheit) as second argument");

    if unit.to_lowercase() == "celcius" {
        let farenheit_degrees = (9.0 * temp / 5.0) + 32.0;
        println!("{farenheit_degrees} degrees Farenheit");
    } else if unit.to_lowercase() == "farenheit" {
        let celcius_degrees = (5.0 * (temp - 32.0)) / 9.0;
        println!("{celcius_degrees} degrees Celcius");
    } else {
        println!("Invalid unit: {unit}");
    }
}

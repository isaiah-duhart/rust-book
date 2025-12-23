fn main() {
    println!("Result: {}", celsius_to_fahrenheit(0.0));
    println!("Result: {}", fahrenheit_to_celsius(32.0));
    println!("Result: {}", celsius_to_fahrenheit(100.0));
    println!("Result: {}", fahrenheit_to_celsius(70.0));
    println!("Result: {}", celsius_to_fahrenheit(10.0));
    println!("Result: {}", fahrenheit_to_celsius(50.0));
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    println!("Converting {c} celsius to fahrenheit");
    c * 1.8 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    println!("Converting {f} fahrenheit to celsius");
    (f - 32.0) / 1.8
}
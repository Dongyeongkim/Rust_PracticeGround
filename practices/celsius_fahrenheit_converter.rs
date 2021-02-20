fn main() {
    println!("{}F",celsius_to_fahrenheit(15.0));
    println!("{}C", fahrenheit_to_celsius(40.0));
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return celsius*1.8 + 32.0;
}

fn fahrenheit_to_celsius(fahrenhit: f64) -> f64 {
    return 1.8*(fahrenhit - 32.0);
}

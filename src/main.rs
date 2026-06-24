fn main() {
   //Convert temperatures between Fahrenheit and Celsius.
    // tC =  5/9 * (tF - 32)
    // tF = 9/5 * tC + 32
    println!("fahrenheit_to_celsius (98): {}\n
    celsius_to_fahrenheit (18.2):{}",
    (fahrenheit_to_celsius(98.0) * 10.0).round() / 10.0,
    (celsius_to_fahrenheit(18.2) * 10.0).round() / 10.0);

}
// tC =  5/9 * (tF - 32)
fn fahrenheit_to_celsius(fahrenheit:f64)->f64{
    5.0/9.0 * (fahrenheit - 32.0)

}
// tF = 9/5 * tC + 32
fn celsius_to_fahrenheit(celsius:f64)->f64{
    9.0/5.0 * celsius + 32.0
}

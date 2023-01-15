use std::io;

fn main() {
    let celcius: f64 = 25.0;

    let fahrenheit_result = to_fahrenheit(celcius);

    println!("{celcius} degrees celcius is {fahrenheit_result} degrees in fahrenheit.");

    let celcius_result = to_celcius(fahrenheit_result);

    println!("{fahrenheit_result} degrees fahrenheit is {celcius_result} degrees in celcius");

    let mut n = String::new();

    println!("Which fibonacci number do you want?");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i64 = n.trim().parse().expect("Please typ a new number");

    let fibonacci_number = generate_fibonacci_number(n);

    println!("The {n}-th fibonacci_number is: {fibonacci_number}");
}

fn to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn to_celcius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn generate_fibonacci_number(n: i64) -> i64 {
    if n <= 1 {
        n
    } else {
        generate_fibonacci_number(n-2) + generate_fibonacci_number(n-1)
    }

}
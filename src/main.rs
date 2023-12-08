use std::io;

fn main() {
    println!("**Temperature Converter**");
    println!("Enter '1' to convert Celsius to Fahrenheit");
    println!("Enter '2' to convert Fahrenheit to Celsius");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    match choice {
        1 => {
            println!("Enter temperature in Celsius:");
            let mut celsius_temperature = String::new();
            io::stdin()
                .read_line(&mut celsius_temperature)
                .expect("Failed to read line");
            let celsius_temperature: f64 = match celsius_temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    return;
                }
            };
            let fahrenheit_temperature: f64 = celsius_temperature * (9.0 / 5.0) + 32.0;
            println!("{:.2} Celsius is {:.2} Fahrenheit", celsius_temperature, fahrenheit_temperature);
        }
        2 => {
            println!("Enter temperature in Fahrenheit:");
            let mut fahrenheit_temperature = String::new();
            io::stdin()
                .read_line(&mut fahrenheit_temperature)
                .expect("Failed to read line");
            let fahrenheit_temperature: f64 = match fahrenheit_temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    return;
                }
            };
            let celsius_temperature = (fahrenheit_temperature - 32.0) * (5.0 / 9.0);
            println!("{:.2} Fahrenheit is {:.2} Celsius", fahrenheit_temperature, celsius_temperature);
        }
        _ => println!("Invalid choice"),
    }
}

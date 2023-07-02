use std::io;

fn main() {
    let temp: f64 = get_temperature();

    print_options();

    let action: u8 = get_action();

    match action {
        1 => println!("{}°C equals {:.2}°F.", temp, celsius_to_fahrenheit(temp)),
        2 => println!("{}°C equals {:.2}°K.", temp, celsius_to_kelvin(temp)),
        3 => println!("{}°F equals {:.2}°C.", temp, fahrenheit_to_celsius(temp)),
        4 => println!("{}°F equals {:.2}°K.", temp, fahrenheit_to_kelvin(temp)),
        5 => println!("{}°K equals {:.2}°C.", temp, kelvin_to_celsius(temp)),
        6 => println!("{}°K equals {:.2}°F.", temp, kelvin_to_fahrenheit(temp)),
        _ => println!("Invalid option!"),
    }
}

fn get_temperature() -> f64 {
    loop {
        println!("Please input the temperature.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match input.trim().parse() {
            Ok(temp) => break temp,
            Err(_) => continue,
        }
    }
}

fn get_action() -> u8 {
    loop {
        println!("Please select a valid action.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match input.trim().parse() {
            Ok(action) => if action > 6 || action < 1 {
                continue;
            } else {
                break action
            },
            Err(_) => continue,
        }
    }
}

fn print_options() {
    println!("ACTIONS:");
    println!("1 - convert from Celsius to Fahreneit");
    println!("2 - convert from Celsius to Kelvin");
    println!("3 - convert from Fahrenheit to Celsius");
    println!("4 - convert from Fahrenheit to Kelvin");
    println!("5 - convert from Kelvin to Celsius");
    println!("6 - convert from Kelvin to Fahrenheit");
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    (fahrenheit + 459.67) * 5.0 / 9.0
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    kelvin * 9.0 / 5.0 - 459.67
}

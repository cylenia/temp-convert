use std::io;

enum Degree {
    Celsius,
    Fahrenheit,
    Kelvin
}

fn convert(input: f32, origin: &Degree, target: &Degree) -> f32 {
    match (origin, target) {
        (Degree::Celsius, Degree::Fahrenheit) => input * 9.0 / 5.0 + 32.0,
        (Degree::Fahrenheit, Degree::Celsius) => (input - 32.0) * 5.0 / 9.0,
        (Degree::Celsius, Degree::Kelvin) => input + 273.15,
        (Degree::Kelvin, Degree::Celsius) => input - 273.15,
        (Degree::Fahrenheit, Degree::Kelvin) => (input - 32.0) * 5.0 / 9.0 + 273.15,
        (Degree::Kelvin, Degree::Fahrenheit) => (input - 273.15) * 9.0 / 5.0 + 32.0,
        (_, _) => input, // same unit, no conversion
    }
}

fn unit_pretty(input: &Degree) -> &'static str {
    match input {
        Degree::Celsius => "Celsius",
        Degree::Fahrenheit => "Fahrenheit",
        Degree::Kelvin => "Kelvin",
    }
}

fn main() {
    println!("Type what you would like to convert from, as \"f\", \"c\" or \"k\".");

    let origin = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "f" => break Degree::Fahrenheit,
            "c" => break Degree::Celsius,
            "k" => break Degree::Kelvin,
            _   => {
                println!("Incorrect. Use \"f\", \"c\" or \"k\".");
                continue;
            }, 
        };
    };
    
    println!("Type the number of degrees you would like to convert. (e.g. 6.7)");

    let degrees = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type a floating point number.");
                continue;
            },
        };

        break input;
    };

    println!("Type what you would like to convert to, as \"f\", \"c\" or \"k\".");

    let target = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "f" => break Degree::Fahrenheit,
            "c" => break Degree::Celsius,
            "k" => break Degree::Kelvin,
            _ => {
                println!("Incorrect. Use \"f\", \"c\" or \"k\".");
                continue;
            },
        };
    };

    println!("{degrees} {} in {} is {:.1}.",
        unit_pretty(&origin),
        unit_pretty(&target),
        convert(degrees, &origin, &target)
    );
}

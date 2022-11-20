use std::io;

fn main() {
    println!("entrer une température en celsius que vous voulez convertir en fahrenheit :");
    let mut temperature_en_celsius: String = String::new();

    io::stdin()
        .read_line( &mut temperature_en_celsius)
        .expect("le programme n'as pas pu lire l'entrée");
    
    let temperature_en_celsius: f32 = temperature_en_celsius.trim().parse::<f32>().unwrap();
    let temperature_en_fahrenheit: f32 = convertir_celsius_en_fahrenheit(temperature_en_celsius);

    println!("la température {temperature_en_celsius}°C est égale à {temperature_en_fahrenheit}°F");

    println!("entrer une température en fahrenheit que vous voulez convertir en celsius :");
    let mut temperature_en_fahrenheit: String = String::new();

    io::stdin()
        .read_line(&mut temperature_en_fahrenheit)
        .expect("le programme n'as pas pu lire l'entrée");
    
    let temperature_en_fahrenheit: f32 = temperature_en_fahrenheit.trim().parse::<f32>().unwrap();
    let temperature_en_celsius: f32 = convertir_fahrenheit_en_celsius(temperature_en_fahrenheit);
    
    println!("la température {temperature_en_fahrenheit}°F est égale à {temperature_en_celsius}°C");
}

fn convertir_celsius_en_fahrenheit(temperature_en_celsius: f32) -> f32 {
    let temperature_en_fahrenheit: f32 = temperature_en_celsius*9.0/5.0+32.0;
    return temperature_en_fahrenheit;
}

fn convertir_fahrenheit_en_celsius(temperature_en_fahrenheit: f32) -> f32 {
    let temperature_en_celsius: f32 = (temperature_en_fahrenheit-32.0)*5.0/9.0;
    return temperature_en_celsius;
}
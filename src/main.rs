use std::io;

fn main() {
    { 
        println!("entrer une température (nombre décimal) en celsius que vous voulez convertir en fahrenheit :");
        let mut temperature_celsius_vers_fahrenheit: String = String::new();

        io::stdin()
            .read_line(&mut temperature_celsius_vers_fahrenheit)
            .expect("Le programme n'as pas pu lire l'entrée !");

        let mut temperature_celsius_vers_fahrenheit: f32 = temperature_celsius_vers_fahrenheit
            .trim()
            .parse::<f32>()
            .expect("Merci de taper un nombre décimal !");

        print!("la température {temperature_celsius_vers_fahrenheit}°c vaut ");
        celsius_vers_fahrenheit(&mut temperature_celsius_vers_fahrenheit);
        println!("{temperature_celsius_vers_fahrenheit}°f !");
    }

    {
        println!("entrer une température (nombre décimal) en fahrenheit que vous voulez convertir en celsius :");
        let mut temperature_fahrenheit_vers_celsius: String = String::new();

        io::stdin()
            .read_line(&mut temperature_fahrenheit_vers_celsius)
            .expect("Le programme n'as pas pu lire l'entrée !");

        let mut temperature_fahrenheit_vers_celsius: f32 = temperature_fahrenheit_vers_celsius
            .trim()
            .parse::<f32>()
            .expect("Merci de taper un nombre décimal !");
        
        print!("la température {temperature_fahrenheit_vers_celsius}°f vaut ");
        fahrenheit_vers_celsius(&mut temperature_fahrenheit_vers_celsius);
        println!("{temperature_fahrenheit_vers_celsius}°c !")
    }
}

fn celsius_vers_fahrenheit(temperature_celsius_vers_fahrenheit: &mut f32) -> () {
    *temperature_celsius_vers_fahrenheit = (*temperature_celsius_vers_fahrenheit - 32.0) / 1.8;
}

fn fahrenheit_vers_celsius(temperature_fahrenheit_vers_celsius: &mut f32) -> () {
    *temperature_fahrenheit_vers_celsius = (*temperature_fahrenheit_vers_celsius * 1.8) + 32.0;
}
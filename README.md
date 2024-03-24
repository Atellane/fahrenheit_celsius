# convertir les Celsius en Fahrenheit et les Fahrenheit en Celsius
Ce projet est un petit exercice réalisé afin de mettre en pratique mes connaissances fraichement acquises en **Rust**.

Pour ce projet, je vais demander des valeurs en degré Celsius ou Fahrenheit que mon programme convertiras ensuite je dois donc commencer par importer la crate 
*input/output* de la *bibliothèque standard*, `std::io` :
```Rust
use std::io;

fn main() {
    
    // --code coupé--
}
```
`std` correspond à la *biliothèque standard* et `io` à la crate *input/output*.

Je placerais chaque partie du projet dans une portée dédiée. Une portée est délimitée par `{}` comme ci-dessous :
```Rust
use std::io;

fn main() {
    {
        // --code Celsius -> Fahrenheit--
    }

    {
        // --code Fahrenheit -> Celsius--
    }
}
```
## Celsius -> Fahrenheit
### Créations d'une fonction pour convertir les Celsius en Fahrenheit
Commencons par créer une fonctions qui convertit les Celsius en Fahrenheit :
```Rust
fn celsius_vers_fahrenheit(temperature_celsius_vers_fahrenheit: &mut f32) -> () {
    *temperature_celsius_vers_fahrenheit = (*temperature_celsius_vers_fahrenheit - 32.0) / 1.8;
}
```
Notre fonction `celsius_vers_fahrenheit` prend en paramètre `temperature_celsius_vers_fahrenheit` qui correspond à la valeur de type `&mut f32` c'est à dire une 
référence à une variable mutable aillant une valeur de type `f32` qui est la température en celsius qu'on renseigne à l'utilisation de la fonction. On modifie la 
valeur de la variable vers laquelle pointe la référence grace à l'opérateur de déréfrencement `*` qui permet de modifier la valeur de la variable, on applique donc 
la formule de conversion. Pas besoin de retourner quoi que ce soit, c'est la variable dont la référence à été donnée en paramètre qui a été directement modififée.
### Implémentations de la fonction préalablement crée à notre programme
Maintenant que nous avons créer notre fonctions, nous allons maintenant l'intégrer à notre programme dans la fonction main, commençons 
par demander une valeur en degré Celsius que le programme doit convertir en degré Fahrenheit :
```Rust
use std::io;

fn main() {
   { 
        println!("entrer une température (nombre décimal) en celsius que vous voulez convertir en fahrenheit :");
        let mut temperature_celsius_vers_fahrenheit: String = String::new();

        io::stdin()
            .read_line(&mut temperature_celsius_vers_fahrenheit)
            .expect("Le programme n'as pas pu lire l'entrée !");
        
        // --code coupé--
   }
    
    // --code coupé--
}
```
Le code ci-dessus affiche d'abord un message avec `println!` pour prévenir l'utilisateur qu'une valeur lui ai demandée. Ensuite on crée une variable mutable 
`temperature_celsius_vers_fahrenheit` de type `String` à laquelle nous assignons pour valeur une nouvelle chaine de caractère. `io::` va chercher une fonction dans 
la crate `io` qu'on a importé.e, la fonction qu'on utilise pour demander la valeur à l'utilistaeur est `stdin()` (= standard input), `.read_line` lit la valeur qui 
a été entrée et la place dans la variable `temperature_celsius_vers_fahrenheit`. Enfin `.expect` se charge d'afficher un message d'erreur si `read_line` n'a pas
fonctionné.

Maintenant que nous savons quelle valeur notre utilisateur veut rentre, il faut convertir cette valeur `String` en `f32` pour pouvoir être utilisée en paramètre de 
notre fonction.
```Rust
use std::io;
fn main() {
    {
        // --code coupé--
        
        let mut temperature_celsius_vers_fahrenheit: f32 = temperature_celsius_vers_fahrenheit
            .trim()
            .parse::<f32>()
            .expect("Merci de taper un nombre décimal !");

        print!("la température {temperature_celsius_vers_fahrenheit}°c vaut ");
        celsius_vers_fahrenheit(&mut temperature_celsius_vers_fahrenheit);
        println!("{temperature_celsius_vers_fahrenheit}°f !");

        // --code coupé--
    }
    
    // --code coupé--
}
```
Tout d'abord, on utilise l'ombrage de **Rust** pour créer une nouvelle variable `temperature_en_celsius` de type `f32` qui [^1]*ombragera* la première, sa valeur 
est le résultat de la conversion de l'ancienne variable du même nom, en `f32`. `.trim()` s'occupe de retirer les espaces au début et à la fin de la chaine de 
caractère d'origine, ensuite `.parse::<f32>()` convertit la chaine de caractère en `f32` et `.expect` affiche un message d'erreur si `parse` n'as pas fonctionné. 
Ensuite on affiche la valeur de la variable `temperature_celsius_vers_fahrenheit` qui est la valeur en degré celius à l'aide de `print!` qui permet d'afficher 
quelque chose sans passer à la ligne à la fin. Après, je convertie `temperature_celsius_vers_fahrenheit` en fahrenheit grâce à ma fonction, je n'ai pas besoin de 
faire `temperature_celsius_vers_fahrenheit = celsius_vers_fahrenheit(&mut temperature_celsius_vers_fahrenheit)` car la fonction `celsius_vers_fahrenheit()` ne 
retourne rien, elle modifie directement la variable dont la référence lui ai passée en paramètre. Enfin j'affiche la température en fahrenheit avec `println!()` 
pour que cette fois un `\n` sois également afficher par défaut afin de faciliter l'affichage de la suite de notre programme.
### résultat
```Rust
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

    // --code coupé--
}
```
## Fahrenheit -> Celsius
### Créations d'une fonction pour convertir les Fahrenheit en Celsius
Commencons par créer une fonctions qui convertit les Fahrenheit en Celsius :
```Rust
fn fahrenheit_vers_celsius(temperature_fahrenheit_vers_celsius: &mut f32) -> () {
    *temperature_fahrenheit_vers_celsius = (*temperature_fahrenheit_vers_celsius * 1.8) + 32.0;
}
```
Notre fonction `convertir_fahrenheit_en_celsius` prend en paramètre `temperature_fahrenheit_vers_celsius` qui correspond à la valeur de type `&mut f32` c'est à dire 
une référence à une variable mutable aillant une valeur de type `f32` qui est la température en celsius qu'on renseigne à l'utilisation de la fonction. On modifie la 
valeur de la variable vers laquelle pointe la référence grace à l'opérateur de déréfrencement `*` qui permet de modifier la valeur de la variable, on applique donc 
la formule de conversion. Pas besoin de retourner quoi que ce soit, c'est la variable dont la référence à été donnée en paramètre qui a été directement modififée.
### Implémentations de la fonction préalablement crée à notre programme
Maintenant que nous avons créer notre fonctions, nous allons maintenant l'intégrer à notre programme dans la fonction main, commençons 
par demander une valeur en degré Fahrenheit que le programme doit convertir en degré Celsius :
```Rust
fn main() {
    {
        println!("entrer une température (nombre décimal) en fahrenheit que vous voulez convertir en celsius :");
        let mut temperature_fahrenheit_vers_celsius: String = String::new();

        io::stdin()
            .read_line(&mut temperature_fahrenheit_vers_celsius)
            .expect("Le programme n'as pas pu lire l'entrée !");

            // --code coupé--
    }
    
    // --code coupé--
}
```
Le code ci-dessus affiche d'abord un message avec `println!` pour prévenir l'utilisateur qu'une valeur lui ai demandée. Ensuite on crée une variable mutable 
`temperature_fahrenheit_vers_celsius` de type `String` à laquelle nous assignons pour valeur une nouvelle chaine de caractère. `io::` va chercher une fonction dans 
la crate `io` qu'on a importé.e, la fonction qu'on utilise pour demander la valeur à l'utilistaeur est `stdin()` (= standard input), `.read_line` lit la valeur qui 
a été entrée et la place dans la variable `temperature_fahrenheit_vers_celsius`. Enfin `.expect` se charge d'afficher un message d'erreur si `read_line` n'a pas 
fonctionné.

Maintenant que nous savons quelle valeur notre utilisateur veut rentre, il faut convertir cette valeur `String` en `f32` pour pouvoir 
être utilisée en paramètre de notre fonction.
```Rust
fn main() {
    // --code coupé--

    {
        // --code coupé--

        let mut temperature_fahrenheit_vers_celsius: f32 = temperature_fahrenheit_vers_celsius
            .trim()
            .parse::<f32>()
            .expect("Merci de taper un nombre décimal !");
        
        print!("la température {temperature_fahrenheit_vers_celsius}°f vaut ");
        fahrenheit_vers_celsius(&mut temperature_fahrenheit_vers_celsius);
        println!("{temperature_fahrenheit_vers_celsius}°c !")
    }
}
```
Tout d'abord, on utilise l'ombrage de **Rust** pour créer une nouvelle variable immutable `temperature_fahrenheit_vers_celsius` de type `f32` qui [^1]*ombragera* la 
première, sa valeur est le résultat de la conversion de l'ancienne variable du même nom, en `f32`. `.trim()` s'occupe de retirer les espaces au début et à la fin de 
la chaine de caractère d'origine, ensuite `.parse::<f32>()` convertit la chaine de caractère en `f32` et `.expect` affiche un message d'erreur si `parse` n'as pas 
fonctionné. Ensuite on affiche la valeur de la variable `temperature_fahrenheit_vers_celsius` qui est la valeur en degré fahrenheit à l'aide de `print!` qui permet 
d'afficher quelque chose sans passer à la ligne à la fin. Après, je convertie `temperature_fahrenheit_vers_celsius` en celsius grâce à ma fonction, je n'ai pas 
besoin de faire `temperature_celsius_vers_fahrenheit = celsius_vers_fahrenheit(&mut temperature_celsius_vers_fahrenheit)` car la fonction `celsius_vers_fahrenheit()`
ne retourne rien, elle modifie directement la variable dont la référence lui ai passée en paramètre. Enfin j'affiche la température en fahrenheit avec `println!()`
pour que cette fois un `\n` sois également afficher par défaut afin de faciliter l'affichage de la suite de notre programme.
### résultat
```Rust
fn main() {
    // --code coupé--

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
```

[^1]: une variable qui ombrage une variable porte le même nom qu'une ancienne variable et la remplacera dans la portée interne

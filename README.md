# convertir les Celsius en Fahrenheit et les Fahrenheit en Celsius
Ce projet est un petit exercice réalisé afin de mettre en pratique mes connaissances fraichement acquises en **Rust**.

Pour ce projet, je vais demander des valeurs en degré Celsius ou Fahrenheit que mon programme convertiras ensuite je dois donc commencer
par importer la crate *input/output* de la *bibliothèque standard*, `std::io` :
```Rust
use std::io;

fn main() {
    // --code coupé--
}
```
`std` correspond à la *biliothèque standard* et `io` à la crate *input/output*
## Celsius -> Fahrenheit
### Créations d'une fonction pour convertir les Celsius en Fahrenheit
Commencons par créer une fonctions qui convertit les Celsius en Fahrenheit :
```Rust
fn convertir_celsius_en_fahrenheit(temperature_en_celsius: f32) -> f32 {
    let temperature_en_fahrenheit: f32 = temperature_en_celsius*9.0/5.0+32.0;
    return temperature_en_fahrenheit;
}
```
Notre fonction `convertir_celsius_en_fahrenheit` prend en paramètre `temperature_en_celsius` qui correspond à la valeur de type `f32` 
qu'on renseigne à l'utilisation de la fonction. on initialise une variable immuable `temperature_en_fahrenheit` qui prend pour valeur 
le résultat de la formule de conversion des Celsius en Fahrenheit. La fonction retourne une valeur `f32` correspondant à la valeur 
contenue dans `temperature_en_fahrenheit`.
### Implémentations de la fonction préalablement crée à notre programme
Maintenant que nous avons créer notre fonctions, nous allons maintenant l'intégrer à notre programme dans la fonction main, commençons 
par demander une valeur en degré Celsius que le programme doit convertir en degré Fahrenheit :
```Rust
use std::io;

fn main() {
    println!("entrer une température en celsius que vous voulez convertir en fahrenheit :");
    let mut temperature_en_celsius: String = String::new();

    io::stdin()
        .read_line( &mut temperature_en_celsius)
        .expect("le programme n'as pas pu lire l'entrée");
    
    // --code coupé--
}
```
Le code ci-dessus affiche d'abord un message avec `println!` pour prévenir l'utilisateur qu'une valeur lui ai demandée. Ensuite on crée 
une variable muable `temperature_en_celsius` de type `String` à laquelle nous assignons pour valeur une nouvelle chaine de caractère. 
`io::` va chercher une fonction dans law crate `io` qu'on a importé.e, la fonction qu'on utilise pour demander la valeur à 
l'utilistaeur est `stdin()` (= standard input), `.read_line` lit la valeur qui a été entrée et la place dans la variable 
`temperature_en_celsius`. Enfin `.expect` se charge d'afficher un message d'erreur si `read_line` n'a pas fonctionné.

Maintenant que nous savons quelle valeur notre utilisateur veut rentre, il faut convertir cette valeur `String` en `f32` pour pouvoir 
être utilisée en paramètre de notre fonction.
```Rust
use std::io;
fn main() {
    // --code coupé--
    
    let temperature_en_celsius: f32 = temperature_en_celsius.trim().parse::<f32>().unwrap();
    let temperature_en_fahrenheit: f32 = convertir_celsius_en_fahrenheit(temperature_en_celsius);

    println!("la température {temperature_en_celsius}°C est égale à {temperature_en_fahrenheit}°F");
    
    // --code coupé--
}
```
Tout d'abord, on utilise l'ombrage de **Rust** pour créer une nouvelle variable immuable `temperature_en_celsius` de type `f32` qui 
[^1]*ombragera* la première, sa valeur est le résultat de la conversion de l'ancienne variable du même nom, en `f32`. `.trim()` 
s'occupe de retirer les espaces au début et à la fin de la chaine de caractère d'origine, ensuite `.parse::<f32>()` convertit la chaine 
de caractère en `f32` et `.unwrap()` vérifie que ça a bien fonctionné. Enfin, on crée une variable immuable `temperature_en_fahrenheit` de type `f32` qui prend pour valeur le résultat de la conversion de la valeur de `temperature_en_celsius` en Fahrenheit, puis le réultat est renvoyée à l'utilisateur avec un petit message à l'aide de `println!`.
### résultat
```Rust
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

    // --code coupé--
}
```
## Fahrenheit -> Celsius
### Créations d'une fonction pour convertir les Fahrenheit en Celsius
Commencons par créer une fonctions qui convertit les Fahrenheit en Celsius :
```Rust
fn convertir_fahrenheit_en_celsius(temperature_en_fahrenheit: f32) -> f32 {
    let temperature_en_celsius: f32 = (temperature_en_fahrenheit-32.0)*5.0/9.0;
    return temperature_en_celsius;
}
```
Notre fonction `convertir_fahrenheit_en_celsius` prend en paramètre `temperature_en_fahrenheit` qui correspond à la valeur de type 
`f32` qu'on renseigne à l'utilisation de la fonction. on initialise une variable immuable `temperature_en_celsius` qui prend pour 
valeur le résultat de la formule de conversion des Fahrenheit en Celsius. La fonction retourne une valeur `f32` correspondant à la 
valeur contenue dans `temperature_en_celsius`.
### Implémentations de la fonction préalablement crée à notre programme
Maintenant que nous avons créer notre fonctions, nous allons maintenant l'intégrer à notre programme dans la fonction main, commençons 
par demander une valeur en degré Fahrenheit que le programme doit convertir en degré Celsius :
```Rust
fn main() {
    // --code coupé--
    
    println!("entrer une température en fahrenheit que vous voulez convertir en celsius :");
    let mut temperature_en_fahrenheit: String = String::new();

    io::stdin()
        .read_line(&mut temperature_en_fahrenheit)
        .expect("le programme n'as pas pu lire l'entrée");
    
    // --code coupé--
}
```
Le code ci-dessus affiche d'abord un message avec `println!` pour prévenir l'utilisateur qu'une valeur lui ai demandée. Ensuite on crée 
une variable muable `temperature_en_fahrenheit` de type `String` à laquelle nous assignons pour valeur une nouvelle chaine de 
caractère. `io::` va chercher une fonction dans law crate `io` qu'on a importé.e, la fonction qu'on utilise pour demander la valeur à 
l'utilistaeur est `stdin()` (= standard input), `.read_line` lit la valeur qui a été entrée et la place dans la variable 
`temperature_en_fahrenheit`. Enfin `.expect` se charge d'afficher un message d'erreur si `read_line` n'a pas fonctionné.

Maintenant que nous savons quelle valeur notre utilisateur veut rentre, il faut convertir cette valeur `String` en `f32` pour pouvoir 
être utilisée en paramètre de notre fonction.
```Rust
fn main() {
    // --code coupé--

    let temperature_en_fahrenheit: f32 = temperature_en_fahrenheit.trim().parse::<f32>().unwrap();
    let temperature_en_celsius: f32 = convertir_fahrenheit_en_celsius(temperature_en_fahrenheit);
    
    println!("la température {temperature_en_fahrenheit}°F est égale à {temperature_en_celsius}°C");
}
```
Tout d'abord, on utilise l'ombrage de **Rust** pour créer une nouvelle variable immuable `temperature_en_fahrenheit` de type `f32` qui 
[^1]*ombragera* la première, sa valeur est le résultat de la conversion de l'ancienne variable du même nom, en `f32`. `.trim()` 
s'occupe de retirer les espaces au début et à la fin de la chaine de caractère d'origine, ensuite `.parse::<f32>()` convertit la chaine 
de caractère en `f32` et `.unwrap()` vérifie que ça a bien fonctionné. Enfin, on crée une variable immuable `temperature_en_celsius` de 
type `f32` qui prend pour valeur le résultat de la conversion de la valeur de `temperature_en_fahrenheit` en Celsius, puis le réultat 
est renvoyée à l'utilisateur avec un petit message à l'aide de `println!`.
### résultat
```Rust
fn main() {
    // --code coupé--

    println!("entrer une température en fahrenheit que vous voulez convertir en celsius :");
    let mut temperature_en_fahrenheit: String = String::new();

    io::stdin()
        .read_line(&mut temperature_en_fahrenheit)
        .expect("le programme n'as pas pu lire l'entrée");
    
    let temperature_en_fahrenheit: f32 = temperature_en_fahrenheit.trim().parse::<f32>().unwrap();
    let temperature_en_celsius: f32 = convertir_fahrenheit_en_celsius(temperature_en_fahrenheit);
    
    println!("la température {temperature_en_fahrenheit}°F est égale à {temperature_en_celsius}°C");
}
```

[^1]: une variable qui ombrage une variable qui porte le même nom qu'une ancienne variable et qui remplacera l'ancienne dans la portée interne
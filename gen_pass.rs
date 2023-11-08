// Importamos las librerías necesarias
use rand::Rng;
use password_strength::StrengthMeter;

// Definimos una función que genera una contraseña aleatoria de una longitud dada
fn generar_password(longitud: usize) -> String {
    // Definimos los posibles caracteres que puede tener la contraseña
    let caracteres = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()";
    // Creamos una variable para almacenar la contraseña
    let mut password = String::new();
    // Creamos un generador de números aleatorios
    let mut rng = rand::thread_rng();
    // Iteramos la cantidad de veces que indica la longitud
    for _ in 0..longitud {
        // Elegimos un caracter aleatorio de la cadena de caracteres
        let c = rng.gen_range(0..caracteres.len());
        // Lo añadimos a la contraseña
        password.push(caracteres.chars().nth(c).unwrap());
    }
    // Devolvemos la contraseña
    password
}

// Definimos una función que evalúa la seguridad de una contraseña
fn evaluar_password(password: &str) -> u8 {
    // Creamos un medidor de seguridad
    let meter = StrengthMeter::new();
    // Obtenemos la puntuación de la contraseña, que va de 0 a 4
    let puntuacion = meter.check(password).score();
    // Devolvemos la puntuación
    puntuacion
}

// Probamos el código
fn main() {
    // Generamos una contraseña de 10 caracteres
    let password = generar_password(10);
    // Evaluamos su seguridad
    let seguridad = evaluar_password(&password);
    // Imprimimos la contraseña y su nivel de seguridad
    println!("La contraseña generada es: {}", password);
    println!("El nivel de seguridad es: {}", seguridad);
}
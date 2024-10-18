use std::fs::File;
use std::io::{ Write, BufWriter };
use std::io::stdin;
use dirs::desktop_dir; // Para obtener la ruta del escritorio

fn main() {
    let mut num_personas = String::new();

    // Solicitamos al usuario la cantidad de personas
    println!("¿Cuántas personas quieres ingresar?");
    stdin().read_line(&mut num_personas).expect("Error al leer la cantidad de personas");

    // Convertimos el número ingresado a un entero
    let num_personas: usize = match num_personas.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Por favor ingresa un número válido.");
            return;
        }
    };

    // Creamos un vector para almacenar los nombres y edades
    let mut personas: Vec<(String, u32)> = Vec::new();

    // Recolectamos el nombre y la edad de cada persona
    for i in 1..=num_personas {
        let mut nombre = String::new();
        let mut edad = String::new();

        println!("Ingresa el nombre de la persona {}:", i);
        stdin().read_line(&mut nombre).expect("Error al leer el nombre");

        println!("Ingresa la edad de la persona {}:", i);
        stdin().read_line(&mut edad).expect("Error al leer la edad");

        // Convertimos la edad a un número entero
        let edad: u32 = match edad.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Por favor ingresa una edad válida.");
                return;
            }
        };

        // Almacenamos el nombre y la edad en el vector
        personas.push((nombre.trim().to_string(), edad));
    }

    // Obtenemos la ruta del escritorio
    if let Some(escritorio_path) = desktop_dir() {
        let archivo_path = escritorio_path.join("personas.txt");

        // Guardamos la información en un archivo en el escritorio
        let file = File::create(&archivo_path).expect(
            "No se pudo crear el archivo en el escritorio"
        );
        let mut writer = BufWriter::new(file);

        for (nombre, edad) in personas {
            writeln!(writer, "Nombre: {}, Edad: {}", nombre, edad).expect(
                "No se pudo escribir en el archivo"
            );
        }

        println!("La información ha sido guardada en '{}'.", archivo_path.display());
    } else {
        println!("No se pudo encontrar el escritorio.");
    }
}

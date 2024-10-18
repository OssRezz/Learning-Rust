# Learning Rust

- Lenguaje de programacion compilado, de bajo nivel y codigo abierto

- Lenguaje de programacion de sistema, significa que no esta pensado para hacer aplicaciones de interfaz grafica. Su enfoque esta en drivers, sistemas operativos... etc.

- El creador de Rust es Mozilla.

- Seguro, practico, pensado en la concurrencia y confiable.

- Usado por Deno, DropBox, NPM, GitHub, Microsoft, Amazon, Next.js

- ¿Para que sive Rust? Aplicaciones en la terminal, WebAssembly, Networking

## Índice

- [Instalación](#instalación)
- [Ejecutar Rust](#ejecutar-rust)
- [Crear un proyecto](#crear-un-proyecto)
- [Funciones](#funciones)
- [Macros](#macros)
- [Variables](#variables)
- [Tipos de datos](#tipos-de-datos)
- [Tuplas y estructuras](#tuplas-y-estructuras)

## Instalación

- Debemos tener instalado Visual studio stools de c ++

- Descargamos desde <https://rustup.rs> el .exe en Windows

- Seguimos los pasos de instalación

- Una vez finalizado abrimos una terminal nuevo y escribimos rustc --version, si nos aparece `rustc 1.82.0 (f6e511eec 2024-10-15)`, Significa que fue instalado correctamente.

## Ejecutar Rust

- `rustc nombre_fichero.rs`, este comando nos ayuda a compilar la aplicacion

- Luego de compilado nos crea un archivo .exe, para ejcutarlo nos vemos donde se encuentra el archivo y escrimos en consola `.\nombre_archivo.exe`

## Crear un proyecto

- Se usará Cargo para escribir y ejecutar el mismo programa.

- En la terminal escribimos `cargo new nombre_proyecto`, Este comando genera un nuevo directorio denominado nombre_proyecto con un subdirectorio src y agrega dos archivos:

```text
hello-cargo/
     Cargo.toml
     src/
         main.rs
```

- El archivo Cargo.toml es el archivo de manifiesto de Rust. Es donde se conservan los metadatos para el proyecto, así como las dependencias.

- El archivo main.rs en el subdirectorio src es donde escribirá el código de la aplicación.

### Compilación y ejecución del programa con Cargo

- Para ejecutar el programa reutilizable, nos movemos al nuevo directorio y ejecutamos el comando `cargo run` en la terminal. Debería aparecer la salida siguiente en el terminal:

```rust
  Compiling hello-cargo v0.1.0 (/tmp/.OFUp/hello-cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.59s
      Running `target/debug/hello-cargo`

Hello, world!
```

## Funciones

- Para declarar una función en Rust, usamos la palabra clave fn. Después del nombre de la función, se le indica al compilador cuántos parámetros o argumentos espera la función como entrada. Los argumentos se enumeran entre paréntesis ().

```rust
fn main() {

}
```

- Cada programa de Rust debe tener una función llamada main. El código de la función main siempre es el primer código que se ejecuta en un programa con Rust. Podemos llamar a otras funciones desde la función main o desde otras funciones.

```rust
fn main() {
    println!("Hello, world!");
}
```

## Identación

- **En el cuerpo de la función, la mayoría de las instrucciones de código terminan con un punto y coma ;. Rust procesa estas instrucciones una tras otra, por orden. Cuando una instrucción de código no termina con un punto y coma, Rust sabe que la línea de código siguiente debe ejecutarse antes de que se pueda completar la instrucción inicial.**

- Para ayudar a ver las relaciones de ejecución en el código, usamos la sangría. Este formato muestra cómo se organiza el código y revela el flujo de pasos necesarios para completar la tarea de la función. A una instrucción de código inicial se le aplica una sangría de cuatro espacios desde el margen izquierdo. Cuando el código no termina con un punto y coma, a la siguiente línea de código que se va a ejecutar se le aplica una sangría de cuatro espacios más.

```rust
fn main() { // The function declaration is not indented

    // First step in function body
        // Substep: execute before First step can be complete

    // Second step in function body
        // Substep A: execute before Second step can be complete
        // Substep B: execute before Second step can be complete
            // Sub-substep 1: execute before Substep B can be complete

    // Third step in function body, and so on...
}
```

## Macros

- En Rust, una macro es como una función y toma un número variable de argumentos de entrada.

### todo!

- La macro todo! se usa para identificar código sin terminar en el programa de Rust. La macro es útil para crear prototipos, o bien cuando se quiere indicar un comportamiento que no está completo.

```rust
fn main() {
    // Display the message "Hello, world!"
    todo!("Display the message by using the println!() macro");
}
```

- Al compilar código en el que se usa la macro todo!, el compilador puede devolver un mensaje de alarma en el que espera encontrar la funcionalidad completada:

```bash
Compiling playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/playground`
thread 'main' panicked at 'not yet implemented: Display the message by using the println!() macro', src/main.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### println!

- La macro println! espera uno o varios argumentos de entrada, que se muestran en la pantalla o en la salida estándar.

```rust
fn main() {
    // Our main function does one task: show a message
    // println! displays the input "Hello, world!" to the screen
    println!("Hello, world!");
}
```

#### Sustitución de valores para argumentos {}

- La macro println! reemplaza cada instancia de llaves {} dentro de una cadena de texto por el valor del argumento siguiente de la lista.

```rust
fn main() {
    // Call println! with three arguments: a string, a value, a value
    println!("The first letter of the English alphabet is {} and the last letter is {}.", 'A', 'Z');
}
```

- Llamamos a la macro println! con tres argumentos: una cadena, un valor y otro valor. La macro procesa los argumentos por orden. Cada instancia de llaves {} dentro de una cadena de texto se reemplaza por el valor del argumento siguiente de la lista. La salida es la siguiente:

```bash
The first letter of the English alphabet is A and the last letter is Z.
```

## Variables

- En Rust, una variable se declara con la palabra clave let. Cada variable tiene un nombre único. Cuando se declara una variable, se puede enlazar a un valor o el valor se puede enlazar más adelante en el programa.

```rust
//Sin valor
let a_number;

// Con valor
let a_number = 10;
```

- El código siguiente declara dos variables. La primera variable se declara, pero no se enlaza a un valor. La segunda variable se declara y enlaza a un valor. Más adelante en el programa, la primera variable se enlaza a un valor.

```rust

// Declare a variable
let a_number;

// Declare a second variable and bind the value
let a_word = "Ten";

// Bind a value to the first variable
a_number = 10;

println!("The number is {}.", a_number);
println!("The word is {}.", a_word);
```

### Inmutable frente a mutable (mut)

- En Rust, los enlaces de variables son inmutables de manera predeterminada. Cuando una variable es inmutable, después de enlazar un valor a un nombre, no se puede cambiar ese valor.

- Para mutar un valor, debemos usar en primer lugar la palabra clave mut para convertir en mutable el enlace de una variable.

```rust
// The `mut` keyword lets the variable be changed
let mut a_number = 10;
println!("The number is {}.", a_number);

// Change the value of an immutable variable
a_number = 15;
println!("Now the number is {}.", a_number);
```

- Rust siempre nos generara una alerta cada vez que declaremos una variable con un valor y la reasignemos con un valor diferente sin ser utilizada, esto no sucedera si usamos la variable y luego la cambiamos de valor.

```text
value assigned to `a_number` is never read
maybe it is overwritten before being read?
```

### Propiedad reemplazada de variables (Shadowing)

- Puede declarar una variable nueva que use el nombre de una existente. La declaración nueva crea un enlace. En Rust, esta operación se denomina "propiedad reemplazada" porque la nueva variable prevalece sobre la anterior. La antigua variable sigue existiendo, pero ya no se puede hacer referencia a ella en este ámbito.

- En el código siguiente se muestra cómo usar la propiedad reemplazada. Declaramos una variable denominada shadow_num. No definimos la variable como mutable porque cada operación let crea una variable denominada shadow_num mientras se reemplaza la propiedad del enlace de la variable anterior.

```rust
// Declare first variable binding with name "shadow_num"
let shadow_num = 5;

// Declare second variable binding, shadows existing variable "shadow_num"
let shadow_num = shadow_num + 5;

// Declare third variable binding, shadows second binding of variable "shadow_num"
let shadow_num = shadow_num * 2;

println!("The number is {}.", shadow_num);

```

## Tipos de datos

- Rust es un lenguaje con establecimiento de tipos en modo estático. El compilador debe conocer el tipo de datos exacto de todas las variables del código para que el programa se compile y ejecute. Normalmente, el compilador puede inferir el tipo de datos de una variable en función del valor enlazado. No siempre es necesario indicar de forma explícita el tipo en el código. Cuando son posibles muchos tipos, debe informar al compilador del tipo específico mediante anotaciones de tipo.

- En el ejemplo siguiente, se le indica al compilador que cree la variable number como un entero de 32 bits. Especificamos el tipo de datos u32 después del nombre de la variable.

```rust
let number: u32 = 14;
println!("The number is {}.", number);
```

- Si ponemos el valor de la variable entre comillas dobles, el compilador interpreta el valor como texto en lugar de como un número. El tipo de datos deducido del valor no coincide con el tipo de datos u32 especificado para la variable, por lo que el compilador emite un error:

```rust
let number: u32 = "14";
```

```bash
Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:2:23
  |
2 |     let number: u32 = "14";
  |                 ---   ^^^^ expected `u32`, found `&str`
  |                 |
  |                 expected due to this

error: aborting due to previous error
```

### Números: valores enteros y de punto flotante

- Los enteros en Rust se identifican por el tamaño en bits y la propiedad signed. Un entero con signo puede ser un número positivo o negativo. Un entero sin signo solo puede ser un número positivo.

```text
Length                               Con signo         Sin signo
__________________________________________________________________

8 bits                                   i8               u8
__________________________________________________________________

16 bits                                  i16              u16
__________________________________________________________________

32 bits                                  i32              u32
__________________________________________________________________

64 bits                                  i64              u64
__________________________________________________________________

128 bits                                 i128             u128
__________________________________________________________________

dependiente de la arquitectura           isize            usize

```

- Los tipos isize y usize dependen del tipo de equipo en el que se ejecuta el programa. El tipo de 64 bits se usa en una arquitectura de 64 bits y el tipo de 32 bits, en una arquitectura de 32 bits. Si no especifica el tipo para un entero, y el sistema no puede deducir el tipo, asigna el tipo i32 (un entero de 32 bits con signo) de forma predeterminada.

- Rust tiene dos tipos de datos de punto flotante para los valores decimales: `f32 (32 bits)` y `f64 (64 bits)`. El tipo de punto flotante predeterminado es `f64`. En las CPU modernas, el tipo `f64` tiene aproximadamente la misma velocidad que el tipo f32, pero cuenta con una mayor precisión.

```rust
let number_64 = 4.0;      // compiler infers the value to use the default type f64
let number_32: f32 = 5.0; // type f32 specified via annotation
```

- Todos los tipos de números primitivos en Rust admiten operaciones matemáticas como suma, resta, multiplicación y división.

```rust
// Addition, Subtraction, and Multiplication
println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

// Integer and Floating point division
println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
```

- \*\*Cuando llamamos a la macro println, agregamos el sufijo de tipo de datos a cada número literal para informar a Rust sobre el tipo de datos. La sintaxis 1u32 indica al compilador que el valor es el número 1 y que interprete el valor como un entero de 32 bits sin signo.

Si no se proporcionan anotaciones de tipo, Rust intenta deducir el tipo a partir del contexto. Cuando el contexto es ambiguo, asigna el tipo i32 (un entero de 32 bits con signo) de forma predeterminada.\*\*

### Valores booleanos: true o false

- El tipo bool tiene dos valores posibles: true o false.

```rust
// Declare variable to store result of "greater than" test, Is 1 > 4? -- false
let is_bigger = 1 > 4;
println!("Is 1 > 4? {}", is_bigger);
```

### Texto: caracteres y cadenas

- Rust tiene más de dos tipos de cadena. Puede obtener más información sobre los tipos de cadena que se ofrecen en la [documentación de Rust](https://doc.rust-lang.org/book/ch08-02-strings.html).

### Caracteres

- Rust admite valores de texto con dos tipos de cadena básicos y un tipo de carácter. Un carácter es un elemento único, mientras que una cadena es una serie de caracteres. Todos los tipos de texto son representaciones UTF-8 válidas.

```rust
let uppercase_s = 'S';
let lowercase_f = 'f';
let smiley_face = '😃';
```

- Algunos lenguajes tratan sus tipos char como enteros de 8 bits sin signo, que es el equivalente del tipo u8 de Rust. El tipo char de Rust contiene puntos de código Unicode, pero no usa la codificación UTF-8. char en Rust es un entero de 21 bits que se ha agregado para ampliar a 32 bits. char contiene directamente el valor de punto de código sin formato. Puede obtener más información sobre el tipochar de Rust en la [documentación de Rust](https://doc.rust-lang.org/std/primitive.char.html).

### Cadenas (Strings)

- El tipo `str`, también conocido como segmento de cadena, es una vista de los datos de la cadena. La mayoría de las veces, se hace referencia a estos tipos usando la sintaxis del estilo de referencia que precede al tipo con el símbolo de y comercial `&str`.

- Puede imaginarse `&str` como un puntero a datos de cadena inmutables. No todas las cadenas pueden conocerse en tiempo de compilación. Un ejemplo se da cuando un usuario interactúa con un programa en tiempo de ejecución y envía texto mediante un terminal. En estos escenarios, Rust tiene un segundo tipo de cadena denominado `String`. Este tipo se asigna en el montón. Cuando se usa el tipo String, no es necesario conocer la longitud de la cadena (número de caracteres) antes de compilar el código.

- **Los datos de tipo String son datos de texto que pueden cambiar a medida que se ejecuta el programa. Las referencias &str son vistas inmutables en los datos de texto que no cambian a medida que se ejecuta el programa.**

#### Ejemplo de texto

En el ejemplo siguiente se muestra cómo usar los tipos de datos char y &str en Rust

- Se declaran dos variables de caracteres con la sintaxis de anotación : char. Los valores se especifican usando comillas simples.

- Se declara una tercera variable de caracteres y se enlaza a una sola imagen. Para esta variable, se permite que el compilador deduzca el tipo de datos.

- Se declaran dos variables de cadena y se enlazan a sus valores respectivos. Las cadenas se ponen entre comillas dobles.

- Una de las variables de cadena se declara con la sintaxis de anotación : &str para especificar el tipo de datos. El tipo de datos de la otra variable se deja sin especificar. El compilador deducirá el tipo de datos de esta variable en función del contexto.

```rust
// Specify the data type "char"
let character_1: char = 'S';
let character_2: char = 'f';

// Compiler interprets a single item in quotations as the "char" data type
let smiley_face = '😃';

// Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
let string_1 = "miley ";

// Specify the data type "str" with the reference syntax "&str"
let string_2: &str = "ace";

println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
```

Resultado en terminal

```bash
😃 is a Smiley face.
```

## Tuplas y estructuras

### Tuplas

- Una tupla es una agrupación de valores de distintos tipos recopilados en un valor compuesto. Los valores individuales de una tupla se denominan elementos. Los valores se especifican como una lista separada por comas entre paréntesis `(<value>, <value>, ...)`

- Una tupla tiene una longitud fija, que es igual a su número de elementos. Una vez declarada una tupla, no puede aumentar ni reducir su tamaño. No se pueden agregar ni quitar elementos. El tipo de datos de una tupla se define mediante la secuencia de los tipos de datos de los elementos.

```rust
// Tuple of length 3
let tuple_e = ('E', 5i32, true);
```

#### Acceso a elementos de una tupla (ndexación de tupla)

Se puede acceder a los elementos de una tupla por la posición del índice, a partir de cero. Este proceso se conoce como indexación de tupla. Para acceder a un elemento de una tupla, usamos la sintaxis `<tuple>.<index>`.

```rust
// Declare a tuple of three elements
let tuple_e = ('E', 5i32, true);

// Use tuple indexing and show the values of the elements in the tuple
println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);

```

En el ejemplo se muestra la salida siguiente:

```bash
Is 'E' the 5th letter of the alphabet? true
```

### Estructuras

Una estructura es un tipo compuesto por otros tipos. Los elementos de una estructura se denominan campos. Al igual que las tuplas, los campos de una estructura pueden tener tipos de datos diferentes. **Una ventaja importante del tipo de estructura es que puede asignar un nombre a cada campo, por lo que queda claro lo que significa el valor**.

Para trabajar con estructuras en un programa con Rust, en primer lugar debe definir la estructura por nombre y especificar el tipo de datos de cada campo. Después, debe crear una instancia de la estructura con otro nombre. Al declarar la instancia, se proporcionan los valores específicos para los campos.

Rust admite tres tipos de estructura: clásicas, de tupla y de unidad. Estos tipos de estructura admiten diferentes maneras de agrupar y trabajar con los datos.

- Las [estructuras de C](<https://en.wikipedia.org/wiki/Struct_(C_programming_language)>) **clásicas** son las más utilizadas. Cada campo de la estructura tiene un nombre y un tipo de datos. Una vez definida una estructura clásica, se puede acceder a los campos de la estructura usando la sintaxis `<struct>.<field>.`

- Las estructuras de tupla son parecidas a las clásicas, pero sus campos no tienen nombres. A fin de acceder a los campos de una estructura de tupla, usamos la misma sintaxis que para indexar una tupla: `<tuple>.<index>`. Al igual que con las tuplas, los valores de índice de la estructura de tupla empiezan por cero.

- Las estructuras de unidad suelen usarse como marcadores.

Definiciones de ejemplo para las tres variedades de tipos de estructura:

```rust
// Classic struct with named fields
struct Student { name: String, level: u8, remote: bool }

// Tuple struct with data types only
struct Grades(char, char, char, char, f32);

// Unit struct
struct Unit;
```

### Definición de una estructura (struct)

- Para definir una estructura, se escribe la palabra clave struct seguida de un nombre de la estructura. Elija un nombre para el tipo de estructura que describa la característica significativa de los datos agrupados. **el nombre de un tipo de estructura se escribe en mayúsculas.**

- Los tipos de estructura se definen a menudo fuera de la función main y de otras funciones en el programa con Rust. Por este motivo, al inicio de la definición de la estructura no se le aplica sangría desde el margen izquierdo. Solo se le aplica sangría a la parte interna de la definición para mostrar cómo se organizan los datos.

#### Estructura clásica

- Al igual que una función, el cuerpo de una estructura clásica se define entre llaves {}. A cada campo de la estructura clásica se le asigna un nombre único dentro de la estructura. El tipo de cada campo se especifica con la sintaxis : `<type>`. Los campos de la estructura clásica se especifican como una lista separada por comas `<field>, <field>,` .... Una definición de estructura clásica no termina con un punto y coma.

```rust
    // Classic struct with named fields
    struct Developer {
        name: String,
        age: u8,
        email: String,
    }
```

#### Estructura de tupla

- Al igual que una tupla, el cuerpo de una estructura de tupla se define entre paréntesis (). Los paréntesis van inmediatamente después del nombre de la estructura. No hay espacio entre el nombre de la estructura y el paréntesis de apertura

- A diferencia de una tupla, la definición de estructura de tupla incluye solo el tipo de datos de cada campo. Los tipos de datos de la estructura de tupla se especifican como una lista separada por comas `<type>, <type>`

```rust
// Tuple struct with data types only
struct Grades(char, char, char, char, f32);
```

### Creación de una instancia de una estructura

Después de definir un tipo de estructura, para usar la estructura se crea una instancia del tipo y se especifican valores para cada campo. Al establecer los valores de campo, no es necesario especificar los campos con el mismo orden con el que están definidos.

En el ejemplo siguiente se usan las definiciones que hemos creado para los tipos de estructura Student y Grades.

```rust
// Instantiate classic struct, specify fields in random order, or in specified order
let user_1 = Student { name: String::from("Constance Sharma"), remote: true, level: 2 };
let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };

// Instantiate tuple structs, pass values in same order as types defined
let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
         user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
         user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);

```

#### Conversión de un literal de cadena en un tipo String

- Los datos de cadena que se almacenan dentro de otra estructura de datos, como una estructura o un vector, se deben convertir de una referencia literal de cadena (`&str`) a un tipo String. Para realizar la conversión, se usa el método `String::from(&str)` estándar.

```rust
// Classic struct with named fields
struct Student { name: String, level: u8, remote: bool }
...
let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };
```

Si no se convierte el tipo antes de asignar el valor, el compilador emite un error:

```bash
error[E0308]: mismatched types
  --> src/main.rs:24:15
   |
24 |         name: "Dyson Tan",
   |               ^^^^^^^^^^^
   |               |
   |               expected struct `String`, found `&str`
   |               help: try using a conversion method: `"Dyson Tan".to_string()`

error: aborting due to previous error
```

- El compilador sugiere que se puede usar la función .to_string() para realizar la conversión. En los ejemplos, se usa el método String::from(&str).

```rust
struct Developer {
    name: String,
    age: u8,
    email: String,
}
fn main() {
    let devs = Developer {
        name: "James Osorio Florez".to_string(),
        age: 28,
        email: "OssRezz.13@gmail.com".to_string(),
    };

    println!("Dev name: {}, age: {}, email: {}", devs.name, devs.age, devs.email);
}

```

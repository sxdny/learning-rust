# Conceptos básicos de Rust

```rust
fn main() {
    println!("Hello, world!")
}
```

`fn` sirve para crear una función. En el `println!` llama a una macro, no a una función. La función "normal" seria `println`.

## Cargo

Cargo es un administrador de paquetes para Rust.

Sirve para crear proyectos y importar dependencias de manera más
rápida.

Para comprobar que lo tenemos instalado:

```bash
cargo --version
```

Si queremos crear un nuevo proyecto con `cargo`, escribiremos en la terminal:

```bash
cargo new nombre_proyecto
```

Esto generá una carpeta (a lo node al crear un proyecto), junto a una carpeta `src/` y un archivo llamado `Cargo.toml` con las dependencias.

## Build a Cargo projecto

Para "buildear" el proyecto con cargo, tendremos que ejecutar el siguiente comando:

```bash
cargo build
```

Esto generará un archivo `.exe`. Una vez haya aparecido, lo ejecutamos:

```bash
./target/debug/hello_cargo.exe
```

Estos dos comandos se pueden "simflicar" en un `cargo run`. **Simplemente, es más eficiente**.

También se puede usar `cargo check` para comprobar que el código se compila sin errores, pero no genera un archivo `.exe`.

## Build for release

Una vez hayas acabado completamente tu proyecto, puede compilarlo con optimizaciones con el comando `cargo build --release`. Hace tu código Rust más rápido.

Recuerda que el archivo `.toml` es como un `package.json`, así que al utilizar git, solo basta subir ese archivo al repositorio remoto:

```bash
git clone example.org/someproject
cd someproject
cargo build
```

## Handle errors

En este caso:

```rust
let guess: u32 = guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue, // continue --> go to the next iteration of the loop
}
```

Si el usuario introduce un número, devolverá una variante `Ok` y seguirá con el programa. En cambio, si sale `Err`, se ejecutará `continue`. Esto irá a la siguiente interacción del loop (la siguiente interacción es introducir nuevamente el número)

# Conocimientos básicos de la Programación en Rust

## Variables e inmutabilidades.

Como hemos visto antes en el capítulo 2, las variables se pueden separar en dos grupos:

- Valores mutables. El valor de podrá cambiar una segunda vez.
- Valores inmutables. Una vez asignado del valor, este no se podrá cambiar.

Ejemplo:

```rust
fn main() {
  let x = 5; // immutable variable
  println!("The value of x is: {}", x);
  // x = 6; // error: cannot assign twice to immutable variable `x`
  let mut y = 5; // mutable variable
  println!("The value of y is: {}", y);
  y = 6;
  println!("The value of y is: {}", y);
}
```

## Shadowing

El shadowing sucede cuando declaramos otra variable con el mismo nombre, por lo
que el compilador solo verá la última declaración que hayamos hecho.

Si declaramos una variable varias veces, los valores siguiente no se resetearán,
sino que se añadiran a la declaración de la variable. Ejemplo:

```rust
let x = 5; // aqui el valor es 5

    let x = x + 1; // aqui el valor es 6 (toma el valor de la x anterior y le suma 1)

    {
        let x = x * 2; // aqui el valor es 12 (toma el valor de la x anterior y lo multiplica por 2)
        println!("The value of x in the inner scope is: {x}");
    }

    // como estamos fuera del scope anterior, el valor de x es 6
    println!("The value of x is: {x}");
```

El shadowing es diferente a hacer las variables mutables, ya que nos dará un error
si intentamos cambiar el valor de una variable sin usar `let`. Por lo que podemos
usar el let para hacer cambios a la variable y después si queremos dejarla inmutable.

Otra diferencia es que, como tecnicamente estamos creando otra variable, podemos
cambiar el tipo de valor de esta si que no de un error.

# Tipos de datos

Cada valor en Rust tiene que tener un tipo de valor.

Antes de todo, Rust tiene que saber cuáles son los tipos de datos de todas las variables que se vaya a utilizar en el programa.

Un ejemplo es el siguiente:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```
Existen dos tipos de datos en Rust:
- **Los Scalar**: que son los más básicos -> integer, float, booleanos y carácteres.
- **Compound types:** son los que pueden almacenar varios valores dentro de un tipo de valor.

## Scalar

### Tipos de integers

Principalmente, existen 2 tipos de integers:
- Con signo. Se representan con una `i` delante del tamaño del integer.
- Sin signo. Se representan con una `u` delante del tamaño del integer.

| Length |Signed |Unsigned |
|--------|-------|---------|
|8-bit|	i8|	u8 |
|16-bit |	i16 |	u16 |
| 32-bit |	i32 |	u32 |
| 64-bit |	i64 |	u64 |
| 128-bit |	i128 |	u128 |
| arch |	isize |	usize |

Los valores con signo pueden almacenar número desde el -2 elevado a n - 1 hasta el 2 elevado a n menos 1.
Por otro lado, los valores sin signo, solo pueden almacenar el 0 hasta el 2 elevado a n - 1 (no permiten número negativos)

El `arch` depende de la arquitectura en la que se esta programando. Puede ser de 64 o 32 bits.

## Compund types

### Tuple

Un tuple almacenar varios valores de distinto tipo. Un ejemplo es el siguiente:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

Después para acceder los valores, haremos lo siguiente:

```rust
let five_hundred = x.0;

let six_point_four = x.1;

let one = x.2;
```

### Array

Podremos definiar una array de la siguiente manera:

```rust
let a = [1, 2, 3, 4, 5];
```

En las arrays, también podremos definir tipos de valores:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Aquí el `i32` nos indica el tipo de valor y el `5` nos indica la longitud de la array.

Podremos también inicializar una array con valores iguales rápidamente de la siguiente manera:

```rust
let a = [3; 5]; // en los 5 índices, habrás 3s.
```

Para acceder a los valores de la array:

```rust
let a = [1, 2, 3, 4, 5];

let first = a[0];
let second = a[1];
```

# Funciones

Como en JavaScript, no importa donde definas las funciones. Mientras estén definidas, todo bien.

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

También les podremos pasar parámetros a las funciones, siempre y cuando definamos el tipo de valor que estas van a recibir:

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

## Devolver valores de una función

Para devolver valorse en una función, no tenemos `return`, sino que usaremos una flecha: `->` para indicar que la función devuelve un valor:

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

Esto de arriba, es lo mismo que hacer un:

```rust
let x = 5
```

Hay que tener cuidado, ya que si ponemos un `;` al final, el valor ya no se devolverá:

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1; // <-- No devolverá el valor
}
```

# Flow Control

Podemos usar también `if else` en las declaraciones:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

Importante tener en cuenta que los dos valores del `if` tienen que ser los mismo, sino nos dará un error de código.

# Loops

Lo único a destacar de esta parte es que podemos usar un `.rev()` para recorrer un elemento al revés:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!"); // 4, 3, 2, 1...
    }
    println!("LIFTOFF!!!");
}
```




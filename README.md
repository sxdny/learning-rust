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

## Variables e inmutabilidades-

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

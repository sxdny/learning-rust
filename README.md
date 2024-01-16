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

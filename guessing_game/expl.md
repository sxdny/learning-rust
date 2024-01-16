# Explicación del programa

El `use std::io` importa una librería de Rust, la cuál incluye un montón de cosas útiles. Entre ellas, se incluye la habilidad de usar un input.

## Variebles mutables e inmutables

Por defecto, las variables en Rust son inmutables. Esto significa que una vez dado el valor, no se podrá modificar. En cambio, si definimos a una variable cono mutable `let mut guess = ..` si que le podremos cambiar el valor.

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

El `String::new()` es básicamente un `String.new()`. Esto significa que la función `new()` está asociada con el objeto `String`. Está creando un string vacio.

## Receivig user input

Dentro del `io::stdin()`, que es una función asociada a io, tendremos:

- `.read_line(&mut guess)`. Leeremos el input del usuario. Le pasamos como argumento la variable `&mut guess`, para decirle donde tiene que guardar el input.
- `.expect("Failed to read line")`. Esto lo que hace es que si hay un error al leer el archivo, nos lo dice sin que pete el programa.

La función `.read_line()` devuelve un Result, que devuelve un estado.

Las variantes de Result son `Ok` y `Err`. Estas dos variantes las pillará el `.expect(...)`. Si detecta que devuelve `Err`, ejecutará el código sino no hará nada.

Si no llamamos a `.expect()`, el código se compilará pero nos advertirá.

```bash
cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Finalmente, una manera de imprimir variables es, como lo vemos a abajo, con: `{variable}` en la función `println!()`.

## Maneras de imprimir las variables

```rust
println!("Secret number is: {secret_number}");
println!("You guessed: {}", guess);
```

## Comparar dos variables

Primero, tendremos usar otro `use` para importar las funcionalidades para comparar.

`Ordering` es otro enum con las siguientes variantes: `Less`, `Greater` y `Equal`. Son los 3 posibles returns que tiene.

`match` lo usaremos para comparar una variable con la otra. Es como un switch.

```rust
match variable1.cmp(&variable2) {
    // ...
}
```

Dentro de las llaves, usaremos `Ordering` para imprimir el resultado. Teniendo en cuenta la variante que devuelva `Ordering`, se imprimirá un texto u otro.

## Interface

El problema ahora es que Rust es muy quisquilloso con las variables y no comparará un String con un número.

Para poder compararlo entonces, tendremos que convertir el String en un número:

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Esto es el *shadowing*, que sirve para reutilizar variables y asi no tener que crear 2 variables distintas.

De momento, solo tienes que saber que esto se utiliza bastante para cuando convertiomos una variable de un tipo a otro.

## Loops

```rust
loop {
    // bucle infinito
}
```



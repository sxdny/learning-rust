fn main() {
  let x = 5; // immutable variable
  println!("The value of x is: {}", x);
  // x = 6; // error: cannot assign twice to immutable variable `x`
  let mut y = 5; // mutable variable
  println!("The value of y is: {}", y);
  y = 6;
  println!("The value of y is: {}", y);
  // también se puede poner {x} y {y} en lugar de {} en el println!

  // También podemos declarar constantes
  // u32 es un tipo de dato de 32 bits sin signo (no permite negativos)
  const MAX_POINTS: u32 = 100_000;

  // Shadowing
  let x = 5; // aqui el valor es 5

    let x = x + 1; // aqui el valor es 6 (toma el valor de la x anterior y le suma 1)

    {
        let x = x * 2; // aqui el valor es 12 (toma el valor de la x anterior y lo multiplica por 2)
        println!("The value of x in the inner scope is: {x}");
    }

    // como estamos fuera del scope anterior, el valor de x es 6
    println!("The value of x is: {x}");
}
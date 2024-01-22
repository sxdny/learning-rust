fn main() {
  let x = 5; // immutable variable
  println!("The value of x is: {}", x);
  // x = 6; // error: cannot assign twice to immutable variable `x`
  let mut y = 5; // mutable variable
  println!("The value of y is: {}", y);
  y = 6;
  println!("The value of y is: {}", y);
}
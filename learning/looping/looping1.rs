fn main() {
  let max_power = 10;
  let mut power = 1;
  while power < max_power {
        print!("{} ", power); // prints without newline
        power += 1;           // increment counter
  }

  for n in 1..11 {println!("The square of {} is {}", n, n * n);}

  // Variables can also be used in a range, like in the following snippet that prints nine dots:
  let mut x = 10;
  for _ in 1 .. x { x -= 1; print!("."); }
  println!("");

  loop_read(power);
}


fn loop_read(mut power:i32) {

loop {
  power += 1;
  if power == 42 { // Skip the rest of this iteration
    continue;
  }
  print!("{}  ", power);
  if power == 50 {
    println!("OK, that's enough for today");
    break;  // exit the loop
  }
}
}
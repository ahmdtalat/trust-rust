pub fn run() {
  let name = "Ahmed";
  let mut age = 24;
  println!("My name is {} and I'm {}.", name, age);
  age = 25;

  println!("My name is {} and I'm {}.", name, age);
  println!("max u64 : {}", std::u64::MAX);
  println!("max i64 : {}", std::i64::MAX);
}

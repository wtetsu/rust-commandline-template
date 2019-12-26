pub fn plus(a: i32, b: i32) -> i32 {
  a + b
}

pub fn print_arguments(args: Vec<String>) {
  for (i, a) in args.iter().enumerate() {
    println!("{}:{}", i, a);
  }
}

pub fn run() {
  // create a empty string
  let mut s = String::new();
  println!("S is {}", s);

  // create a string with initial value
  s = "Initial content".to_string();
  println!("S is {}", s);
  s = String::from("Initial content");
  println!("S is {}", s);

  // Add text to original string
  let mut s = String::from("foo");
  s.push_str("bar");
  println!("S is {}", s);

  // push_str() don't get the ownership of the param
  let mut s1 = String::from("foo");
  let mut s2 = String::from("bar");
  s1.push_str(&s2);
  println!("S1 is {}", s1);
  s2.push_str("!");
  println!("S2 is {}", s2);

  // push() only can add one character
  let mut s = String::from("lo");
  s.push('l');

  // Using + to concatenate strings
  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2;
  println!("s3 is {}", s3);
  // + take own of s1 and add the slice of s2,
  // because this, s1 won't be able anymore, but s2 yes
  // println!("s1 is {} and s2 is {}", s1, s2);
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  // Instead this
  // let s = s1 + "-" + &s2 + "-" + &s3
  // Use this
  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("s is {}", s);
  // It's more legible and all variables will be able to reuse after

  // Iterating each element from a string
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  // Iterating each byte from a string
  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}

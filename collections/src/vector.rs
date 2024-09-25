pub fn run() {
  // Setting type because de vector is empty
  // let v: Vec<i32> = Vec::new();
  // Setting a filled vector
  let mut v = vec![1, 2, 3];
  v.push(4);
  v.push(5);
  let third = &v[2];
  println!("Third is {}", third);
  let third = v.get(2);
  if third.is_none() { println!("Third does not exist") }
  else { println!("Third is {:?}", third); }

  // Try access an invalid index will panic the program
  // let does_not_exist = &v[100];
  // println!("Index does not exist: {}", does_not_exist);
  let does_not_exist = v.get(100);
  println!("Index does not exist: {:?}", does_not_exist);

  // It's impossible set an index from mutable array
  //  to a immutable variable
  let mut v = vec![1, 2, 3, 4, 5];
  let first = &v[0];
  v.push(6);
  // Try access the immutable variable after change the original vector
  //  will break the build
  // println!("First is {}", first);
  println!("v is {:?}", v);

  // Using a vector of Enum
  #[derive(Debug)]
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12)
  ];
  println!("Row is {:#?}", row);
}
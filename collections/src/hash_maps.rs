use std::collections::HashMap;

pub fn run() {
  // Setting a new empty HashMap
  let mut scores = HashMap::new();

  // Inserting values to HashMap
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  println!("Scores is {:?}", scores);

  // Using Vector and Tuple to create a HashMap
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];
  let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
  println!("Scores is {:?}", scores);

  // Types without Copy trait such as String (Owned), have his own moved to HashMap
  let field_name = String::from("Key");
  // let field_value = String::from("Value");
  let field_value = 5; // With i32 which have a Copy trait, the values are only copied
  let mut map = HashMap::new();
  map.insert(&field_name, &field_value);
  // println!("Field name is {} and Field value is {}", field_name, field_value); Error: the variables doesn't exist anymore
  println!("Field value is {}", field_value);

  // get() to get a specific value from HashMap
  let team_name = String::from("Blue");
  let blue_score = scores.get(&team_name);
  if blue_score.is_none() { println!("Blue score not found") }
  else { println!("Blue score is {:?}", blue_score); }

  // Iterating a HashMap
  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  let mut scores = HashMap::new();
  // Updating a HashMap rewriting the value
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);
  println!("{:?}", scores);

  // Insering in a HashMap only if the key is empty
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);
  println!("{:?}", scores);

  // Updating the values of a key in a HashMap
  for _ in "add points to Red".split_whitespace() {
    let count = scores.entry(String::from("Red")).or_insert(0);
    *count += 10;
  }
  println!("{:?}", scores);
}

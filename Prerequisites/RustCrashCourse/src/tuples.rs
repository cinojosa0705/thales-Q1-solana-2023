pub fn run() {
    let person: (&str, &str, i8) = ("Thales", "Italy", 17);
  
    println!("{} is from {} and is {}", person.0, person.1, person.2);
  }
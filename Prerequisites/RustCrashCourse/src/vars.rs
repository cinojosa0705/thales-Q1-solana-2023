pub fn run() {
    let name = "Thales Brrrr";
    let mut age = 17;
    println!("My name is {} and I am {}", name, age);
    age = 18;
    println!("My name is {} and I am {}", name, age);
  
    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);
  
    // Assign multiple vars
    let ( my_name, my_age ) = ("Thales", 17);
    println!("{} is {}", my_name, my_age );
  }
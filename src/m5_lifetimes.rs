#[allow(dead_code, unused_variables)]
fn example_0() {
  let r: &i32; // 'a         
                                   
  let x: i32 = 5;  // 'b  
  r = &x;                     
                        
  println!("r: {}", r); 
}

#[allow(dead_code, unused_variables)]
fn example_1() {

  // Allocate space in memory
  let highest_age: i32;

  // Initialize vars
  let alice_age: i32 = 20; // 'a
  let bob_age: i32 = 21; // 'b: 'a

  // Call function
  highest_age = largest(&alice_age, &bob_age);

  // Print output
  println!("Highest age is {}", highest_age);

  fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
    if compare_1 > compare_2 {
      *compare_1
    } else {
      *compare_2
    }
  }
}


#[allow(dead_code, unused_variables)]
fn example_2() {

  // Allocate space in memory
  let highest_age: &i32;
  let new_value: i32;

  // Initialize vars
  let alice_age: i32 = 20; // 'a

  {
    let bob_age: i32 = 21; // 'b
  
    // Call function
    highest_age = largest(&alice_age, &bob_age);
    new_value = *highest_age;
  } // 'b out of scope

  // Print output
  println!("New value age is {}", new_value);

  fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
    if compare_1 > compare_2 {
      compare_1
    } else {
      compare_2
    }
  }
}


#[allow(dead_code, unused_variables)]
fn example_3_generics() {

  // Allocate space in memory
  let highest_age: &i32;
  let new_value: i32;

  // Initialize vars
  let alice_age: i32 = 20; // 'a

  {
    let bob_age: i32 = 21; // 'b
  
    // Call function
    highest_age = largest::<i32>(&alice_age, &bob_age);
    new_value = *highest_age;
  } // 'b out of scope

  // Print output
  println!("New value age is {}", new_value);

  fn largest<'a, 'b: 'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'b T) -> &'a T {
    if compare_1 > compare_2 {
      compare_1
    } else {
      compare_2
    }
  }
}


#[allow(dead_code, unused_variables)]
struct Person<'p, 'q> {
  name: &'p str,
  points: &'q f32
}


#[allow(dead_code, unused_variables)]
fn example_4_with_struct() {

  // Allocate space in memory
  let highest_age: &f32;
  let new_value: f32;

  // Initialize vars
  let alice: Person = Person { name: "alice", points: &50.2 };

  {
    let bob: Person = Person { name: "bob", points: &40.5 };
  
    // Call function
    highest_age = largest::<f32>(alice.points, bob.points);
    new_value = *highest_age;
  } // 'b out of scope

  // Print output
  println!("New value age is {}", new_value);

  fn largest<'a, 'b: 'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'b T) -> &'a T {
    if compare_1 > compare_2 {
      compare_1
    } else {
      compare_2
    }
  }
}

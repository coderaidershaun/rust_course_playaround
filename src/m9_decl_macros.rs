// MACRO CAPTURES

/* expr
    matches to a valid rust expression
    "hello".to_string(), vec![1, 2, 3], 1 + 2, 1
*/

/* stmt
    matches to a rust statement
    let x = 5, x.push(1), return Some(x)
*/

/* ident
    matches to a rust identifier
    variable name, function name, module name
*/

/* ty
    matches to a rust type
    i32, Vec<String>, Option<T>
*/

/* path
    matches to a rust path
    std::collections::HashMap
*/

// REPITITION SPECIFIER

// * - match zero or more repititions
// + - match one or more repititions
// ? - Match zero or one repetition

#[cfg(test)]
mod tests {
  

  macro_rules! mad_skills {
    // ($x: expr) => {
    //   format!("You sent an expression: {}", $x)
    // };
    ($x: ty) => {
      match stringify!($x) {
        "i32" => "You sent an i32 type".to_string(),
        _ => "You sent something else".to_string(),
      }
    }
  }


  macro_rules! my_vec {
    ( $($x: expr),+ ) => {
      {
        let mut temp_vec = Vec::new();
        $(
          temp_vec.push($x);
        )+
        temp_vec
      }
    }
  }


  #[test]
  fn tests_declarative_macro() {

    let mut _x: Vec<i32> = vec!();
    let y: Vec<i32> = my_vec!(1);
    dbg!(y);

    let some_var: String = mad_skills!(u8);
    dbg!(some_var);

  }
}

#[cfg(test)]
mod test {

  #[derive(Debug)]
  enum Message {
    Quit,
    ChangeColour(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String)
  }
  
  fn process_message(msg: Message) {
    match msg {
      Message::Quit => {
        println!("I quit!");
      },
      Message::ChangeColour(red, green, blue) => {
        println!("Red {}, Green {}, Blue {}", red, green, blue);
      },
      Message::Move { x, y: new_name } => {
        println!("X is {}, Y as new_name is {}", x, new_name);
      }
      Message::Write(text) => {
        println!("{}", text);
      }
    };
  }

  #[test]
  fn tests_large_enum() {
    let _my_quit: Message = Message::Quit;
    let _my_colour: Message = Message::ChangeColour(10, 20, 255);
    let _my_move: Message = Message::Move { x: 10, y: 200 };
    let my_write: Message = Message::Write("My awesome string".to_string());
    process_message(my_write);
  }

  #[test]
  fn tests_match_literals() {
    
    let number: i32 = 20;

    let res: &str = match number {
      1 => "It was the first!",
      2 | 3 | 5 | 7 | 15 | 20 => "We found it in the sequence!",
      _ => "It was something else!"
    };

    println!("{}", res);
  }

  #[test]
  fn tests_match_option() {

    let some_num: Option<i32> = Some(10);

    let my_int: i32 = if let Some(i) = some_num {
      i
    } else {
      panic!("There was a problem");
    };

    println!("My int: {}", my_int);

    // let res = match some_num {
    //   Some(i) => i,
    //   None => {
    //     panic!("There was a problem");
    //   }
    // };

    // println!("{:?}", some_num);
    // println!("{}", res);
  }


  #[test]
  fn tests_match_result() {

    let some_res: Result<i32, &str> = Ok(50);
    let _some_err: Result<i32, &str> = Err("There was a problem");

    let res = match some_res {
      Ok(val) => val,
      Err(e) => panic!("{}", e)
    };

    println!("{}", res);

    let my_int: i32 = if let Ok(r) = some_res {
      r
    } else {
      panic!("There was a problem");
    };

    println!("{}", my_int);
  }

  #[test]
  fn tests_match_guard() {
    let pair: (i32, i32) = (2, -2);
    match pair {
      (x, y) if x == y => println!("They match!"),
      (x, y) if x + y == 0 => println!("They neutralize"),
      (_, y) if y == 2 => println!("Y is indeed +2"),
      _ => println!("We are no bothered")
    };
  }

  #[test]
  #[allow(unused_variables)]
  fn tests_match_struct() {

    struct Location {
      x: i32,
      y: i32
    }

    let location: Location = Location { x: 0, y: 20 };

    match location {
      Location { x, y: 0 } => println!("Y is on the axis"),
      Location { x: 0, y } => println!("X is on the axis"),
      Location { x, y } => println!("X and Y are not on the axis"),
    };

  }
}

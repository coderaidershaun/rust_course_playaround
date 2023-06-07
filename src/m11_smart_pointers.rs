#[cfg(test)]
mod tests {
  use std::rc::{Rc, Weak};
  use std::cell::RefCell;

  #[test]
  #[allow(dead_code, unused_variables)]
  fn tests_box_smart_pointer() {
    
    #[derive(Debug)]
    struct Node {
      id: u32,
      next: Option<Box<Node>>
    }
    
    let nodes: Box<Node> = Box::new(
      Node { id: 0, next: Some(
        Box::new(Node { id: 1, next: Some(Box::new(
          Node { id: 2, next: None }
        ))})
      )}
    );

    dbg!(nodes);
    
  }

  #[test]
  #[allow(dead_code, unused_variables)]
  fn tests_reference_counter() {
    
    let x: Rc<RefCell<i32>> = Rc::new(RefCell::new(50));
    let y: Rc<RefCell<i32>> = Rc::clone(&x);
    let z: Rc<RefCell<i32>> = Rc::clone(&x);

    *x.borrow_mut() = 70;

    dbg!(x.borrow());
    dbg!(y.borrow());
    
    // Credit to Bocksdin Coding and Lets Get Rusty for inspiration

    #[derive(Debug, Clone)]
    struct House {
      address_number: u16,
      street: String,
      furniture: RefCell<Vec<Rc<Furniture>>>
    }

    #[derive(Debug, Clone)]
    struct Furniture {
      id: String,
      description: String,
      house: Weak<House>
    }

    let house_1 = Rc::new(House {
      address_number: 123,
      street: "coding avenue".to_string(),
      furniture: RefCell::new(vec!())
    });

    let table = Rc::new(Furniture {
      id: "table1".to_string(),
      description: "kitchen table".to_string(),
      house: Rc::downgrade(&house_1)
    });
    
    let desk = Rc::new(Furniture {
      id: "desk1".to_string(),
      description: "office desk".to_string(),
      house: Rc::downgrade(&house_1)
    });

    house_1.furniture.borrow_mut().push(Rc::clone(&table));
    house_1.furniture.borrow_mut().push(Rc::clone(&desk));

    dbg!(house_1);

  }
}

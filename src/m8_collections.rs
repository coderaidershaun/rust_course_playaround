#[cfg(test)]
mod test {
  use std::collections::{HashMap, HashSet};

  #[test]
  fn tests_hashmap() {

    let person_1: &str = "alice";
    let person_2: &str = "bob";

    // &str -> Person
    // u8 -> &str
    // &str -> u32

    let mut results_hm: HashMap<&str, u32> = HashMap::new();
    results_hm.insert(person_1, 55);
    results_hm.insert(person_2, 51);

    let test_score: Option<&u32> = results_hm.get(person_1);
    dbg!(test_score.unwrap());

    if results_hm.contains_key("alice") {
      dbg!("Alice is present!");
    }
  }

  #[test]
  fn tests_hashset() {
    let mut names_hs: HashSet<&str> = HashSet::new();
    names_hs.insert("alice");
    names_hs.insert("bob");
    names_hs.insert("jane");

    if names_hs.contains("bob") {
      dbg!("Bob is here!");
    }
  }
}
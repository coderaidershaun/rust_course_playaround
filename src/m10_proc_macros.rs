#[cfg(test)]
mod test {
  use my_proc_macro::function_to_string;

  const OUTPUT: &str = "";

  #[function_to_string]
  fn some_function_for_ai_llm(_whatever_param: &str) {
    /// This is an awesome function
    /// We shall give it to an AI to guess and output
    /// In a structured manner
    println!("{}", OUTPUT);
  }

  #[test]
  fn tests_proc_macro() {
    
    let x: &str = some_function_for_ai_llm("some_input");
    dbg!(x);
  }
}

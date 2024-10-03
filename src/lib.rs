pub enum Case {
  No,
  Yes
}

pub fn new(case: &Case) -> String {
  match case {
    Case::No => String::from("No"),
    Case::Yes => String::from("Yes")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn work() {
    let yes = Case::Yes;
    let no = Case::No;
    assert_eq!(new(&no), "No");
    assert_eq!(new(&yes), "Yes");
  }
}
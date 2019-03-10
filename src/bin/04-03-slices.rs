fn main() {
  let mut s = String::from("hello world");
  let word = first_word_idx(&s);

  s.clear();
  // word is sad

  let mut s = String::from("hello");
  let s_len = s.len();
  let slice = &s[0..2];
  let slice = &s[..2];
  let slice = &s[..=2];
  let slice = &s[3..s_len];
  let slice = &s[3..];
  let slice = &s[0..s_len];
  let slice = &s[..];
  // doesn't work because borrowed
  // s.clear();
  let mut s2 = String::from("hello");
  first_word(&s2);
  s2.clear();
  println!("slice = {:?}", slice);

  let string_literal = "hello";
  let word = first_word(string_literal);
  let word = first_word(&string_literal);


  let a = [1, 2, 3, 4, 5];
  let slice = &a[1..3];
  println!("slice = {:?}", slice);
}

fn first_word_idx(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }
  s.len()
}

fn first_word_string(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}

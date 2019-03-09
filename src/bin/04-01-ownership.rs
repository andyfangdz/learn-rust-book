fn main() {
  let s = String::from("hello");
  let mut s = String::from("hello");

  s.push_str(", world!");

  println!("{}", s);

  let s1 = String::from("hello");

  // let s2 = s1;
  // println!("{}", s1);
  // doesn't work

  let s2 = s1.clone();
  println!("s1 = {}, s2 = {}", s1, s2);
  takes_ownership(s2);

  let x = 5;
  let y = x;
  println!("x = {}, y = {}", x, y);
  makes_copy(x);

  let s1 = gives_ownership();
  let s2 = String::from("hello");
  let s3 = takes_and_gives_back(s2);

  // println!("s2 = {:?}", s2);
  // doesn't work!
  println!("s1 = {:?}, s3 = {:?}", s1, s3);

  let (s2, len) = calculate_length(s1);

  println!("The length of {:?} is {}.", s2, len);
  // println!("This doesn't work {}.", s1);
}

fn takes_ownership(some_string: String) {
  println!("takes ownership of {:?}", some_string);
}

fn makes_copy(some_integer: i32) {
  println!("makes copy of {:?}", some_integer)
}

fn gives_ownership() -> String {
  let some_string = String::from("hello");

  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();
  (s, length)
}

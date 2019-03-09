fn main() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of {:?} is {}.", s1, len);

  let mut s = String::from("hello");
  change(&mut s);
  // doesn't work
  // change(&s1);

  let r1 = &mut s;
  let r2 = &mut s;

  // doesn't work
  // println!("{}, {}", r1, r2);
  println!("r2 = {:?}", r2);

  let mut s = String::from("hello");

  {
    let r1 = &mut s;
    println!("r1 = {:?}", r1);
  }

  let r2 = &mut s;
  println!("r2 = {:?}", r2);

  let ir1 = &s;
  let ir2 = &s;
  // doesn't work, kinda like RW Lock?
  let ir3 = &mut s;
  // println!("ir1 = {:?}", ir1);
  // println!("ir2 = {:?}", ir2);
  println!("ir3 = {:?}", ir3);

  // doesn't work
  // let broken_ref = dangle();
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

/*
fn dangle() -> &String {
  let s = String::from("hello");

  &s
}
*/

fn no_dangle() -> String {
  String::from("hello")
}

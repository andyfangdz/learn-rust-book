struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  let mut user1 = User {
    email: String::from("a@b.com"),
    username: String::from("abcom"),
    active: true,
    sign_in_count: 1,
  };
  user1.email = String::from("hello@b.com");
  let user2 = build_user(String::from("a@b.com"), String::from("lol"));
  let user2 = User {
    email: String::from("a@b.com"),
    username: String::from("lol"),
    ..user1
  };
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
  println!("black = {:?}", black);
}

fn build_user(email: String, username: String) -> User {
  User {
    email: email,
    username: username,
    active: true,
    sign_in_count: 1,
  }
}

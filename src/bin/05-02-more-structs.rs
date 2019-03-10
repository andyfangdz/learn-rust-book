fn main() {
  let width = 30;
  let height = 50;

  println!("{}", area_scalar(width, height));

  let rect1 = (30, 50);
  println!("{}", area_tuple(rect1));

  let rect2 = Rectangle { width: 30, height: 50 };
  println!("{}", area(rect2));
  println!("rect2 is: {:?}", rect2);
  let rect3 = NonClonableRectangle { width: 30, height: 50 };
  area_ref(&rect3);
  println!("rect3 is: {:#?}", rect3);
}

fn area_scalar(width: u32, height: u32) -> u32 {
  width * height
}

fn area_tuple(dim: (u32, u32)) -> u32 {
  dim.0 * dim.1
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn area(rect: Rectangle) -> u32 {
  rect.width * rect.height
}
#[derive(Debug)]
struct NonClonableRectangle {
  width: u32,
  height: u32,
}

fn area_ref(rect: &NonClonableRectangle) -> u32 {
  rect.width * rect.height
}

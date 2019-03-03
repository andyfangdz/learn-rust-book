fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("{}", y);
    println!("{}", tup.1);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
}

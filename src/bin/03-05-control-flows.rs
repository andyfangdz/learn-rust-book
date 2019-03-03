fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value is: {}", number);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let a = [1, 2, 3, 4, 5];

    for elem in a.iter() {
        println!("value is: {}", elem);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!");
}

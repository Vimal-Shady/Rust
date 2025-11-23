use std::io;
fn main() {
    println!("Hello, world!");
    println!("Enter the guess:");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to readline");
    println!("You guess:{guess}");
}

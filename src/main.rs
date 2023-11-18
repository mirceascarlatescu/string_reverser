use std::io;

fn main() {
    println!("Please insert the string you want to reverse:");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let chars = guess.chars().rev();
    let reversed: String = chars.collect();

    println!("Reversed is {reversed}");
}
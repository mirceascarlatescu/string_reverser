use std::io;

fn main() {
    println!("Please insert the string you want to reverse:");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let chars = guess.chars().rev();
    let mut reversed = "".to_owned();
    for char in chars
    {
        reversed.push_str(&char.to_string());
    }
    
    println!("Reversed is {reversed}")
}
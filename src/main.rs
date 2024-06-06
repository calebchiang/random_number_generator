use std::io;
use rand::Rng;

fn get_password_length() -> usize {
    println!("Enter password length: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let length: usize = input.trim().parse().expect("Please enter a valid number!");
    length
}

fn generate_password(length: usize) -> String {
    let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()";
    let mut password = String::new();
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        let idx = rng.gen_range(0..characters.len()); // Randomly generated to point to one index in the characters string
        password.push(characters.chars().nth(idx).unwrap()); // Appends the element found in that randomly generated index to the password variable
    }

    password

}


fn main() {
    let length = get_password_length();
    let password = generate_password(length);
    println!("Generated password: {}", password);
}

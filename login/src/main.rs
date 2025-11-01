use authentication::{greet_user, login, read_line};

fn main() {
    let mut tries = 0;
    loop {
        println!("Please enter your username:");
        let username = read_line();
        println!("Enter your password");
        let password = read_line();
        if login(&username, &password) {
            println!("Login successful");
            break;
        } else {
            println!("Password or username incorrect or does not exist");
            tries += 1;
            if tries >= 3 {
                println!("Too many attempts ");
                break;
            }
        }
    }
}

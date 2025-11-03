use authentication::{greet_user, login, read_line, LoginAction, LoginRole};

fn main() {
    let mut tries = 0;
    loop {
        println!("Please enter your username:");
        let username = read_line();
        println!("Enter your password");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    authentication::LoginRole::Admin => {println!("Admin login")}
                    authentication::LoginRole::User => {println!("User login")}
                }
                break;
            }
            Some(LoginAction::Denied) => {}
            None => {println!("No login found")}
        }
        println!("Password or username incorrect or does not exist");
        tries += 1;
        if tries >= 3 {
            println!("Too many attempts ");
            break;
        }
    }
}

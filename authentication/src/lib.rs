pub fn greet_user(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    if username != "admin" && username != "bob" {
        return None;
    }

    if username == "admin" && password == "password" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if username == "bob" && password == "password" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied)
    }

    // username.to_lowercase() == "admin" && password == "password"
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}


#[derive(PartialEq, Debug)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!(greet_user("John"), "Hello, John!");
    }

    #[test]
    fn test_login() {
        // assert_eq!(login("admin", "password"), LoginAction::Admin);
        // assert_eq!(login("bob", "password"), LoginAction::User);
        // assert_eq!(login("admin", "wrongPassword"), LoginAction::Denied);
    }
}

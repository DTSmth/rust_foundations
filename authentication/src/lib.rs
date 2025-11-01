pub fn greet_user(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn login(username: &str, password: &str) -> bool {
    username.to_lowercase() == "admin" && password == "password"
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
        assert!(login("admin", "password"));
        assert!(!login("admi", "password"));
        assert!(!login("admin", "passsword"));
        assert!(login("ADMIN", "password"));
    }
}

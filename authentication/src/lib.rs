pub fn greet_user(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
    let username = username.to_string();
    if let Some(user) = users.iter().find(|user| user.username == username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()))
        } else {
            return Some(LoginAction::Denied)
        }
    }
    None
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}


#[derive(PartialEq, Debug, Clone)]
pub enum LoginRole {
    Admin,
    User,
}

pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

pub fn get_users() -> [User; 2] {
    [
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User),
    ]
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

use std::io::ErrorKind;
use std::path::Path;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsersFound,
    #[error("Too many users found")]
    TooManyUsersFound,
}

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("my_file.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let content = maybe_read_a_file()?;
    Ok(content.to_uppercase())
}

#[derive(Deserialize)]
struct User {
    username: String,
}

type GenericType<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_user() -> Result<Vec<User>, UsersError> {
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path).map_err(|e| UsersError::NoUsersFound)?;
    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|e| UsersError::NoUsersFound)?;
    Ok(users)
}

fn main() {
    if let Ok(content) = file_to_uppercase() {
        println!("{}", content);
    }

    let my_file = Path::new("myfile.txt");
    let contents = std::fs::read_to_string(my_file);
    match contents {
        Ok(contents) => println!("Contents: {}", contents),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => println!("File not found!"),
            _ => println!("Other Error: {}", err),
        },
    }
}

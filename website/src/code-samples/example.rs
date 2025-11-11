use std::fmt;
use std::result::Result;

/// Represents different types of HTTP methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

/// Custom error types for our application
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    InvalidInput { field: String, message: String },
    DatabaseError(String),
    Unauthorized,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::InvalidInput { field, message } => {
                write!(f, "Invalid input for {}: {}", field, message)
            }
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::Unauthorized => write!(f, "Unauthorized access"),
        }
    }
}

/// A trait for serializable types
pub trait Serializable {
    fn serialize(&self) -> String;
    fn deserialize(data: &str) -> Result<Self, AppError>
    where
        Self: Sized;
}

/// User struct with lifetime annotations
#[derive(Debug, Clone)]
pub struct User<'a> {
    pub id: u64,
    pub username: &'a str,
    pub email: &'a str,
    pub age: Option<u8>,
    pub is_active: bool,
}

impl<'a> User<'a> {
    /// Creates a new user with validation
    pub fn new(id: u64, username: &'a str, email: &'a str) -> Result<Self, AppError> {
        if username.is_empty() {
            return Err(AppError::InvalidInput {
                field: "username".to_string(),
                message: "Username cannot be empty".to_string(),
            });
        }

        if !email.contains('@') {
            return Err(AppError::InvalidInput {
                field: "email".to_string(),
                message: "Invalid email format".to_string(),
            });
        }

        Ok(User {
            id,
            username,
            email,
            age: None,
            is_active: true,
        })
    }

    /// Updates the user's age
    pub fn set_age(&mut self, age: u8) -> Result<(), AppError> {
        if age > 150 {
            return Err(AppError::InvalidInput {
                field: "age".to_string(),
                message: "Age must be realistic".to_string(),
            });
        }
        self.age = Some(age);
        Ok(())
    }
}

/// Generic repository trait with associated types
pub trait Repository<T> {
    type Error;
    type Id;

    fn find(&self, id: Self::Id) -> Result<T, Self::Error>;
    fn save(&mut self, item: T) -> Result<Self::Id, Self::Error>;
    fn delete(&mut self, id: Self::Id) -> Result<(), Self::Error>;
}

/// Process a request with pattern matching
pub fn process_request(method: HttpMethod, path: &str) -> Result<String, AppError> {
    match (method, path) {
        (HttpMethod::Get, "/users") => Ok("List all users".to_string()),
        (HttpMethod::Get, path) if path.starts_with("/users/") => {
            let id = path.strip_prefix("/users/").unwrap_or("0");
            Ok(format!("Get user with id: {}", id))
        }
        (HttpMethod::Post, "/users") => Ok("Create new user".to_string()),
        (HttpMethod::Put, path) if path.starts_with("/users/") => Ok("Update user".to_string()),
        (HttpMethod::Delete, path) if path.starts_with("/users/") => Ok("Delete user".to_string()),
        _ => Err(AppError::NotFound(format!(
            "Route not found: {:?} {}",
            method, path
        ))),
    }
}

/// Macro for creating a user quickly
#[macro_export]
macro_rules! create_user {
    ($id:expr, $username:expr, $email:expr) => {
        User::new($id, $username, $email)
    };
    ($id:expr, $username:expr, $email:expr, age: $age:expr) => {{
        let mut user = User::new($id, $username, $email)?;
        user.set_age($age)?;
        Ok(user)
    }};
}

fn main() -> Result<(), AppError> {
    // Create a new user
    let mut user = User::new(1, "alice", "alice@example.com")?;
    user.set_age(28)?;

    println!("Created user: {:?}", user);

    // Pattern matching with enums
    let methods = vec![
        HttpMethod::Get,
        HttpMethod::Post,
        HttpMethod::Put,
        HttpMethod::Delete,
    ];

    for method in methods {
        match method {
            HttpMethod::Get => println!("Handling GET request"),
            HttpMethod::Post => println!("Handling POST request"),
            HttpMethod::Put => println!("Handling PUT request"),
            HttpMethod::Delete => println!("Handling DELETE request"),
            HttpMethod::Patch => println!("Handling PATCH request"),
        }
    }

    // Process some requests
    let result = process_request(HttpMethod::Get, "/users")?;
    println!("Result: {}", result);

    // Option handling
    if let Some(age) = user.age {
        println!("User age: {}", age);
    } else {
        println!("Age not set");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(1, "test", "test@example.com");
        assert!(user.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let user = User::new(1, "test", "invalid-email");
        assert!(user.is_err());
    }

    #[test]
    fn test_age_validation() {
        let mut user = User::new(1, "test", "test@example.com").unwrap();
        assert!(user.set_age(200).is_err());
        assert!(user.set_age(25).is_ok());
    }
}

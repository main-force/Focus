pub mod manager;

use std::fmt;


#[derive(Debug)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> User {
        User {
            name: String::from(name),
        }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::cmp::PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
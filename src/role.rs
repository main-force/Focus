pub mod manager;

use std::fmt;


#[derive(Debug)]
pub struct Role {
    name: String,
}

impl Role {
    pub fn new(name: &str) -> Role {
        Role {
            name: String::from(name),
        }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::cmp::PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
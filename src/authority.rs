pub mod manager;

use std::fmt;


#[derive(Debug)]
pub struct Authority {
    name: String,
}

impl Authority {
    pub fn new(name: &str) -> Authority {
        Authority {
            name: String::from(name),
        }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Authority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::cmp::PartialEq for Authority {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


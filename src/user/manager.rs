use crate::user::User;
use crate::file_manager::get_user_list_file_path;
use std::fs::File;
use std::error::Error;
use csv::{ Reader, Writer };

pub struct UserList {
    state: Option<Box<dyn State>>,
    user_list: Vec<User>,
}

trait State {
    fn synchronize_with_file(self: Box<Self>, user_list: &mut UserList) -> Box<dyn State>;
}

struct Synchronized {}

impl State for Synchronized {
    fn synchronize_with_file(self: Box<Self>, _user_list: &mut UserList) -> Box<dyn State> {
        self
    }
}

struct Unsynchronized {}

impl State for Unsynchronized {
    fn synchronize_with_file(self: Box<Self>, user_list: &mut UserList) -> Box<dyn State> {
        match write_user_list_in_file(user_list) {
            Err(_) => {
                println!("Cant write user list in file.");
                self
            },
            Ok(_) => {
                println!("Write user list complete.");
                Box::new(Synchronized {})
            }
        }
    }
}

impl UserList {
    pub fn new() -> UserList { 
        UserList{
            state: Some(Box::new(Synchronized {})),
            user_list: get_user_list_from_file(),
        }
    }
    
    pub fn is_exist(&self, name: &str) -> bool {
        self.user_list.contains(&User::new(name)) 
    }
    
    pub fn add(&mut self, user: User) -> bool {
        if self.is_exist(user.get_name()) {
            false
        } else {
            self.user_list.push(user);
            self.state = Some(Box::new(Unsynchronized {}));
            true
        }
    }
    
    pub fn remove(&mut self, user_name: &str) -> bool {
        match self.user_list.iter().position(|v| v.get_name() == user_name) {
            Some(index) => { 
                self.user_list.remove(index);
                self.state = Some(Box::new(Unsynchronized {}));
                true
            },
            None => false,
        }
    }
    
    pub fn get_user_list(&self) -> &Vec<User> {
        &self.user_list      
    }
    
    pub fn get_users_name(&self) -> Vec<&str> {
        self.user_list
            .iter()
            .map(|user| user.get_name())
            .collect()
    }
    
    pub fn synchronize_with_file(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.synchronize_with_file(self));
        }
    }
    
}

fn get_user_list_from_file() -> Vec<User> {
    let mut rdr = match Reader::from_path(get_user_list_file_path()) {
        Ok(reader) => reader,
        Err(_) => panic!("can't get user list from the path"),
    };
    
    let mut user_list = Vec::new();
    
    for result in rdr.records() {
        let record = match result {
            Ok(record) => record,
            Err(_) => panic!("Can't read user list because of data's form.")
        };
        
        for field in &record {
            user_list.push(User::new(field));
        }
    }
    
    user_list
}


//---------------------------------------------------------------------------

fn write_user_list_in_file(user_list: &mut UserList) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(get_user_list_file_path())?;
    write_header_in_file(&mut wtr)?;
    write_records_in_file(&mut wtr, user_list.get_users_name())?;

    //Todo: have to return the user.get_info to iterable type.
    //vec!(something) is very bad...
    
    Ok(())
}

fn write_header_in_file(wtr: &mut Writer<File>) -> Result<(), Box<dyn Error>> {
    wtr.write_record(&["Name"])?;
    Ok(())
}

fn write_records_in_file(wtr: &mut Writer<File>, records: Vec<&str>) -> Result<(), Box<dyn Error>> {
    for record in records.iter() {
        wtr.write_record(vec![record])?;
    }
    
    Ok(())
}

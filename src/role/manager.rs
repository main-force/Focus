use crate::role::Role;
use crate::file_manager::get_role_list_file_path;
use std::fs::File;
use std::error::Error;
use csv::{ Reader, Writer };

pub struct RoleList {
    state: Option<Box<dyn State>>,
    role_list: Vec<Role>,
}

trait State {
    fn synchronize_with_file(self: Box<Self>, role_list: &mut RoleList) -> Box<dyn State>;
}

struct Synchronized {}

impl State for Synchronized {
    fn synchronize_with_file(self: Box<Self>, _role_list: &mut RoleList) -> Box<dyn State> {
        self
    }
}

struct Unsynchronized {}

impl State for Unsynchronized {
    fn synchronize_with_file(self: Box<Self>, role_list: &mut RoleList) -> Box<dyn State> {
        match write_role_list_in_file(role_list) {
            Err(_) => {
                println!("Cant write role list in file.");
                self
            },
            Ok(_) => {
                println!("Write role list complete.");
                Box::new(Synchronized {})
            }
        }
    }
}

impl RoleList {
    pub fn new() -> RoleList { 
        RoleList{
            state: Some(Box::new(Synchronized {})),
            role_list: get_role_list_from_file(),
        }
    }
    
    pub fn is_exist(&self, name: &str) -> bool {
        self.role_list.contains(&Role::new(name)) 
    }
    
    pub fn add(&mut self, role: Role) -> bool {
        if self.is_exist(role.get_name()) {
            false
        } else {
            self.role_list.push(role);
            self.state = Some(Box::new(Unsynchronized {}));
            true
        }
    }
    
    pub fn remove(&mut self, role_name: &str) -> bool {
        match self.role_list.iter().position(|v| v.get_name() == role_name) {
            Some(index) => { 
                self.role_list.remove(index);
                self.state = Some(Box::new(Unsynchronized {}));
                true
            },
            None => false,
        }
    }
    
    pub fn get_role_list(&self) -> &Vec<Role> {
        &self.role_list      
    }
    
    pub fn get_roles_name(&self) -> Vec<&str> {
        self.role_list
            .iter()
            .map(|role| role.get_name())
            .collect()
    }
    
    pub fn synchronize_with_file(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.synchronize_with_file(self));
        }
    }
    
}

fn get_role_list_from_file() -> Vec<Role> {
    let mut rdr = match Reader::from_path(get_role_list_file_path()) {
        Ok(reader) => reader,
        Err(_) => panic!("can't get role list from the path"),
    };
    
    let mut role_list = Vec::new();
    
    for result in rdr.records() {
        let record = match result {
            Ok(record) => record,
            Err(_) => panic!("Can't read role list because of data's form.")
        };
        
        for field in &record {
            role_list.push(Role::new(field));
        }
    }
    
    role_list
}


//---------------------------------------------------------------------------

fn write_role_list_in_file(role_list: &mut RoleList) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(get_role_list_file_path())?;
    write_header_in_file(&mut wtr)?;
    write_records_in_file(&mut wtr, role_list.get_roles_name())?;

    //Todo: have to return the role.get_info to iterable type.
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

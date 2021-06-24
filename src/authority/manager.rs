use crate::authority::Authority;
use crate::file_manager::get_authority_list_file_path;
use std::fs::File;
use std::error::Error;
use csv::{ Reader, Writer };

pub struct AuthorityList {
    state: Option<Box<dyn State>>,
    authority_list: Vec<Authority>,
}

trait State {
    fn synchronize_with_file(self: Box<Self>, authority_list: &mut AuthorityList) -> Box<dyn State>;
}

struct Synchronized {}

impl State for Synchronized {
    fn synchronize_with_file(self: Box<Self>, _authority_list: &mut AuthorityList) -> Box<dyn State> {
        self
    }
}

struct Unsynchronized {}

impl State for Unsynchronized {
    fn synchronize_with_file(self: Box<Self>, authority_list: &mut AuthorityList) -> Box<dyn State> {
        match write_authority_list_in_file(authority_list) {
            Err(_) => {
                println!("Cant write authority list in file.");
                self
            },
            Ok(_) => {
                println!("Write authority list complete.");
                Box::new(Synchronized {})
            }
        }
    }
}

impl AuthorityList {
    pub fn new() -> AuthorityList { 
        AuthorityList{
            state: Some(Box::new(Synchronized {})),
            authority_list: get_authority_list_from_file(),
        }
    }
    
    pub fn is_exist(&self, name: &str) -> bool {
        self.authority_list.contains(&Authority::new(name)) 
    }
    
    pub fn add(&mut self, auth: Authority) -> bool {
        if self.is_exist(auth.get_name()) {
            false
        } else {
            self.authority_list.push(auth);
            self.state = Some(Box::new(Unsynchronized {}));
            true
        }
    }
    
    pub fn remove(&mut self, auth_name: &str) -> bool {
        match self.authority_list.iter().position(|v| v.get_name() == auth_name) {
            Some(index) => { 
                self.authority_list.remove(index);
                self.state = Some(Box::new(Unsynchronized {}));
                true
            },
            None => false,
        }
    }
    
    pub fn get_authority_list(&self) -> &Vec<Authority> {
        &self.authority_list      
    }
    
    pub fn get_authorities_name(&self) -> Vec<&str> {
        self.authority_list
            .iter()
            .map(|authority| authority.get_name())
            .collect()
    }
    
    pub fn synchronize_with_file(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.synchronize_with_file(self));
        }
    }
    
}

fn get_authority_list_from_file() -> Vec<Authority> {
    let mut rdr = match Reader::from_path(get_authority_list_file_path()) {
        Ok(reader) => reader,
        Err(_) => panic!("can't get authority list from the path"),
    };
    
    let mut authority_list = Vec::new();
    
    for result in rdr.records() {
        let record = match result {
            Ok(record) => record,
            Err(_) => panic!("Can't read authority list because of data's form.")
        };
        
        for field in &record {
            authority_list.push(Authority::new(field));
        }
    }
    
    authority_list
}


//---------------------------------------------------------------------------

fn write_authority_list_in_file(authority_list: &mut AuthorityList) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(get_authority_list_file_path())?;
    write_header_in_file(&mut wtr)?;
    write_records_in_file(&mut wtr, authority_list.get_authorities_name())?;

    //Todo: have to return the auth.get_info to iterable type.
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

//------------------------------------------------------------------------------


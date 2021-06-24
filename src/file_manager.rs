use std::path::{Path, PathBuf};
use std::error::Error;
use std::env;
use csv::{WriterBuilder};

static AUTHORITY_LIST_FILE: &str = "authority_list.csv";
static ROLE_LIST_FILE: &str = "role_list.csv";
static USER_LIST_FILE: &str = "user_list.csv";

pub fn get_root_dir_path() -> PathBuf {
    let root_dir = match env::current_dir() {
        Ok(r) => r,
        Err(..) => panic!("Can't read current_dir"),
    };
    
    root_dir
}

//---------------------------------------------------------
pub fn check_authority_list_file() {
    if Path::is_file(get_authority_list_file_path().as_path()) == false {
        if let Err(..) = create_authority_list_file() {
            panic!("can't create authority_list_file.");
        }
    }
}

pub fn check_role_list_file() {
    if Path::is_file(get_role_list_file_path().as_path()) == false {
        if let Err(..) = create_role_list_file() {
            panic!("can't create role_list_file.");
        }
    }
}

pub fn check_user_list_file() {
    if Path::is_file(get_user_list_file_path().as_path()) == false {
        if let Err(..) = create_user_list_file() {
            panic!("can't create user_list_file.");
        }
    }
}

pub fn get_authority_list_file_path() -> PathBuf {
    get_root_dir_path()
        .join(PathBuf::from(AUTHORITY_LIST_FILE))
}

pub fn get_role_list_file_path() -> PathBuf {
    get_root_dir_path()
        .join(PathBuf::from(ROLE_LIST_FILE))
}

pub fn get_user_list_file_path() -> PathBuf {
    get_root_dir_path()
        .join(PathBuf::from(USER_LIST_FILE))
}

fn create_authority_list_file() -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().from_path(get_authority_list_file_path())?;
    wtr.write_record(&["Name"])?;
    
    Ok(())
}

fn create_role_list_file() -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().from_path(get_role_list_file_path())?;
    wtr.write_record(&["Name"])?;
    
    Ok(())
}

fn create_user_list_file() -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().from_path(get_user_list_file_path())?;
    wtr.write_record(&["Name"])?;
    
    Ok(())
}




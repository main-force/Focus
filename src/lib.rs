mod file_manager;
pub mod authority;
pub mod role;
pub mod user;

pub mod post;

use post::Text;

use authority::Authority;
use authority::manager::AuthorityList;

use role::Role;
use role::manager::RoleList;

use user::User;
use user::manager::UserList;

use std::error::Error;

pub mod init {
    use crate::file_manager;
    
    pub fn file_check() {
        file_manager::check_authority_list_file();
        file_manager::check_role_list_file();
        file_manager::check_user_list_file();
    }

}

//Todo: Implement to process user input

pub fn run() -> std::result::Result<(), Box<dyn Error>> {
    //Check Authority list
    {
        let mut authority_list = AuthorityList::new();
    
        let name = "Pay Manage";
        let pay_manage = Authority::new(name);
        if !authority_list.add(pay_manage) {
            println!("Already exist: {}", name);
        } else {
            authority_list.synchronize_with_file();
        }

        let name = "Approve Request";
        if !authority_list.add(Authority::new(name)) {
            println!("Already exist: {}", name);
        } else {
            authority_list.synchronize_with_file();
        }

        print!("\n");

        println!("Before remove Authority List");
        for e in authority_list.get_authority_list().iter() {
            println!("{}", e);    
        }

        println!("After not remove Authority List");
        for e in authority_list.get_authority_list().iter() {
            println!("{}", e);    
        }

        authority_list.synchronize_with_file();
    }
    
    //Check role_list.
    {
        let mut role_list = RoleList::new();
        
        let name = "Director";
        let director = Role::new(name);
        
        if !role_list.add(director) {
            println!("Already exist: {}", name);
        } else {
            role_list.synchronize_with_file();
        }

        let name = "Accounter";
        if !role_list.add(Role::new(name)) {
            println!("Already exist: {}", name);
        } else {
            role_list.synchronize_with_file();
        }

        print!("\n");

        println!("Before remove Role List");
        for e in role_list.get_role_list().iter() {
            println!("{}", e);    
        }

        println!("After not remove Role List");
        for e in role_list.get_role_list().iter() {
            println!("{}", e);    
        }

        role_list.synchronize_with_file();
        
    }
    
    // Check User list
    {
        let mut user_list = UserList::new();
        
        let name = "MainForce";
        let developer = User::new(name);
        
        if !user_list.add(developer) {
            println!("Already exist: {}", name);
        } else {
            user_list.synchronize_with_file();
        }

        let name = "dylee";
        if !user_list.add(User::new(name)) {
            println!("Already exist: {}", name);
        } else {
            user_list.synchronize_with_file();
        }

        print!("\n");

        println!("Before remove User List");
        for e in user_list.get_user_list().iter() {
            println!("{}", e);    
        }

        println!("After not remove User List");
        for e in user_list.get_user_list().iter() {
            println!("{}", e);    
        }

        user_list.synchronize_with_file();
        
    }
    
    Ok(())
}
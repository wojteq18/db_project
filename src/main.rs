mod db;  // Importujemy moduł db.rs
mod models;  // Importujemy moduł models.rs

use mysql::*;
use mysql::prelude::*;
use db::connect_to_db;
use models::user::User; 
use models::city::City;
use models::country::Country;
use bcrypt::{verify};

fn main() -> Result<()> {
    let pool = connect_to_db()?;
    let mut conn = pool.get_conn()?;
    /*let mut is_logged: bool = false;
    let mut is_admin: bool;
    let mut is_end: bool = false;

    while is_logged == false {
        println!("You must be logged in to use the application");
        println!("1: Log in");
        println!("2: Register");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: i32 = choice.trim().parse().expect("Please type a number!");

        match choice {
            1 => {
                println!("Enter your login: ");
                let mut login = String::new();
                std::io::stdin().read_line(&mut login).expect("Failed to read line");
        
                println!("Enter your password: ");
                let mut password = String::new();
                std::io::stdin().read_line(&mut password).expect("Failed to read line");
                
                if User::user_exists(&mut conn, &login) {
                    // Weryfikacja hasła
                    match verify(&password, &stored_user.password) {
                        Ok(true) => {
                            println!("Login successful! Welcome, {}", login);
                            is_logged = true;
                            is_admin = stored_user.status == "admin";
                        },
                        Ok(false) => println!("Invalid password. Try again."),
                        Err(_) => println!("Error verifying password."),
                    }
                } else {
                    println!("User not found. Please register.");
                }
            },  // <-- Domykamy blok dla opcji 1
        
            2 => {
                println!("Enter your login: ");
                let mut login = String::new();
                std::io::stdin().read_line(&mut login).expect("Failed to read line");
                
                println!("Enter your password: ");
                let mut password = String::new();
                std::io::stdin().read_line(&mut password).expect("Failed to read line");
        
                let does_exists = User::user_exists(&mut conn, &login);
                if does_exists == false {
                    let user = User::new(&login, &password);
                    user.add_user(&mut conn);
                    is_logged = true;
                } else {
                    println!("User already exists");
                    continue;
                }
            }
        }        
    }

    
    
    while is_end == false {
        let mut choice = String::new();
        println!("Pick what you want to do: ");
        println!("1. Add user");
        println!("2. Remove user");
        println!("3. Change user's status");
        println!("4. Add city");
        println!("5. Remove city");
        println!("6. Add country");
        println!("7. Remove country");
        println!("8. Add city transport");
        println!("9. Remove city transport");
        println!("10. End program");
    } */
    Ok(())
}
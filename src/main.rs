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
    let mut is_logged: bool = false;
    let mut is_admin: bool = false;

    while is_logged == false {
        println!("To use application, you have to be logged");
        println!("Choose number of action:");
        println!("1. Log in");
        println!("2. Register");

        let mut action = String::new();
        std::io::stdin().read_line(&mut action).expect("Failed to read line");
        let action: i32 = action.trim().parse().expect("Please type a number!");

        match action {
            1 => { 
                println!("Type your login: ");
                let mut login = String::new();
                std::io::stdin().read_line(&mut login).expect("Failed to read line");
                let login: String = login.trim().parse().expect("Please type a string!");

                println!("Type your password: ");
                let mut password = String::new();
                std::io::stdin().read_line(&mut password).expect("Failed to read line");
                let password: String = password.trim().parse().expect("Please type a string!");

                if User::user_exists(&mut conn, &login) == true {
                    // Pobierz dane użytkownika
                    let query = "SELECT password, status FROM user WHERE login = :login";
                    let result: Option<(String, String)> = conn.exec_first(
                        query,
                        params! {
                            "login" => &login
                        },
                    )?;
        
                    match result {
                        Some((hashed_password, status)) => {
                            // Weryfikuj hasło
                            if bcrypt::verify(&password, &hashed_password).unwrap_or(false) {
                                println!("Login successful!");
                                is_logged = true;
        
                                if status == "admin" {
                                    println!("Welcome, admin!");
                                    is_admin = true;
                                } else {
                                    println!("Welcome, regular user!");
                                }
                            } else {
                                println!("Invalid password. Please try again.");
                            }
                        },
                        None => {
                            println!("No user found with the provided login.");
                        }
                    }
                } else {
                    println!("User '{}' does not exist.", login);
                }
            },

            2 => {
                println!("Type your login: ");
                let mut login = String::new();
                std::io::stdin().read_line(&mut login).expect("Failed to read line");
                let login: String = login.trim().to_string();

                println!("Type your password: ");
                let mut password = String::new();
                std::io::stdin().read_line(&mut password).expect("Failed to read line");
                let password: String = password.trim().to_string();

                if User::user_exists(&mut conn, &login) == true {
                    println!("User '{}' already exists!", login);
                } else {
                    let user = User::new(&login, &password);
                    user.add_user(&mut conn)?;
                    is_logged = true;
                }
            },
            _ => println!("Please type a number from 1 to 2")
        }

    }
    Ok(())
}
mod db;  // Importujemy moduł db.rs
mod models;  // Importujemy moduł models.rs
mod possibilities; //importujemy moduł odpowiedzialny za mozliwosci admina i normal usera
mod auth; //importujemy moduł odpowiedzialny za logowanie

use mysql::*;
use mysql::prelude::*;
use db::connect_to_db;
use models::{city_transport::City_transport, user::User}; 
use models::city::City;
use models::country::Country;
use models::hotel::Hotel;
use bcrypt::{verify};
use possibilities::user_actions::{UserActions, NormalUser};
use possibilities::admin_actions::{AdminActions, Admin};
use auth::{Auth, AuthService};

fn main() -> Result<()> {
    let pool = connect_to_db()?;
    let mut conn = pool.get_conn()?;
    let mut is_logged = false;
    let mut is_admin = false;

    while !is_logged {
        println!("To use the application, you have to be logged in");
        println!("Choose the number of an action:");
        println!("1. Log in");
        println!("2. Register");

        let mut action = String::new();
        std::io::stdin().read_line(&mut action).expect("Failed to read line");
        let action: i32 = action.trim().parse().expect("Please type a number!");

        match action {
            1 => {
                let auth_status = AuthService::log_in(&mut conn);
                is_admin = auth_status.is_admin;
                is_logged = auth_status.is_logged;
            }
            2 => AuthService::register(&mut conn),
            _ => println!("Please type a number from 1 to 2"),
        }
    }

    let mut is_ended = false;

    while !is_ended {
        if !is_admin {
            // Opcje dla zwykłego użytkownika
            println!("Choose the number of an action:");
            println!("1. Show connections between cities");
            println!("2. Show hotels in a city");
            println!("3. Log out");

            let mut action = String::new();
            std::io::stdin().read_line(&mut action).expect("Failed to read line");
            let action: i32 = action.trim().parse().expect("Please type a number!");

            match action {
                1 => NormalUser::show_connections(&mut conn),
                2 => NormalUser::show_hotels(&mut conn),
                3 => {
                    is_ended = true;
                    println!("Logged out successfully.");
                }
                _ => println!("Please type a number from 1 to 3"),
            }
        } else {
            // Opcje dla administratora
            println!("Choose the number of an action:");
            println!("1. Add city");
            println!("2. Remove city");
            println!("3. Add country");
            println!("4. Remove country");
            println!("5. Add hotel");
            println!("6. Remove hotel");
            println!("7. Log out");

            let mut action = String::new();
            std::io::stdin().read_line(&mut action).expect("Failed to read line");
            let action: i32 = action.trim().parse().expect("Please type a number!");

            match action {
                1 => Admin::add_city(&mut conn),
                2 => Admin::remove_city(&mut conn),
                3 => Admin::add_country(&mut conn),
                4 => Admin::remove_country(&mut conn),
                5 => Admin::add_hotel(&mut conn),
                6 => Admin::remove_hotel(&mut conn),
                7 => {
                    is_ended = true;
                    println!("Logged out successfully.");
                }
                _ => println!("Please type a number from 1 to 7"),
            }
        }
    }

    Ok(())
}

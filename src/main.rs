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
use auth::{Auth, AuthService};

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
                let auth_status = AuthService::log_in(&mut conn);
                is_admin = auth_status.is_admin;
                is_logged = auth_status.is_logged
            }
            2 => AuthService::register(&mut conn),
            _ => println!("Please type a number from 1 to 2")
        }
    }

    if is_admin == false {
        let mut is_ended: bool = false;

        while is_ended == false {
            println!("Choose number of action: ");
            println!("1. Show connection beetwen cities");
            println!("2. Show hotels in city");
            println!("3. Log out");

            let mut action = String::new();
            std::io::stdin().read_line(&mut action).expect("Failed to read line");
            let action: i32 = action.trim().parse().expect("Please type a number!");

            match action {
                1 => NormalUser::show_connections(&mut conn),
                2 => NormalUser::show_hotels(&mut conn),
                3 => {
                    is_ended = true;
                }
                _ => println!("Please type number from 1 to 3")
            }
        }
    }
    Ok(())
}
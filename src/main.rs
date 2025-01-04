mod db;  // Importujemy moduł db.rs
mod models;  // Importujemy moduł models.rs

use mysql::*;
use mysql::prelude::*;
use db::connect_to_db;
use models::user::User; 
use models::city::City;
use models::country::Country;

fn main() -> Result<()> {
    let pool = connect_to_db()?;
    let mut conn = pool.get_conn()?;
    let mut isEnd: bool = false;

    while isEnd == false {
        println!("1. Add user");
        println!("2. Add city");
        println!("3. Add country");
        println!("4. Exit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number!");

        match choice {
            1 => {
                let user = User::new("admin", "admin", "active");
                user.add_user(&mut conn)?;
            },
            2 => {
                let city = City::new("Seoul", 1);
                city.add_city(&mut conn)?;
            },
            3 => {
                let country = Country::new("South Korea", "SK");
                country.add_country(&mut conn)?;
            },
            4 => {
                isEnd = true;
            },
            _ => {
                println!("Invalid choice");
            }
        }
    }


    let country = Country::new("South Korea", "SK");
    country.add_country(&mut conn)?;
    Ok(())

}

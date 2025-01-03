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

    /*let new_user = User::new(
        "Marian",
        "Kowalski",
        "mariankowalski@gmail.com",
        "password123"
    );*/

    //new_user.add_user(&mut conn)?;
    Ok(())
}

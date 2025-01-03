mod db;  // Importujemy moduł db.rs
mod models;  // Importujemy moduł models.rs

use mysql::*;
use mysql::prelude::*;
use db::connect_to_db;
use models::user::User; 

fn main() -> Result<()> {
    let pool = connect_to_db()?;
    let mut conn = pool.get_conn()?;

    let new_user = User::new(
        "Jan",
        "Kowalski",
        "jankowalski@gmail.com",
        "password"
    );

    new_user.add_user(&mut conn)?;
    Ok(())
}

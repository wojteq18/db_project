use mysql::params;
use mysql::PooledConn;
use mysql::prelude::Queryable;

#[derive(Debug, PartialEq, Eq)]
pub struct City {
    pub city_id: i32,
    pub name: String,
    pub country_id: i32,
}

impl City {
    // Konstruktor nowego miasta
    pub fn new(name: &str, country_id: i32) -> Self {
        City {
            city_id: 0,
            name: name.to_owned(),
            country_id,
        }
    }

    // Sprawdzenie, czy miasto istnieje w bazie
    pub fn city_exists(conn: &mut PooledConn, name: &str, country_id: i32) -> bool {
        let exists: Option<String> = conn.exec_first(
            "SELECT name FROM city WHERE name = :name AND country_id = :country_id",
            params! {
                "name" => name,
                "country_id" => country_id,
            },
        ).unwrap_or(None);

        exists.is_some()
    }

    // Dodanie miasta, jeśli nie istnieje
    pub fn add_city(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        if Self::city_exists(conn, &self.name, self.country_id) == false {
            conn.exec_drop(
                r"INSERT INTO city (name, country_id)
                VALUES (:name, :country_id)",
                params! {
                    "name" => &self.name,
                    "country_id" => &self.country_id,
                }
            )?;
            println!("City '{}' added successfully!", &self.name);
        } else {
            println!("City '{}' already exists!", &self.name);
        }
        Ok(())
    }    

    // Usunięcie miasta, jeśli istnieje
    pub fn remove_city(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        if Self::city_exists(conn, &self.name, self.country_id) == true {
            conn.exec_drop(
                r"DELETE FROM city WHERE name = :name AND country_id = :country_id",
                params! {
                    "name" => &self.name,
                    "country_id" => &self.country_id,
                }
            )?;
            println!("City '{}' removed successfully!", &self.name);
        } else {
            println!("City '{}' does not exist.", &self.name);
        }
        Ok(())
    }
}

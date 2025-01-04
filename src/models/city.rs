use mysql::params;
use mysql::PooledConn;
use mysql::prelude::Queryable;

#[derive(Debug, PartialEq, Eq)]
pub struct City {
    pub city_id: i32,
    pub name: String,
    pub country_name: String,
}

impl City {
    // Konstruktor nowego miasta
    pub fn new(name: &str, country_name: &str) -> Self {
        City {
            city_id: 0,
            name: name.to_owned(),
            country_name: country_name.to_owned(),
        }
    }

    // Sprawdzenie, czy miasto istnieje w bazie
    pub fn city_exists(conn: &mut PooledConn, name: &str, country_name: &str) -> bool {
        let exists: Option<String> = conn.exec_first(
            r"SELECT c.name 
            FROM city c
            JOIN country co ON c.country_id = co.country_id
            WHERE c.name = :name AND co.name = :country_name",
            params! {
                "name" => name,
                "country_name" => country_name,
            },
        ).unwrap_or(None);

        exists.is_some()
    }

    // Znalezienie country_id na podstawie nazwy kraju
    pub fn get_country_id(conn: &mut PooledConn, country_name: &str) -> Option<i32> {
        conn.exec_first(
            "SELECT country_id FROM country WHERE name = :country_name",
            params! {
                "country_name" => country_name,
            },
        ).unwrap_or(None)
    }

    // Dodanie miasta, jeśli nie istnieje
    pub fn add_city(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        if !Self::city_exists(conn, &self.name, &self.country_name) {
            if let Some(country_id) = Self::get_country_id(conn, &self.country_name) {
                conn.exec_drop(
                    r"INSERT INTO city (name, country_id)
                    VALUES (:name, :country_id)",
                    params! {
                        "name" => &self.name,
                        "country_id" => country_id,
                    }
                )?;
                println!("City '{}' added successfully in '{}'.", &self.name, &self.country_name);
            } else {
                println!("Country '{}' does not exist.", &self.country_name);
            }
        } else {
            println!("City '{}' already exists in '{}'.", &self.name, &self.country_name);
        }
        Ok(())
    }

    // Usunięcie miasta, jeśli istnieje
    pub fn remove_city(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        if Self::city_exists(conn, &self.name, &self.country_name) {
            conn.exec_drop(
                r"DELETE c FROM city c
                JOIN country co ON c.country_id = co.country_id
                WHERE c.name = :name AND co.name = :country_name",
                params! {
                    "name" => &self.name,
                    "country_name" => &self.country_name,
                }
            )?;
            println!("City '{}' removed successfully from '{}'.", &self.name, &self.country_name);
        } else {
            println!("City '{}' does not exist in '{}'.", &self.name, &self.country_name);
        }
        Ok(())
    }
}

use mysql::params;
use mysql::PooledConn;
use mysql::prelude::Queryable;

#[derive(Debug, PartialEq, Eq)]
pub struct Country {
    pub country_id: i32,
    pub name: String,
    pub code: String,
}

impl Country {
    pub fn country_exists(conn: &mut PooledConn, name: &str) -> bool {
        let exists: Option<String> = conn.exec_first(
            "SELECT name FROM country WHERE name = :name",
            params! {
                "name" => name,
            },
        ).unwrap_or(None);
        exists.is_some()
    }

    pub fn new(name: &str, code: &str) -> Self {
        Country {
            country_id: 0, // Domyślnie 0, do nadpisania później
            name: name.to_owned(),
            code: code.to_owned(),
        }
    }

    pub fn add_country(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        if Self::country_exists(conn, &self.name) == false {
            conn.exec_drop(
                r"INSERT INTO country (name, code)
                VALUES (:name, :code)",
                params! {
                    "name" => &self.name,
                    "code" => &self.code,
                }
            )?;
            println!("Country '{}' added successfully!", &self.name);
        } else {
            println!("Country '{}' already exists!", &self.name);
        }
        Ok(())
    }  

    pub fn remove_country(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        if Self::country_exists(conn, &self.name) == true {
            conn.exec_drop(
                r"DELETE FROM country WHERE name = :name AND code = :code",
                params! {
                    "name" => &self.name,
                    "code" => &self.code,
                }
            )?;
            println!("Country '{}' removed successfully!", &self.name);
        } else {
            println!("Country '{}' does not exist!", &self.name);
        }
     Ok(())
    }
}

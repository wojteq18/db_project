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
    pub fn new(name: &str, country_id: i32) -> Self {
        City {
            city_id: 0,  // Autoinkrementacja, zostanie nadpisane po dodaniu do bazy
            name: name.to_owned(),
            country_id,
        }
    }

    pub fn add_city(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        let exists: Option<String> = conn.exec_first(
            "SELECT name FROM city WHERE name = :name AND country_id = :country_id",
            params! {
                "name" => &self.name,
                "country_id" => &self.country_id,
            },
        )?;
    
        if exists.is_none() {
            conn.exec_drop(
                r"INSERT INTO city (name, country_id)
                VALUES (:name, :country_id)",
                params! {
                    "name" => &self.name,
                    "country_id" => &self.country_id,
                }
            )?;
        } else {
            println!("City already exists!");
        }
        Ok(())
    }    

    pub fn remove_city(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        conn.exec_drop(
            r"DELETE FROM city WHERE name = :name AND country_id = :country_id",
            params! {
                "name" => &self.name,
                "country_id" => &self.country_id,
            }
        )?;
        Ok(())
    }
}

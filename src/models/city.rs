use mysql::params;
use mysql::PooledConn;
use mysql::prelude::Queryable;

#[derive(Debug, PartialEq, Eq)]

pub struct City {
    pub city_id: i32,
    pub city_name: String,
    pub country_id: i32,
}

impl City {
    pub fn new(city_name: &str) -> Self {
        City {
            city_id: 0, //domyslnie 0, do nadpisania pozniej
            city_name: city_name.to_owned(),
            country_id: 0, //domyslnie 0, do nadpisania pozniej
        }
    }

    pub fn add_city(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        let exists: Option<String> = conn.exec_first(
            "SELECT city_name FROM city WHERE city_name = :city_name AND country_id = :country_id",
            params! {
                "city_name" => &self.city_name,
                "country_id" => &self.country_id,
            },
        )?;
    
        if exists.is_none() {
            conn.exec_drop(
                r"INSERT INTO city (city_name, country_id)
                VALUES (:city_name, :country_id)",
                params! {
                    "city_name" => &self.city_name,
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
            r"DELETE FROM city WHERE city_name = :city_name AND country_id = :country_id",
            params! {
                "city_name" => &self.city_name,
                "country_id" => &self.country_id,
            }
        )?;
        Ok(())
    }
}
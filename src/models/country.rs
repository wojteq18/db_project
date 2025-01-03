use mysql::params;
use mysql::PooledConn;
use mysql::prelude::Queryable;

#[derive(Debug, PartialEq, Eq)]
pub struct Country {
    pub country_id: i32,
    pub country_name: String,
}

impl Country {
    pub fn new(country_name: &str) -> Self {
        Country {
            country_id: 0, //domyslnie 0, do nadpisania pozniej
            country_name: country_name.to_owned(),
        }
    }

    pub fn add_country(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        let exists: Option<String> = conn.exec_first(
            "SELECT city_name FROM city WHERE city_name = :city_name AND country_id = :country_id",
            params! {
                "country_name" => &self.country_name,
                "country_id" => &self.country_id,
            },
        )?;
    
        if exists.is_none() {
            conn.exec_drop(
                r"INSERT INTO city (city_name, country_id)
                VALUES (:city_name, :country_id)",
                params! {
                    "city_name" => &self.country_name,
                    "country_id" => &self.country_id,
                }
            )?;
        } else {
            println!("Country already exists!");
        }
        Ok(())
    }  

    pub fn remove_country(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        conn.exec_drop(
            r"DELETE FROM country WHERE country_name = :country_name AND country_id = :country_id",
            params! {
                "country_name" => &self.country_name,
                "country_id" => &self.country_id,
            }
        )?;
        Ok(())
    }
}
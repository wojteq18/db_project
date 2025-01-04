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
    pub fn new(name: &str, code: &str) -> Self {
        Country {
            country_id: 0, // Domyślnie 0, do nadpisania później
            name: name.to_owned(),
            code: code.to_owned(),
        }
    }

    pub fn add_country(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        let exists: Option<String> = conn.exec_first(
            "SELECT name FROM country WHERE name = :name OR code = :code",
            params! {
                "name" => &self.name,
                "code" => &self.code,
            },
        )?;
    
        if exists.is_none() {
            conn.exec_drop(
                r"INSERT INTO country (name, code)
                VALUES (:name, :code)",
                params! {
                    "name" => &self.name,
                    "code" => &self.code,
                }
            )?;
        } else {
            println!("Country already exists!");
        }
        Ok(())
    }  

    pub fn remove_country(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        conn.exec_drop(
            r"DELETE FROM country WHERE name = :name AND code = :code",
            params! {
                "name" => &self.name,
                "code" => &self.code,
            }
        )?;
        Ok(())
    }
}

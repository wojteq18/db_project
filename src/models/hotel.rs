use mysql::params;
use mysql::PooledConn;
use mysql::prelude::Queryable;

#[derive(Debug, PartialEq, Eq)]
pub struct Hotel {
    pub hotel_id: i32,
    pub name: String,
    pub city_id: i32,
    pub rating: i32,
}

impl Hotel {
    pub fn hotel_exists(conn: &mut PooledConn, name: &str, city_name: &str) -> bool {
        let exists: Option<String> = conn.exec_first(
            "SELECT h.name
            FROM hotel h
            JOIN city c ON h.city_id = c.city_id
            WHERE h.name = :name AND c.name = :city_name",
            params! {
                "name" => name,
                "city_name" => city_name,
            },
        ).unwrap_or(None);
        exists.is_some()
    }

    pub fn get_city_id(conn: &mut PooledConn, city_name: &str) -> Option<i32> {
        conn.exec_first(
            "SELECT city_id FROM city WHERE name = :city_name",
            params! {
                "city_name" => city_name,
            },
        ).unwrap_or(None)
    }

    pub fn new(conn: &mut PooledConn, name: &str, city_name: &str, rating: i32) -> Option<Self> {
        let city_id = Self::get_city_id(conn, city_name)?;
        Some(Hotel {
            hotel_id: 0,
            name: name.to_owned(),
            city_id,
            rating,
        })
    }

    pub fn add_hotel(conn: &mut PooledConn, hotel_name: &str, city_name: &str, rating: i32) -> Result<(), mysql::Error> {
        let hotel = match Self::new(conn, hotel_name, city_name, rating) {
            Some(h) => h,
            None => {
                println!("Error: City '{}' not found.", city_name);
                return Ok(());
            }
        };

        if !Self::hotel_exists(conn, hotel_name, city_name) {
            conn.exec_drop(
                r"INSERT INTO hotel (name, city_id, rating)
                VALUES (:name, :city_id, :rating)",
                params! {
                    "name" => hotel.name,
                    "city_id" => hotel.city_id,
                    "rating" => hotel.rating,
                }
            )?;
            println!("Hotel '{}' added successfully in '{}'.", hotel_name, city_name);
        } else {
            println!("Hotel '{}' already exists in '{}'.", hotel_name, city_name);
        }
        Ok(())
    }

    pub fn remove_hotel(conn: &mut PooledConn, hotel_name: &str, city_name: &str) -> Result<(), mysql::Error> {
        let city_id = match Self::get_city_id(conn, city_name) {
            Some(id) => id,
            None => {
                println!("Error: City '{}' not found.", city_name);
                return Ok(());
            }
        };

        if Self::hotel_exists(conn, hotel_name, city_name) {
            conn.exec_drop(
                r"DELETE FROM hotel WHERE name = :name AND city_id = :city_id",
                params! {
                    "name" => hotel_name,
                    "city_id" => city_id,
                }
            )?;
            println!("Hotel '{}' removed successfully from '{}'.", hotel_name, city_name);
        } else {
            println!("Hotel '{}' does not exist in '{}'.", hotel_name, city_name);
        }
        Ok(())
    }

    pub fn select_hotel(conn: &mut PooledConn, city_name: &str) -> Result<(), mysql::Error> {
        // Pobieramy tylko name i rating z bazy danych
        let hotels: Vec<(String, f64)> = conn.exec(
            "SELECT h.name, h.rating
             FROM hotel h
             JOIN city c ON h.city_id = c.city_id
             WHERE c.name = :city_name",
            params! {
                "city_name" => city_name,
            },
        )?;
    
        // Jeśli nie znaleziono hoteli, informujemy o tym użytkownika
        if hotels.is_empty() {
            println!("No hotels found in '{}'.", city_name);
        } else {
            for (name, rating) in hotels {
                println!("{} - rating: {}", name, rating);
            }
        }
        Ok(())
    }
}

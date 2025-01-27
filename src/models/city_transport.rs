use mysql::params;
use mysql::PooledConn;
use mysql::prelude::Queryable;
use chrono::NaiveDateTime;

#[derive(Debug, PartialEq)]
pub struct City_transport {
    pub city_transport_id: i32,
    pub city_departure_id: i32,
    pub city_arrival_id: i32,
    pub price: f64,
    pub departure_time: NaiveDateTime,
    pub arrival_time: NaiveDateTime,
    transport_id: i32,
}

#[derive(Debug, PartialEq)]
pub struct CityTransportResult {
    pub city_departure_name: String,
    pub city_arrival_name: String,
    pub price: f64,
    pub departure_time: NaiveDateTime,
    pub arrival_time: NaiveDateTime,
    pub transport_name: String,
}

impl City_transport {
    //sprawdzenie czy dokladnie ten srodek transportu nie jest juz w bazie
    pub fn city_transport_exists(
        conn: &mut PooledConn,
        city_departure_name: &str,
        city_arrival_name: &str,
    ) -> bool {
        let exists: Option<i32> = conn.exec_first(
            r"SELECT ct.city_transport_id
            FROM city_transport ct
            JOIN city c1 ON ct.city_departure_id = c1.city_id
            JOIN city c2 ON ct.city_arrival_id = c2.city_id
            JOIN transport t ON ct.transport_id = t.transport_id
            WHERE c1.name = :city_departure_name AND c2.name = :city_arrival_name",    
            params! {
                "city_departure_name" => city_departure_name,
                "city_arrival_name" => city_arrival_name,
            },
        ).unwrap_or(None);
    
        exists.is_some()
    }    

    //znalezienie city_id na podstawie nazwy miasta
    pub fn get_city_id(conn: &mut PooledConn, city_name: &str) -> Option<i32> {
        conn.exec_first(
            "SELECT city_id FROM city WHERE name = :city_name",
            params! {
                "city_name" => city_name,
            },
        ).unwrap_or(None)
    }

    //znalezienie transport_id na podstawie nazwy transportu
    pub fn get_transport_id(conn: &mut PooledConn, transport_name: &str) -> Option<i32> {
        conn.exec_first(
            "SELECT transport_id FROM transport WHERE name = :transport_name",
            params! {
                "transport_name" => transport_name,
            },
        ).unwrap_or(None)
    }

    pub fn add_city_transport(conn: &mut PooledConn, city_departure_name: &str, city_arrival_name: &str, price: f64, departure_time: NaiveDateTime, arrival_time: NaiveDateTime, transport_name: &str) -> Result<(), mysql::Error> {
        if !Self::city_transport_exists(conn, city_departure_name, city_arrival_name) {
            if let Some(city_departure_id) = Self::get_city_id(conn, city_departure_name) {
                if let Some(city_arrival_id) = Self::get_city_id(conn, city_arrival_name) {
                    if let Some(transport_id) = Self::get_transport_id(conn, transport_name) {
                        conn.exec_drop(
                            r"INSERT INTO city_transport (city_departure_id, city_arrival_id, price, departure_time, arrival_time, transport_id)
                            VALUES (:city_departure_id, :city_arrival_id, :price, :departure_time, :arrival_time, :transport_id)",
                            params! {
                                "city_departure_id" => city_departure_id,
                                "city_arrival_id" => city_arrival_id,
                                "price" => price,
                                "departure_time" => departure_time.format("%Y-%m-%d %H:%M:%S").to_string(), //taki format jest rozumiany przez sql
                                "arrival_time" => arrival_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                                "transport_id" => transport_id,
                            }
                        )?;
                        println!("City transport from '{}' to '{}' added successfully by '{}'.", city_departure_name, city_arrival_name, transport_name);
                    }
                }
            }
        } else {
            println!("City transport from '{}' to '{}' by '{}' already exists!", city_departure_name, city_arrival_name, transport_name);
        }
        Ok(())
    }

    pub fn select_city_transport(
        conn: &mut PooledConn,
        city_departure_name: &str,
        city_arrival_name: &str,
    ) -> Result<(), mysql::Error> {
        let result: Vec<CityTransportResult> = conn.exec_map(
            r"SELECT c1.name, c2.name, ct.price, ct.departure_time, ct.arrival_time, t.name
            FROM city_transport ct
            JOIN city c1 ON ct.city_departure_id = c1.city_id
            JOIN city c2 ON ct.city_arrival_id = c2.city_id
            JOIN transport t ON ct.transport_id = t.transport_id
            WHERE c1.name = :city_departure_name AND c2.name = :city_arrival_name",
            params! {
                "city_departure_name" => city_departure_name,
                "city_arrival_name" => city_arrival_name,
            },
            |(city_departure_name, city_arrival_name, price, departure_time, arrival_time, transport_name): (String, String, f64, mysql::Value, mysql::Value, String)| {
                let departure_time = match departure_time {
                    mysql::Value::Date(year, month, day, hour, minute, second, _) => {
                        let naive_date = chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)//tworzy date z wartościami ro, miesiąc, dzień
                            .expect("Invalid departure_time date");
                        naive_date.and_hms_opt(hour as u32, minute as u32, second as u32)//wzbogaca date o godzine, minute i sekunde tworzac NaiveDateTime
                            .expect("Invalid departure_time time")
                    },
                    _ => panic!("Unexpected type for departure_time"),
                };
    
                let arrival_time = match arrival_time {
                    mysql::Value::Date(year, month, day, hour, minute, second, _) => {
                        let naive_date = chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
                            .expect("Invalid arrival_time date");
                        naive_date.and_hms_opt(hour as u32, minute as u32, second as u32)
                            .expect("Invalid arrival_time time")
                    },
                    _ => panic!("Unexpected type for arrival_time"),
                };
    
                CityTransportResult {
                    city_departure_name,
                    city_arrival_name,
                    price,
                    departure_time,
                    arrival_time,
                    transport_name,
                }
            },
        )?;
    
        for transport in result {
            println!(
                "City transport from '{}' to '{}' by '{}' costs {} PLN. Departure time: {}. Arrival time: {}.",
                transport.city_departure_name,
                transport.city_arrival_name,
                transport.transport_name,
                transport.price,
                transport.departure_time,
                transport.arrival_time
            );
        }
        Ok(())
    }    
}    
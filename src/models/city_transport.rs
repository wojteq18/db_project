use mysql::params;
use mysql::PooledConn;
use mysql::prelude::Queryable;
use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Eq)]
pub struct City_transport {
    pub city_transport_id: i32,
    pub city_departure_id: i32,
    pub city_arrival_id: i32,
    pub price: f64,
    pub departure_time: NaiveDateTime,
    pub arrival_time: NaiveDateTime,
    transport_id: i32,
}

impl City_transport {
    //sprawdzenie czy dokladnie ten srodek transportu nie jest juz w bazie
    pub fn city_transport_exists(
        conn: &mut PooledConn,
        city_departure_name: &str,
        city_arrival_name: &str,
        transport_name: &str
    ) -> bool {
        let exists: Option<String> = conn.exec_first(
            r"SELECT ct.city_transport_id
            FROM city_transport ct
            JOIN city c1 ON ct.city_departure_id = c1.city_id
            JOIN city c2 ON ct.city_arrival_id = c2.city_id
            JOIN transport t ON ct.transport_id = t.transport_id
            WHERE c1.name = :city_departure_name AND c2.name = :city_arrival_name AND t.name = :transport_name",
            params! {
                "city_departure_name" => city_departure_name,
                "city_arrival_name" => city_arrival_name,
                "transport_name" => transport_name,
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
}


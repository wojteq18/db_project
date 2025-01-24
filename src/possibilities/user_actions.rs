use mysql::*;
use mysql::prelude::*;
use crate::models::{city_transport::City_transport, city::City, country::Country, hotel::Hotel};

pub trait UserActions {
    fn show_connections(conn: &mut PooledConn);
    fn show_hotels(conn: &mut PooledConn);
}

pub struct NormalUser;

impl UserActions for NormalUser {
    fn show_connections(conn: &mut PooledConn) {
        println!("Type city departure: ");
        let mut city_departure = String::new();
        std::io::stdin().read_line(&mut city_departure).expect("Failed to read line");
        let city_departure: String = city_departure.trim().to_string();

        println!("Type city arrival: ");
        let mut city_arrival = String::new();
        std::io::stdin().read_line(&mut city_arrival).expect("Failed to read line");
        let city_arrival: String = city_arrival.trim().to_string();

        if City_transport::city_transport_exists(conn, &city_departure, &city_arrival) {
            City_transport::select_city_transport(conn, &city_departure, &city_arrival).expect("Failed to fetch connections");
        } else {
            println!("Connection between '{}' and '{}' does not exist.", city_departure, city_arrival);
        }
    }

    fn show_hotels(conn: &mut PooledConn) {
        println!("Type country name: ");
        let mut country_name = String::new();
        std::io::stdin().read_line(&mut country_name).expect("Failed to read line");
        let country_name: String = country_name.trim().to_string();

        println!("Type city name: ");
        let mut city_name = String::new();
        std::io::stdin().read_line(&mut city_name).expect("Failed to read line");
        let city_name: String = city_name.trim().to_string();

        if !Country::country_exists(conn, &country_name) {
            println!("Country does not exist!");
        } else if !City::city_exists(conn, &city_name, &country_name) {
            println!("City does not exist!");
        } else {
            println!("Hotels in {}, {}", city_name, country_name);
            Hotel::select_hotel(conn, &city_name).expect("Failed to fetch hotels");
        }
    }
}


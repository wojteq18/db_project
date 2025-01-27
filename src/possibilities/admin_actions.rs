use mysql::*;
use mysql::prelude::*;
use crate::models::{city::{self, City}, city_transport::City_transport, country::Country, hotel::Hotel};

pub trait AdminActions {
    fn add_city(conn: &mut PooledConn);
    fn add_country(conn: &mut PooledConn);
    fn add_hotel(conn: &mut PooledConn);
    fn add_city_transport(conn: &mut PooledConn);
}

pub struct Admin;

impl AdminActions for Admin {
    fn add_city(conn: &mut PooledConn) {
        println!("Type country name: ");
        let mut country_name = String::new();
        std::io::stdin().read_line(&mut country_name).expect("Failed to read line");
        let country_name = country_name.trim().to_string();

        println!("Type city name: ");
        let mut city_name = String::new();
        std::io::stdin().read_line(&mut city_name).expect("Failed to read line");
        let city_name = city_name.trim().to_string();

        if City::city_exists(conn, &country_name, &city_name) {
            println!("City '{}' already exists!", city_name);
        } else {
            let city = City::new(&city_name, &country_name);
            city.add_city(conn).expect("Failed to add city");
        }
    }
    fn add_country(conn: &mut PooledConn) {
        println!("Type country name: ");
        let mut country_name = String::new();
        std::io::stdin().read_line(&mut country_name).expect("Failed to read line");
        let country_name = country_name.trim().to_string();

        println!("Type country code: ");
        let mut country_code = String::new();
        std::io::stdin().read_line(&mut country_code).expect("Failed to read line");
        let country_code = country_code.trim().to_string();

        if Country::country_exists(conn, &country_name) {
            println!("Country '{}' already exists!", country_name);
        } else {
            let country = Country::new(&country_name, &country_code);
            country.add_country(conn).expect("Failed to add country");
        }
    }
    fn add_hotel(conn: &mut PooledConn) {
        println!("Type hotel name: ");
        let mut hotel_name = String::new();
        std::io::stdin().read_line(&mut hotel_name).expect("Failed to read line");
        let hotel_name = hotel_name.trim().to_string();

        println!("Type city name: ");
        let mut city_name = String::new();
        std::io::stdin().read_line(&mut city_name).expect("Failed to read line");
        let city_name = city_name.trim().to_string();

        println!("Type hotel rating: ");
        let mut hotel_rating = String::new();
        std::io::stdin().read_line(&mut hotel_rating).expect("Failed to read line");
        let hotel_rating: i32 = hotel_rating.trim().parse().expect("Please type a number!");

        if Hotel::hotel_exists(conn, &hotel_name, &city_name) {
            println!("Hotel '{}' already exists!", hotel_name);
        } else {
            if let Some(hotel) = Hotel::new(conn, &hotel_name, &city_name, hotel_rating) {
                Hotel::add_hotel(conn, &hotel_name, &city_name, hotel_rating).expect("Failed to add hotel");
            } else {
                println!("Error: City '{}' not found.", city_name);
            }   
        }
    }
    fn add_city_transport(conn: &mut PooledConn) {
        //TODO
    }
}
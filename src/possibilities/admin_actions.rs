use mysql::*;
use mysql::prelude::*;
use crate::models::{city::{self, City}, city_transport::City_transport, country::{self, Country}, hotel::Hotel};
use chrono::NaiveDateTime;


pub trait AdminActions {
    fn add_city(conn: &mut PooledConn);
    fn add_country(conn: &mut PooledConn);
    fn add_hotel(conn: &mut PooledConn);
    fn add_city_transport(conn: &mut PooledConn);
    fn remove_city(conn: &mut PooledConn);
    fn remove_country(conn: &mut PooledConn);
    fn remove_hotel(conn: &mut PooledConn);
    //TODO add remove fuction
}

pub struct Admin;

impl AdminActions for Admin {
    fn add_city(conn: &mut PooledConn) {
        
        let city_name = get_city();
        let country_name = get_country();

        if City::city_exists(conn, &country_name, &city_name) {
            println!("City '{}' already exists!", city_name);
        } else {
            let city = City::new(&city_name, &country_name);
            city.add_city(conn).expect("Failed to add city");
        }
    }
    fn add_country(conn: &mut PooledConn) {
        let country_name = get_country();
        let country_code = get_country_code();
    
        if Country::country_exists(conn, &country_name) {
            println!("Country '{}' already exists!", country_name);
        } else {
            let country = Country::new(&country_name, &country_code);
            country.add_country(conn).expect("Failed to add country");
        }
    }
    fn add_hotel(conn: &mut PooledConn) {
        let hotel_name = get_hotel();
        let city_name = get_city();
        let hotel_rating = get_hotel_rating();

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
        let city_departure_name = get_city();
        let city_arrival_name = get_city();
        let transport_name = get_transport();
        let price = get_price();
        let departure_time = get_date();
        let arrival_time = get_date();
    
        // Sprawdzenie, czy transport już istnieje
        if City_transport::city_transport_exists(conn, &city_departure_name, &city_arrival_name) {
            println!(
                "City transport from '{}' to '{}' by '{}' already exists!",
                city_departure_name, city_arrival_name, transport_name
            );
            } else {
            // Dodanie nowego transportu
            City_transport::add_city_transport(
                conn,
                &city_departure_name,
                &city_arrival_name,
                price,
                departure_time,
                arrival_time,
                &transport_name,
            ).expect("Failed to add city transport"); 
        }
    }
    

    fn remove_city(conn: &mut PooledConn) {
        
        let city_name = get_city();
        let country_name = get_country();

        if City::city_exists(conn, &city_name, &country_name) {
            let city = City::new(&city_name, &country_name);
            city.remove_city(conn).expect("Failed to remove city");
        }
    }

    fn remove_country(conn: &mut PooledConn) {
        let country_name = get_country();
        let country_code = get_country_code();

        if Country::country_exists(conn, &country_name) {
            let country = Country::new(&country_name, &country_code);
            country.remove_country(conn).expect("Failed to remove country");
        } 
    }

    fn remove_hotel(conn: &mut PooledConn) {
        let hotel_name = get_hotel();
        let city_name = get_city();

        if Hotel::hotel_exists(conn, &hotel_name, &city_name) {
            Hotel::remove_hotel(conn, &hotel_name, &city_name).expect("Failed to remove hotel");
        } else {
            println!("Hotel '{}' does not exist!", hotel_name);
        }
    }
}

fn get_city() -> String { //funkcja pobierająca nazwe miasta

    println!("Type city name: ");
    let mut city_name = String::new();
    std::io::stdin().read_line(&mut city_name).expect("Failed to read line");
    let city_name = city_name.trim().to_string();

    return city_name
}

fn get_country() -> String { //funkcja pobierająca nazwe kraju

    println!("Type country name: ");
    let mut country_name = String::new();
    std::io::stdin().read_line(&mut country_name).expect("Failed to read line");
    let country_name = country_name.trim().to_string();

    return country_name
}

fn get_country_code() -> String { //funkcja pobierająca kod kraju

    println!("Type country code: ");
    let mut country_code = String::new();
    std::io::stdin().read_line(&mut country_code).expect("Failed to read line");
    let country_code = country_code.trim().to_string();

    return country_code
}

fn get_hotel() -> String { //funkcja pobierająca nazwe hotelu

    println!("Type hotel name: ");
    let mut hotel_name = String::new();
    std::io::stdin().read_line(&mut hotel_name).expect("Failed to read line");
    let hotel_name = hotel_name.trim().to_string();

    return hotel_name

}

fn get_hotel_rating() -> i32 { //funkcja pobierająca ocene hotelu

    println!("Type hotel rating: ");
    let mut hotel_rating = String::new();
    std::io::stdin().read_line(&mut hotel_rating).expect("Failed to read line");
    let hotel_rating: i32 = hotel_rating.trim().parse().expect("Please type a number!");

    return hotel_rating

}

fn get_transport() -> String { //funkcja pobierająca nazwe transportu

    println!("Type transport name: ");
    let mut transport_name = String::new();
    std::io::stdin().read_line(&mut transport_name).expect("Failed to read line");
    let transport_name = transport_name.trim().to_string();

    return transport_name

}

fn get_price() -> f64 { //funkcja pobierająca cene transportu

    println!("Type transport price: ");
    let mut price = String::new();
    std::io::stdin().read_line(&mut price).expect("Failed to read line");
    let price: f64 = price.trim().parse().expect("Please type a valid number!");

    return price

}

fn get_date() -> NaiveDateTime { //funkcja pobierająca date

    println!("Type date (YYYY-MM-DD HH:MM:SS): ");
    let mut date = String::new();
    std::io::stdin().read_line(&mut date).expect("Failed to read line");
    let date = NaiveDateTime::parse_from_str(date.trim(), "%Y-%m-%d %H:%M:%S")
        .expect("Invalid date format!");

    return date

}

use std::result;
use std::option;

use mysql::*;
use mysql::prelude::*;
use bcrypt::{hash, verify};
use crate::models::user::User;

pub trait Auth {
    fn register(conn: &mut PooledConn);
    fn log_in(conn: &mut PooledConn) -> AuthService;
}

pub struct AuthService {
    pub is_logged: bool,
    pub is_admin: bool,
}

impl AuthService {
    /// Tworzy nową instancję AuthService z domyślnymi wartościami
    pub fn new() -> Self {
        AuthService {
            is_logged: false,
            is_admin: false,
        }
    }
}

impl Auth for AuthService {
    fn register(conn: &mut PooledConn) {
        println!("Type your login: ");
        let mut login = String::new();
        std::io::stdin().read_line(&mut login).expect("Failed to read line");
        let login = login.trim().to_string();

        println!("Type your password: ");
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).expect("Failed to read line");
        let password = password.trim().to_string();

        if User::user_exists(conn, &login) {
            println!("User '{}' already exists!", login);
        } else {
            let hashed_password = hash(&password, 6).expect("Failed to hash password");
            let status = "user";
            conn.exec_drop(
                r"INSERT INTO user (login, password, status)
                VALUES (:login, :password, :status)",
                params! {
                    "login" => &login,
                    "password" => hashed_password,
                    "status" => status,
                },
            )
            .expect("Failed to insert user");
            println!("User '{}' registered successfully!", login);
        }
    }

    fn log_in(conn: &mut PooledConn) -> AuthService{
        println!("Type your login: ");
        let mut login = String::new();
        std::io::stdin().read_line(&mut login).expect("Failed to read line");
        let login = login.trim().to_string();

        println!("Type your password: ");
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).expect("Failed to read line");
        let password = password.trim().to_string();

        let mut auth_service = AuthService::new();

        if User::user_exists(conn, &login) == true {
            let query = "SELECT password, status FROM user WHERE login = :login";
            let result: Option<(String, String)> = conn.exec_first(
                query,
                params! {
                    "login" => login,
                },
            ).expect("Failed to fetch user data");

            match result {
                Some((hashed_password, status)) => {
                    if verify(&password, &hashed_password).unwrap_or(false) {
                        println!("Login successful!");
                        auth_service.is_logged = true;

                        if status == "admin" {
                            println!("Welcome, admin!");
                            auth_service.is_admin = true;
                        } else {
                            println!("Welcome, regular user!");
                        }
                    } else {
                        println!("Invalid password. Please try again.");
                    }
                }
                None => {
                    println!("No user found with the provided login.");
                }
            }
        } else {
            println!("User '{}' does not exist.", login);
        }
        return auth_service
    }
}

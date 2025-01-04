use mysql::params;
use mysql::PooledConn;
use mysql::prelude::Queryable;

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub user_id: i32,
    pub login: String,
    pub password: String,
    pub status: String,
}

impl User {

    // Sprawdzenie, czy użytkownik istnieje w bazie
    pub fn user_exists(conn: &mut PooledConn, login: &str) -> bool {
        let exists: Option<String> = conn.exec_first(
            "SELECT login FROM user WHERE login = :login",
            params! {
                "login" => login,
            },
        ).unwrap_or(None); 
    
        exists.is_some() // Zwróć true, jeśli użytkownik istnieje
    }

    // Konstruktor nowego użytkownika
    pub fn new(login: &str, password: &str, status: &str) -> Self {
        User {
            user_id: 0,
            login: login.to_owned(),
            password: password.to_owned(),
            status: status.to_owned(),
        }
    }

    // Dodanie użytkownika, tylko jeśli nie istnieje
    pub fn add_user(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        if Self::user_exists(conn, &self.login) == false {
            conn.exec_drop(
                r"INSERT INTO user (login, password, status)
                VALUES (:login, :password, :status)",
                params! {
                    "login" => &self.login,
                    "password" => &self.password,
                    "status" => &self.status,
                }
            )?;
            println!("User '{}' added successfully!", &self.login);
        } else {
            println!("User '{}' already exists!", &self.login);
        }
        Ok(())
    }

    // Usuwanie użytkownika
    pub fn remove_user(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        if Self::user_exists(conn, &self.login) {
            conn.exec_drop(
                r"DELETE FROM user WHERE login = :login",
                params! {
                    "login" => &self.login,
                }
            )?;
            println!("User '{}' removed.", &self.login);
        } else {
            println!("User '{}' does not exist.", &self.login);
        }
        Ok(())
    }

    // Promowanie użytkownika do admina
    pub fn promote_user(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        if Self::user_exists(conn, &self.login) {
            conn.exec_drop(
                r"UPDATE user SET status = 'admin' WHERE login = :login",
                params! {
                    "login" => &self.login,
                }
            )?;
            println!("User '{}' promoted to admin.", &self.login);
        } else {
            println!("User '{}' does not exist.", &self.login);
        }
        Ok(())
    }
}

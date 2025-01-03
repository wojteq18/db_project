use mysql::params;
use mysql::PooledConn;
use mysql::prelude::Queryable;

#[derive(Debug, PartialEq, Eq)]

pub struct User {
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(first_name: &str, last_name: &str, email: &str, password: &str) -> Self {
        User {
            user_id: 0, //domyslnie 0, do nadpisania pozniej
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            email: email.to_owned(),
            password: password.to_owned(),
        }
    }

    pub fn add_user(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        conn.exec_drop(
            r"INSERT INTO user (first_name, last_name, email, password)
            VALUES (:first_name, :last_name, :email, :password)",
            params! {
                "first_name" => &self.first_name,
                "last_name" => &self.last_name,
                "email" => &self.email,
                "password" => &self.password,
            }
        )?;
        Ok(())
    }
}
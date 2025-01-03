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
    pub fn new(login: &str, status: &str, password: &str) -> Self {
        User {
            user_id: 0, //domyslnie 0, do nadpisania pozniej
            login: login.to_owned(),
            status: status.to_owned(),
            password: password.to_owned(),
        }
    }

    pub fn add_user(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        conn.exec_drop(
            r"INSERT INTO user (status, login, password)
            VALUES (:status, :login, :password)",
            params! {
                "login" => &self.login,
                "status" => &self.status,
                "password" => &self.password,
            }
        )?;
        Ok(())
    }

    pub fn remove_user(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        conn.exec_drop(
            r"DELETE FROM user WHERE status = :status AND login = :login AND password = :password",
            params! {
                "status" => &self.status,
                "login" => &self.login,
                "password" => &self.password,
            }
        )?;
        Ok(())
    }
}
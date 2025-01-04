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
    pub fn new(login: &str, password: &str, status: &str) -> Self {
        User {
            user_id: 0,  // Autoinkrementacja, zostanie nadpisane po dodaniu do bazy
            login: login.to_owned(),
            password: password.to_owned(),
            status: status.to_owned(),
        }
    }

    pub fn add_user(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        conn.exec_drop(
            r"INSERT INTO user (login, password, status)
            VALUES (:login, :password, :status)",
            params! {
                "login" => &self.login,
                "password" => &self.password,
                "status" => &self.status,
            }
        )?;
        Ok(())
    }

    pub fn remove_user(&self, conn: &mut PooledConn) -> Result<(), mysql::Error> {
        conn.exec_drop(
            r"DELETE FROM user WHERE login = :login AND password = :password AND status = :status",
            params! {
                "login" => &self.login,
                "password" => &self.password,
                "status" => &self.status,
            }
        )?;
        Ok(())
    }
}

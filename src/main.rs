use mysql::*;
use mysql::prelude::*;
use dotenvy::dotenv;
use std::env;

fn connect_to_db() -> Result<PooledConn> {
    // Wczytanie zmiennych środowiskowych z pliku .env
    dotenv().ok();

    // Pobranie URL bazy danych z .env
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not found in .env file");

    // Konwersja String na Opts (obsługa błędu URL)
    let opts = Opts::from_url(&database_url)
        .expect("Invalid DATABASE_URL format");

    // Tworzenie puli połączeń
    let pool = Pool::new(opts)?;

    // Pobranie pojedynczego połączenia
    let conn = pool.get_conn()?;
    Ok(conn)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Połączenie z bazą danych
    let mut conn = connect_to_db()?;

    // Tworzenie testowej tabeli (jeśli nie istnieje)
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS test_table (
            id INT PRIMARY KEY AUTO_INCREMENT,
            name VARCHAR(50) NOT NULL
        )"
    )?;

    println!("Tabela test_table została utworzona!");

    // Wstawienie testowego rekordu
    conn.exec_drop(
        "INSERT INTO test_table (name) VALUES (:name)",
        params! {
            "name" => "Rust User"
        }
    )?;

    println!("Wstawiono dane!");

    // Wykonanie zapytania SELECT
    let result: Vec<(i32, String)> = conn.query(
        "SELECT id, name FROM test_table"
    )?;

    // Wyświetlenie wyników
    for (id, name) in result {
        println!("ID: {}, Name: {}", id, name);
    }

    Ok(())
}

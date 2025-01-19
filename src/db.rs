use mysql::*;
use dotenvy::dotenv;
use std::env;

// Funkcja nawiązująca połączenie
pub fn connect_to_db() -> Result<Pool> { //Pool -> pula zasobów polączeń do bazy danych
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not found in .env file");

    // Tworzenie opcji połączenia
    let opts = Opts::from_url(&database_url)
        .expect("Invalid DATABASE_URL format");

    // Tworzenie puli połączeń
    let pool = Pool::new(opts)?;
    Ok(pool)
}

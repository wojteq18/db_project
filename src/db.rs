use mysql::*;
use dotenvy::dotenv;
use std::env;

pub struct DatabaseConfig {
    pub readonly_pool: Pool,
    pub admin_pool: Pool,
}

pub fn connect_to_db() -> Result<DatabaseConfig> {
    dotenv().ok(); //ładuje zmienne z pliku env
    let readonly_url = env::var("DATABASE_READONLY_URL").expect("DATABASE_READONLY_URL not found");
    let readonly_opts = Opts::from_url(&readonly_url)?; //parsuje url na strukture Opts - wymagane przez biblioteke mysql
    let readonly_pool = Pool::new(readonly_opts)?; //tworzy pule połączeń

    let admin_url = env::var("DATABASE_ADMIN_URL").expect("DATABASE_ADMIN_URL not found");
    let admin_opts = Opts::from_url(&admin_url)?;
    let admin_pool = Pool::new(admin_opts)?;

    Ok(DatabaseConfig { //w przypadku sukcesu zwraca pule polaczen
        readonly_pool,
        admin_pool,
    })
}


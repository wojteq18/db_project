use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let hashed_password = hash("haslo123", DEFAULT_COST).unwrap();
    println!("Hashed password: {}", hashed_password);
}

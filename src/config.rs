use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

pub fn load_env() {
    dotenv().ok();
}

lazy_static! {
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap_or_default();
}

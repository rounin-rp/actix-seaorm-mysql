pub mod config;

fn main() {
    config::load_env();
    let db_url: String = config::DATABASE_URL.to_string();
    actix_api::main(db_url);
}

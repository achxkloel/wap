fn main() {
    let database_url_name = "DATABASE_URL";
    let database_url = std::env::var(database_url_name)
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/wap_db".to_string());
    println!("cargo:rustc-env={}={}", database_url_name, database_url);

    println!("cargo:rerun-if-changed=migrations");
}

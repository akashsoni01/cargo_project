fn main() {
    println!("Hello, world!");
}

/*
sqlx database create
sqlx database drop

sqlx migrate add <name>

sqlx migrate run

*/ 

/*
sqlx database create
sqlx database drop

sqlx migrate add -r your_table_or_migration_name

sqlx migrate run
sqlx migrate revert

*/ 

/*
very important command to ofline your query and to check the syntax of your query
cargo sqlx prepare --workspace -- --all-targets --all-features

*/
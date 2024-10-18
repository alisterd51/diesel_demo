use diesel_demo::establish_connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

fn main() {
    let connection = &mut establish_connection();
    let _ = connection.run_pending_migrations(MIGRATIONS);
}

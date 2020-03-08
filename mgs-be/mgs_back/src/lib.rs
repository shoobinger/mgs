pub mod db;
use r2d2_sqlite::SqliteConnectionManager;
use lazy_static::lazy_static;
use rusqlite::{Connection, Error};
use r2d2::{PooledConnection, CustomizeConnection};
use std::io;
use std::io::Write;

pub type Pool = r2d2::Pool<SqliteConnectionManager>;

lazy_static! {
    pub static ref DB_POOL: Pool = setup_db();
}

fn tracer(s: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(s.as_ref());
    handle.write("\n".as_ref());
    handle.flush();
}

#[derive(Debug)]
struct Customizer;

impl CustomizeConnection<Connection, Error> for Customizer {
    fn on_acquire(&self, conn: &mut Connection) -> Result<(), Error> {
        conn.trace(Some(tracer));
        Ok(())
    }
}

fn setup_db() -> Pool {
    let manager = SqliteConnectionManager::file("mgs.sqlite");
    Pool::builder()
        .connection_customizer(Box::new(Customizer{}))
        .build(manager).unwrap()
}

fn conn() -> PooledConnection<SqliteConnectionManager> {
    DB_POOL.get().unwrap()
}

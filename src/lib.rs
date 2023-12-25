use rusqlite::Connection;
use thiserror::Error;

// NOTE: See if creating new connection for every function is worse than keeping a persistent
// connection.
struct Database {
    path: &'static str,
}
// API USAGE:
// Creating a SQLite Database
// --------------------------
// let db = Database::create("Caissabase 2022", None, "/assets/dbs/Caissabase-2022");
//
// Searching over database
// -----------------------
// let positions : Vec<Position> = db.exact_pos_search(<some board representation>);
// let fuzzy : Vec<Position> = db.fuzzy_pos_search(<some board representation>);
// let query : Vec<Position> = db.query_search(<some board representation>, <query params>);
//
// Delete
// ------
// let deleted_entry = db.delete_game(<id of entry to delete>);
// let deleted_entries = db.delete_games([<id of entry to delete>]);
//
// Add
// ---
// let new_entry = db.add_game(<game type>);
// let new_entries = db.add_games([<game type>]);

impl Database {
    // since db's stored in state array,can just call open since only store ones that exist
    // (scanned from db directory)
    /// Open an existing database
    fn open(&self) -> Result<Connection, DatabaseError> {
        Connection::open(self.path).map_err(DatabaseError::Rusqlite)
    }

    /// Create a new database
    pub fn create(&self) -> Result<Connection, DatabaseError> {
        let conn = Connection::open(self.path).map_err(DatabaseError::Rusqlite)?;
        conn.execute(
            "CREATE TABLE games (
                id INTEGER PRIMARY KEY,
                event TEXT,
                site TEXT,
                date TEXT,
                round TEXT,
                white TEXT,
                black TEXT,
                result TEXT,
                white_elo INTEGER,
                black_elo INTEGER,
                eco TEXT,
                moves TEXT
            )",
            [],
        )
        .map_err(DatabaseError::Rusqlite)?;
        Ok(conn)
    }
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database with same name already exists")]
    AlreadyExists,
    #[error("SQLite error")]
    Rusqlite(#[from] rusqlite::Error),
}

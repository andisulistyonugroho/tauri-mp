use rusqlite::{Connection, Result};
use std::path::Path;

pub fn connect_db() -> Result<Connection> {
    let db_path = Path::new("/Users/asn/Sites/MP/database/mp_database.db");
    let conn = Connection::open(db_path)?;

    // create table if not exists
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS posts_metadata (
        id INTEGER PRIMARY KEY,
        slug TEXT NOT NULL,
        title TEXT NOT NULL,
        category TEXT NOT NULL,
        status TEXT CHECK(status IN('draft','published','archived')) DEFAULT 'draft',
        published_at TEXT,
        created_at TEXT DEFAULT CURRENT_TIMESTAMP,
        updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
        author TEXT
    );

    CREATE TRIGGER IF NOT EXISTS update_posts_metadata_updated_at
    AFTER UPDATE OF slug, title, category, status, author ON posts_metadata
    BEGIN
        UPDATE posts_metadata
        SET updated_at = CURRENT_TIMESTAMP
        WHERE id = OLD.id;
    END;

    CREATE TABLE IF NOT EXISTS posts_content (post_id INTEGER NOT NULL UNIQUE,body TEXT NOT NULL);

    CREATE TABLE IF NOT EXISTS posts_tags (post_id INTEGER NOT NULL, tag TEXT NOT NULL);
    ",
    )?;
    Ok(conn)
}

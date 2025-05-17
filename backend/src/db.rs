use crate::models::Event;
use chrono::NaiveDateTime;
use rusqlite::{params, Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("letmeseeifimfree.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            location TEXT
        )",
        [],
    )?;

    Ok(conn)
}

pub fn get_all_events(conn: &Connection) -> Result<Vec<Event>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, start_time, end_time, location FROM events ORDER BY start_time ASC",
    )?;

    let event_iter = stmt.query_map([], |row| {
        Ok(Event {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            start_time: NaiveDateTime::parse_from_str(&row.get::<_, String>(3)?, "%Y-%m-%d %H:%M:%S")?,
            end_time: NaiveDateTime::parse_from_str(&row.get::<_, String>(4)?, "%Y-%m-%d %H:%M:%S")?,
            location: row.get(5)?,
        })
    })?;

    let mut events = Vec::new();
    for event in event_iter {
        events.push(event?);
    }

    Ok(events)
}

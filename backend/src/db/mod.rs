use rusqlite::{params, Connection, Result};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("letmeseeifimfree.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn create_event(
    conn: &Connection,
    title: &str,
    description: &str,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> Result<()> {
    conn.execute(
        "INSERT INTO events (title, description, start_time, end_time) VALUES (?1, ?2, ?3, ?4)",
        params![title, description, start_time, end_time],
    )?;
    Ok(())
}

pub fn get_all_events(conn: &Connection) -> Result<Vec<Event>> {
    let mut stmt = conn.prepare("SELECT id, title, description, start_time, end_time FROM events")?;
    let rows = stmt.query_map([], |row| {
        Ok(Event {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            start_time: row.get::<_, String>(3)?.parse::<NaiveDateTime>().unwrap(),
            end_time: row.get::<_, String>(4)?.parse::<NaiveDateTime>().unwrap(),
        })
    })?;
    rows.collect()
}

pub fn delete_event(conn: &Connection, event_id: i32) -> Result<()> {
    conn.execute("DELETE FROM events WHERE id = ?1", params![event_id])?;
    Ok(())
}

pub fn update_event(
    conn: &Connection,
    event_id: i32,
    title: &str,
    description: &str,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> Result<()> {
    conn.execute(
        "UPDATE events SET title = ?1, description = ?2, start_time = ?3, end_time = ?4 WHERE id = ?5",
        params![title, description, start_time, end_time, event_id],
    )?;
    Ok(())
}

pub fn get_events_by_date_range(
    conn: &Connection,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<Vec<Event>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, start_time, end_time FROM events
         WHERE start_time >= ?1 AND end_time <= ?2",
    )?;
    let rows = stmt.query_map(params![start, end], |row| {
        Ok(Event {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            start_time: row.get::<_, String>(3)?.parse::<NaiveDateTime>().unwrap(),
            end_time: row.get::<_, String>(4)?.parse::<NaiveDateTime>().unwrap(),
        })
    })?;
    rows.collect()
}

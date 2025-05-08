CREATE TABLE events (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    source TEXT NOT NULL, -- google, outlook, canvas, or custom
    created_at TEXT NOT NULL
);

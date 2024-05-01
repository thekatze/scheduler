CREATE TABLE IF NOT EXISTS calendars (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS events (
    calendar_id BLOB NOT NULL,
    id BLOB PRIMARY KEY NOT NULL,
    date DATETIME NOT NULL,
    summary TEXT NOT NULL,
    FOREIGN KEY(calendar_id) REFERENCES calendars(id)
);


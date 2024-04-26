CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    date DATETIME,
    type INT,
    CHECK (type IN (0, 1))
)

DROP TABLE IF EXISTS 'Transaction';

CREATE TABLE Transaction (
    id INTEGER PRIMARY KEY,
    budget_id INTEGER,
    name TEXT,
    AMOUNT FLOAT,
);
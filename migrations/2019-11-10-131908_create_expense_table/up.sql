CREATE TABLE expense_t (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    cost REAL NOT NULL,
    description TEXT NOT NULL,
    category TEXT DEFAULT "other" NOT NULL,
    tags TEXT DEFAULT "unspecified" NOT NULL,
    ts DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
)

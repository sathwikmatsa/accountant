CREATE TABLE friend_t (
    name CHAR(10) NOT NULL PRIMARY KEY,
    upi_id TEXT NOT NULL UNIQUE,
    phone TEXT NOT NULL UNIQUE
        CHECK (length(phone) >= 10)
)

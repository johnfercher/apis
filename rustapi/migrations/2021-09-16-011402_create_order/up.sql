CREATE TABLE orders (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    label_code VARCHAR(100) NULL,
    origin VARCHAR(100) NOT NULL,
    destiny VARCHAR(100) NOT NULL
);
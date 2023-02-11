-- Your SQL goes here

CREATE TABLE page_type (
    id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE field_type (
    id INT NOT NULL,
    type VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    page_type INT NOT NULL REFERENCES page_type(id),
    PRIMARY KEY (id)
);

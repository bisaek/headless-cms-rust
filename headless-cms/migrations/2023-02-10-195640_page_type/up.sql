-- Your SQL goes here

CREATE TABLE page_type (
    id serial NOT NULL,
    name VARCHAR(255) NOT NULL,
    CONSTRAINT page_type_key PRIMARY KEY (id)
);

CREATE TABLE field_type (
    id serial NOT NULL,
    f_type VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    page_type INT NOT NULL REFERENCES page_type(id),
    CONSTRAINT field_type_key PRIMARY KEY (id)
);

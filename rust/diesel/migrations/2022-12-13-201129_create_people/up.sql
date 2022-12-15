-- Your SQL goes here
CREATE TABLE people (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    AGE int4
);

CREATE TABLE foods (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE people_foods (
    foods_id int4 NOT NULL,
    people_id int4 NOT NULL,

    PRIMARY KEY(foods_id, people_id),

    CONSTRAINT fk_food
        FOREIGN KEY(foods_id)
            REFERENCES foods(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_people
        FOREIGN KEY(people_id)
            REFERENCES people(id)
            ON DELETE CASCADE
)

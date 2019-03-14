ALTER TABLE users
  ADD COLUMN email VARCHAR NULL;

CREATE TABLE roles (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR NULL
);

CREATE TABLE user_roles (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  role_id INTEGER NOT NULL REFERENCES roles(id)
);

INSERT INTO roles (name, description) VALUES ('admin', 'An administrator: manage settings, create topics');
INSERT INTO roles (name, description) VALUES ('viewer', 'A simple viewer: subscribe to topics');
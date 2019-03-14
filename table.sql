CREATE TABLE confirmations (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  email VARCHAR NOT NULL,
  code VARCHAR NOT NULL,
  sent_on TIMESTAMP NOT NULL DEFAULT now(),
  expiring_on TIMESTAMP NOT NULL,
  confirmed_on TIMESTAMP NULL,
  status INTEGER -- 1-waiting, 2-successful, 3-declined
);

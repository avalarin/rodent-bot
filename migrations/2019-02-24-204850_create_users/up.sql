CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  tg_id BIGINT NOT NULL,
  tg_username VARCHAR NULL,
  tg_fullname VARCHAR NULL,
  active BOOLEAN NOT NULL DEFAULT 'f'
);
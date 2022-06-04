DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE testing.users (
	id  BIGSERIAL PRIMARY KEY,
  phone_number  VARCHAR(50)  UNIQUE NOT NULL,
	first_name    VARCHAR(200)        NOT NULL,
	last_name     VARCHAR(200)        NOT NULL,
  birthday      DATE                NOT NULL,
  contacts      VARCHAR(50)[]
);

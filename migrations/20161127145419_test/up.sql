CREATE TABLE sessions (
	id SERIAL PRIMARY KEY,
	sess_token BYTEA NOT NULL UNIQUE,
	proposed_token BYTEA UNIQUE,
	last_ip BYTEA NOT NULL
);
